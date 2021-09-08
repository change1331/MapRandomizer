from typing import Optional, List
import copy
import torch
import torch.nn.functional as F
from maze_builder.model import Model
from maze_builder.env import MazeBuilderEnv
from maze_builder.replay import ReplayBuffer
from maze_builder.types import EpisodeData, TrainingData
from model_average import ExponentialAverage
import concurrent.futures
import logging
from dataclasses import dataclass


# TODO: try using torch.multinomial instead of implementing this from scratch?
def _rand_choice(p):
    cumul_p = torch.cumsum(p, dim=1)
    rnd = torch.rand([p.shape[0], 1], device=p.device)
    choice = torch.clamp(torch.searchsorted(cumul_p, rnd), max=p.shape[1] - 1).view(-1)
    return choice


class TrainingSession():
    def __init__(self, envs: List[MazeBuilderEnv],
                 model: Model,
                 optimizer: torch.optim.Optimizer,
                 ema_beta: float,
                 replay_size: int,
                 decay_amount: float,
                 sam_scale: Optional[float],
                 ):
        self.envs = envs
        self.model = model
        self.optimizer = optimizer
        self.average_parameters = ExponentialAverage(model.all_param_data(), beta=ema_beta)
        self.num_rounds = 0
        self.decay_amount = decay_amount
        self.sam_scale = sam_scale
        self.grad_scaler = torch.cuda.amp.GradScaler()
        self.replay_buffer = ReplayBuffer(replay_size, len(self.envs[0].rooms), storage_device=torch.device('cpu'))

        self.total_step_remaining_gen = 0.0
        self.total_step_remaining_train = 0.0

    def forward_state_action(self, model, room_mask, room_position_x, room_position_y, action_candidates,
                             steps_remaining,
                             env_id, round):
        # print({k: v.shape for k, v in locals().items() if hasattr(v, 'shape')})
        #
        # torch.cuda.synchronize()
        # logging.info("Processing candidate data")
        num_envs = room_mask.shape[0]
        num_candidates = action_candidates.shape[1]
        num_rooms = len(self.envs[0].rooms)
        action_room_id = action_candidates[:, :, 0]
        action_x = action_candidates[:, :, 1]
        action_y = action_candidates[:, :, 2]
        all_room_mask = room_mask.unsqueeze(1).repeat(1, num_candidates + 1, 1)
        all_room_position_x = room_position_x.unsqueeze(1).repeat(1, num_candidates + 1, 1)
        all_room_position_y = room_position_y.unsqueeze(1).repeat(1, num_candidates + 1, 1)
        all_steps_remaining = steps_remaining.unsqueeze(1).repeat(1, num_candidates + 1)
        # all_round = round.unsqueeze(1).repeat(1, num_candidates + 1)

        # print(action_candidates.device, action_room_id.device)
        all_room_mask[torch.arange(num_envs, device=action_candidates.device).view(-1, 1),
                      torch.arange(1, 1 + num_candidates, device=action_candidates.device).view(1, -1),
                      action_room_id] = True
        all_room_mask[:, :, -1] = False
        all_room_position_x[torch.arange(num_envs, device=action_candidates.device).view(-1, 1),
                            torch.arange(1, 1 + num_candidates, device=action_candidates.device).view(1, -1),
                            action_room_id] = action_x
        all_room_position_y[torch.arange(num_envs, device=action_candidates.device).view(-1, 1),
                            torch.arange(1, 1 + num_candidates, device=action_candidates.device).view(1, -1),
                            action_room_id] = action_y
        all_steps_remaining[:, 1:] -= 1

        room_mask_flat = all_room_mask.view(num_envs * (1 + num_candidates), num_rooms)
        room_position_x_flat = all_room_position_x.view(num_envs * (1 + num_candidates), num_rooms)
        room_position_y_flat = all_room_position_y.view(num_envs * (1 + num_candidates), num_rooms)
        steps_remaining_flat = all_steps_remaining.view(num_envs * (1 + num_candidates))
        # round_flat = all_round.view(num_envs * (1 + num_candidates))

        # torch.cuda.synchronize()
        # logging.info("Creating map")

        map_flat = self.envs[env_id].compute_map(room_mask_flat, room_position_x_flat, room_position_y_flat)

        # torch.cuda.synchronize()
        # logging.info("Model forward")
        flat_raw_logodds, _, flat_expected = model.forward_multiclass(
            map_flat, room_mask_flat, room_position_x_flat, room_position_y_flat, steps_remaining_flat)
        # raw_logodds = flat_raw_logodds.view(num_envs, 1 + num_candidates, self.envs[env_id].max_reward + 1)
        raw_logodds = flat_raw_logodds.view(num_envs, 1 + num_candidates, -1)
        expected = flat_expected.view(num_envs, 1 + num_candidates)
        state_raw_logodds = raw_logodds[:, 0, :]
        state_expected = expected[:, 0]
        action_expected = expected[:, 1:]
        return state_expected, action_expected, state_raw_logodds

    def generate_round_inner(self, model, episode_length: int, num_candidates: int, temperature: float,
                             explore_eps: float,
                             env_id, render=False) -> EpisodeData:
        device = self.envs[env_id].device
        env = self.envs[env_id]
        env.reset()
        state_raw_logodds_list = []
        action_list = []
        prob_list = []
        round_tensor = torch.full([env.num_envs], self.num_rounds, dtype=torch.int64, device=device)
        model.eval()
        # torch.cuda.synchronize()
        # logging.debug("Averaging parameters")
        for j in range(episode_length):
            if render:
                env.render()
            # torch.cuda.synchronize()
            # logging.debug("Getting candidates")
            action_candidates = env.get_action_candidates(num_candidates)
            steps_remaining = torch.full([env.num_envs], episode_length - j,
                                         dtype=torch.float32, device=device)
            with torch.no_grad():
                # print("inner", env_id, j, env.device, model.state_value_lin.weight.device)
                state_expected, action_expected, state_raw_logodds = self.forward_state_action(
                    model, env.room_mask, env.room_position_x, env.room_position_y,
                    action_candidates, steps_remaining, env_id=env_id, round=torch.zeros_like(round_tensor))
            probs = torch.softmax(action_expected / temperature, dim=1)
            probs = torch.full_like(probs, explore_eps / num_candidates) + (
                    1 - explore_eps) * probs
            action_index = _rand_choice(probs)
            selected_prob = probs[torch.arange(env.num_envs, device=device), action_index]
            action = action_candidates[torch.arange(env.num_envs, device=device), action_index, :]

            env.step(action)
            action_list.append(action.to('cpu'))
            state_raw_logodds_list.append(state_raw_logodds.to('cpu'))
            prob_list.append(selected_prob.to('cpu'))

        reward_tensor = env.reward().to('cpu')
        door_connects_tensor = env.door_connects().to('cpu')
        state_raw_logodds_tensor = torch.stack(state_raw_logodds_list, dim=1)
        action_tensor = torch.stack(action_list, dim=1)
        prob_tensor = torch.mean(torch.stack(prob_list, dim=1), dim=1)

        state_raw_logodds_flat = state_raw_logodds_tensor.view(env.num_envs * episode_length,
                                                               state_raw_logodds_tensor.shape[-1])
        # reward_flat = reward_tensor.view(env.num_envs, 1).repeat(1, episode_length).view(-1)
        door_connects_flat = door_connects_tensor.view(env.num_envs, 1, -1).repeat(1, episode_length, 1).view(env.num_envs * episode_length, -1)
        # loss_flat = torch.nn.functional.cross_entropy(state_raw_logodds_flat, reward_flat, reduction='none')
        loss_flat = torch.mean(torch.nn.functional.binary_cross_entropy_with_logits(state_raw_logodds_flat,
                                                                    door_connects_flat.to(state_raw_logodds_flat.dtype),
                                                                    reduction='none'), dim=1)
        loss = loss_flat.view(env.num_envs, episode_length)
        episode_loss = torch.mean(loss, dim=1)

        return EpisodeData(
            reward=reward_tensor,
            door_connects=door_connects_tensor,
            action=action_tensor.to(torch.uint8),
            prob=prob_tensor,
            test_loss=episode_loss,
        )

    def generate_round(self, episode_length: int, num_candidates: int, temperature: float, explore_eps: float,
                       executor: concurrent.futures.ThreadPoolExecutor,
                       render=False) -> EpisodeData:
        futures_list = []
        with self.average_parameters.average_parameters(self.model.all_param_data()):
            models = [copy.deepcopy(self.model).to(env.device) for env in self.envs]
            for i, env in enumerate(self.envs):
                model = models[i]
                # print("gen", i, env.device, model.state_value_lin.weight.device)
                future = executor.submit(lambda i=i, model=model: self.generate_round_inner(
                    model, episode_length, num_candidates, temperature, explore_eps, render=render, env_id=i))
                futures_list.append(future)
            episode_data_list = [future.result() for future in futures_list]
            for env in self.envs:
                if env.room_mask.is_cuda:
                    torch.cuda.synchronize(env.device)
            return EpisodeData(
                reward=torch.cat([d.reward for d in episode_data_list], dim=0),
                door_connects=torch.cat([d.door_connects for d in episode_data_list], dim=0),
                action=torch.cat([d.action for d in episode_data_list], dim=0),
                prob=torch.cat([d.prob for d in episode_data_list], dim=0),
                test_loss=torch.cat([d.test_loss for d in episode_data_list], dim=0),
            )

    def train_batch(self, data: TrainingData):
        self.model.train()

        if self.sam_scale is not None:
            saved_params = [param.data.clone() for param in self.model.parameters()]
            for param in self.model.parameters():
                param.data += torch.randn_like(param.data) * self.sam_scale
            self.model.project()

        map = self.envs[0].compute_map(data.room_mask, data.room_position_x, data.room_position_y)
        state_value_raw_logprobs, _, _ = self.model.forward_multiclass(
            map, data.room_mask, data.room_position_x, data.room_position_y, data.steps_remaining)

        # loss = torch.nn.functional.cross_entropy(state_value_raw_logprobs, data.reward)
        loss = torch.nn.functional.binary_cross_entropy_with_logits(state_value_raw_logprobs,
                                                                    data.door_connects.to(state_value_raw_logprobs.dtype))
        self.optimizer.zero_grad()
        self.grad_scaler.scale(loss).backward()

        if self.sam_scale is not None:
            for i, param in enumerate(self.model.parameters()):
                param.data.copy_(saved_params[i])

        self.grad_scaler.step(self.optimizer)
        self.grad_scaler.update()
        self.model.decay(self.decay_amount * self.optimizer.param_groups[0]['lr'])
        self.model.project()
        self.average_parameters.update(self.model.all_param_data())
        return loss.item()

    def eval_batch(self, data: TrainingData):
        with self.average_parameters.average_parameters(self.model.all_param_data()):
            self.model.eval()
            with torch.no_grad():
                map = self.envs[0].compute_map(data.room_mask, data.room_position_x, data.room_position_y)
                state_value_raw_logprobs, _, state_value_expected = self.model.forward_multiclass(
                    map, data.room_mask, data.room_position_x, data.room_position_y, data.steps_remaining)

        # loss = torch.nn.functional.cross_entropy(state_value_raw_logprobs, data.reward)
        loss = torch.nn.functional.binary_cross_entropy_with_logits(state_value_raw_logprobs,
                                                                    data.door_connects.to(state_value_raw_logprobs.dtype))
        mse = torch.nn.functional.mse_loss(state_value_expected, data.reward)
        return loss.item(), mse.item()
