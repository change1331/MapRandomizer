use std::mem::swap;

use hashbrown::HashSet;

use crate::{
    game_data::{Capacity, GameData, Item, Link, Requirement, WeaponMask},
    randomize::DifficultyConfig,
};

#[derive(Clone, Debug)]
pub struct GlobalState {
    pub tech: Vec<bool>,
    pub items: Vec<bool>,
    pub flags: Vec<bool>,
    pub max_energy: Capacity,
    pub max_reserves: Capacity,
    pub max_missiles: Capacity,
    pub max_supers: Capacity,
    pub max_power_bombs: Capacity,
    pub weapon_mask: WeaponMask,
    pub shine_charge_tiles: Capacity,
}

#[derive(Copy, Clone, Debug)]
pub struct LocalState {
    pub energy_used: Capacity,
    pub reserves_used: Capacity,
    pub missiles_used: Capacity,
    pub supers_used: Capacity,
    pub power_bombs_used: Capacity,
}

impl LocalState {
    pub fn new() -> Self {
        Self {
            energy_used: 0,
            reserves_used: 0,
            missiles_used: 0,
            supers_used: 0,
            power_bombs_used: 0,
        }
    }
}

fn compute_cost(local: LocalState, global: &GlobalState) -> f32 {
    let eps = 1e-15;
    let energy_cost = (local.energy_used as f32) / (global.max_energy as f32 + eps);
    let reserve_cost = (local.reserves_used as f32) / (global.max_reserves as f32 + eps);
    let missiles_cost = (local.missiles_used as f32) / (global.max_missiles as f32 + eps);
    let supers_cost = (local.supers_used as f32) / (global.max_supers as f32 + eps);
    let power_bombs_cost = (local.power_bombs_used as f32) / (global.max_power_bombs as f32 + eps);
    energy_cost + reserve_cost + missiles_cost + supers_cost + power_bombs_cost
}

fn validate_energy(mut local: LocalState, global: &GlobalState) -> Option<LocalState> {
    if local.energy_used >= global.max_energy {
        local.reserves_used += local.energy_used - (global.max_energy - 1);
        local.energy_used = global.max_energy - 1;
    }
    if local.reserves_used > global.max_reserves {
        return None;
    }
    Some(local)
}

fn validate_missiles(local: LocalState, global: &GlobalState) -> Option<LocalState> {
    if local.missiles_used > global.max_missiles {
        None
    } else {
        Some(local)
    }
}

fn validate_supers(local: LocalState, global: &GlobalState) -> Option<LocalState> {
    if local.supers_used > global.max_supers {
        None
    } else {
        Some(local)
    }
}

fn validate_power_bombs(local: LocalState, global: &GlobalState) -> Option<LocalState> {
    if local.power_bombs_used > global.max_power_bombs {
        None
    } else {
        Some(local)
    }
}

fn multiply(amount: Capacity, difficulty: &DifficultyConfig) -> Capacity {
    ((amount as f32) * difficulty.resource_multiplier) as Capacity
}

pub fn apply_requirement(
    req: &Requirement,
    global: &GlobalState,
    local: LocalState,
    reverse: bool,
    difficulty: &DifficultyConfig,
) -> Option<LocalState> {
    match req {
        Requirement::Free => Some(local),
        Requirement::Never => None,
        Requirement::Tech(tech_id) => {
            if global.tech[*tech_id] {
                Some(local)
            } else {
                None
            }
        }
        Requirement::Item(item_id) => {
            if global.items[*item_id] {
                Some(local)
            } else {
                None
            }
        }
        Requirement::Flag(flag_id) => {
            if global.flags[*flag_id] {
                Some(local)
            } else {
                None
            }
        }
        Requirement::HeatFrames(frames) => {
            let varia = global.items[Item::Varia as usize];
            let gravity = global.items[Item::Gravity as usize];
            let mut new_local = local;
            if varia {
                Some(new_local)
            } else if gravity {
                new_local.energy_used += multiply(frames / 8, difficulty);
                validate_energy(new_local, global)
            } else {
                new_local.energy_used += multiply(frames / 4, difficulty);
                validate_energy(new_local, global)
            }
        }
        Requirement::LavaFrames(frames) => {
            let varia = global.items[Item::Varia as usize];
            let gravity = global.items[Item::Gravity as usize];
            let mut new_local = local;
            if gravity {
                Some(new_local)
            } else if varia {
                new_local.energy_used += multiply(frames / 4, difficulty);
                validate_energy(new_local, global)
            } else {
                new_local.energy_used += multiply(frames / 2, difficulty);
                validate_energy(new_local, global)
            }
        }
        Requirement::LavaPhysicsFrames(frames) => {
            let varia = global.items[Item::Varia as usize];
            let mut new_local = local;
            if varia {
                new_local.energy_used += multiply(frames / 4, difficulty);
            } else {
                new_local.energy_used += multiply(frames / 2, difficulty);
            }
            validate_energy(new_local, global)
        }
        Requirement::Damage(base_energy) => {
            let varia = global.items[Item::Varia as usize];
            let gravity = global.items[Item::Gravity as usize];
            let mut new_local = local;
            if gravity && varia {
                new_local.energy_used += multiply(base_energy / 4, difficulty);
            } else if gravity || varia {
                new_local.energy_used += multiply(base_energy / 2, difficulty);
            } else {
                new_local.energy_used += multiply(*base_energy, difficulty);
            }
            validate_energy(new_local, global)
        }
        // Requirement::Energy(count) => {
        //     let mut new_local = local;
        //     new_local.energy_used += count;
        //     validate_energy(new_local, global)
        // },
        Requirement::Missiles(count) => {
            let mut new_local = local;
            new_local.missiles_used += multiply(*count, difficulty);
            validate_missiles(new_local, global)
        }
        Requirement::Supers(count) => {
            let mut new_local = local;
            new_local.supers_used += multiply(*count, difficulty);
            validate_supers(new_local, global)
        }
        Requirement::PowerBombs(count) => {
            let mut new_local = local;
            new_local.power_bombs_used += multiply(*count, difficulty);
            validate_power_bombs(new_local, global)
        }
        Requirement::EnergyRefill => {
            let mut new_local = local;
            new_local.energy_used = 0;
            Some(new_local)
        }
        Requirement::ReserveRefill => {
            let mut new_local = local;
            new_local.reserves_used = 0;
            Some(new_local)
        }
        Requirement::MissileRefill => {
            let mut new_local = local;
            new_local.missiles_used = 0;
            Some(new_local)
        }
        Requirement::SuperRefill => {
            let mut new_local = local;
            new_local.supers_used = 0;
            Some(new_local)
        }
        Requirement::PowerBombRefill => {
            let mut new_local = local;
            new_local.power_bombs_used = 0;
            Some(new_local)
        }
        Requirement::EnergyDrain => {
            if reverse {
                let mut new_local = local;
                new_local.reserves_used += new_local.energy_used;
                new_local.energy_used = 0;
                if new_local.reserves_used > global.max_reserves {
                    None
                } else {
                    Some(new_local)
                }
            } else {
                let mut new_local = local;
                new_local.energy_used = global.max_energy - 1;
                Some(new_local)
            }
        }
        Requirement::EnemyKill(weapon_mask) => {
            // TODO: Take into account ammo-kill strats
            if global.weapon_mask & *weapon_mask != 0 {
                Some(local)
            } else {
                None
            }
        }
        Requirement::ShineCharge {
            used_tiles,
            shinespark_frames,
        } => {
            if global.items[Item::SpeedBooster as usize] && *used_tiles >= global.shine_charge_tiles
            {
                let mut new_local = local;
                if reverse {
                    if new_local.energy_used < 28 {
                        new_local.energy_used = 28;
                    }
                    new_local.energy_used += shinespark_frames;
                    validate_energy(new_local, global)
                } else {
                    new_local.energy_used += shinespark_frames + 28;
                    if let Some(mut new_local) = validate_energy(new_local, global) {
                        new_local.energy_used -= 28;
                        Some(new_local)
                    } else {
                        None
                    }    
                }
            } else {
                None
            }
        }
        Requirement::And(reqs) => {
            let mut new_local = local;
            for req in reqs {
                new_local = apply_requirement(req, global, new_local, reverse, difficulty)?;
            }
            Some(new_local)
        }
        Requirement::Or(reqs) => {
            let mut best_local = None;
            let mut best_cost = f32::INFINITY;
            for req in reqs {
                if let Some(new_local) = apply_requirement(req, global, local, reverse, difficulty)
                {
                    let cost = compute_cost(new_local, global);
                    if cost < best_cost {
                        best_cost = cost;
                        best_local = Some(new_local);
                    }
                }
            }
            best_local
        }
    }
}

pub fn is_bireachable(
    global: &GlobalState,
    forward_local_state: &Option<LocalState>,
    reverse_local_state: &Option<LocalState>,
) -> bool {
    if forward_local_state.is_none() || reverse_local_state.is_none() {
        return false;
    }
    let forward = forward_local_state.unwrap();
    let reverse = reverse_local_state.unwrap();
    if forward.reserves_used + reverse.reserves_used > global.max_reserves {
        return false;
    }
    let forward_total_energy_used = forward.energy_used + forward.reserves_used;
    let reverse_total_energy_used = reverse.energy_used + reverse.reserves_used;
    let max_total_energy = global.max_energy + global.max_reserves;
    if forward_total_energy_used + reverse_total_energy_used >= max_total_energy {
        return false;
    }
    if forward.missiles_used + reverse.missiles_used > global.max_missiles {
        return false;
    }
    if forward.supers_used + reverse.supers_used > global.max_supers {
        return false;
    }
    if forward.power_bombs_used + reverse.power_bombs_used > global.max_power_bombs {
        return false;
    }
    true
}

pub type StepTrailId = i32;
pub type LinkIdx = u32;

pub struct StepTrail {
    pub prev_trail_id: StepTrailId,
    pub link_idx: LinkIdx,
}

pub struct TraverseResult {
    pub local_states: Vec<Option<LocalState>>,
    pub cost: Vec<f32>,
    pub step_trails: Vec<StepTrail>,
    pub start_trail_ids: Vec<Option<StepTrailId>>,
}

pub fn traverse(
    links: &[Link],
    global: &GlobalState,
    num_vertices: usize,
    start_vertex_id: usize,
    reverse: bool,
    difficulty: &DifficultyConfig,
    _game_data: &GameData, // May be used for debugging
) -> TraverseResult {
    let mut result = TraverseResult {
        local_states: vec![None; num_vertices],
        cost: vec![f32::INFINITY; num_vertices],
        step_trails: Vec::with_capacity(num_vertices * 10),
        start_trail_ids: vec![None; num_vertices],
    };
    result.local_states[start_vertex_id] = Some(LocalState::new());
    result.start_trail_ids[start_vertex_id] = Some(-1);
    result.cost[start_vertex_id] =
        compute_cost(result.local_states[start_vertex_id].unwrap(), global);

    let mut links_by_src: Vec<Vec<(LinkIdx, Link)>> = vec![Vec::new(); num_vertices];
    for (idx, link) in links.iter().enumerate() {
        if reverse {
            let mut reversed_link = link.clone();
            swap(
                &mut reversed_link.from_vertex_id,
                &mut reversed_link.to_vertex_id,
            );
            links_by_src[reversed_link.from_vertex_id].push((idx as LinkIdx, reversed_link));
        } else {
            links_by_src[link.from_vertex_id].push((idx as LinkIdx, link.clone()));
        }
    }

    let mut modified_vertices: HashSet<usize> = HashSet::new();
    modified_vertices.insert(start_vertex_id);
    while modified_vertices.len() > 0 {
        let mut new_modified_vertices: HashSet<usize> = HashSet::new();
        for &src_id in &modified_vertices {
            let src_local_state = result.local_states[src_id].unwrap();
            let src_trail_id = result.start_trail_ids[src_id].unwrap();
            for &(link_idx, ref link) in &links_by_src[src_id] {
                let dst_id = link.to_vertex_id;
                let dst_old_cost = result.cost[dst_id];
                if let Some(dst_new_local_state) = apply_requirement(
                    &link.requirement,
                    global,
                    src_local_state,
                    reverse,
                    difficulty,
                ) {
                    let dst_new_cost = compute_cost(dst_new_local_state, global);
                    if dst_new_cost < dst_old_cost {
                        let new_step_trail = StepTrail {
                            prev_trail_id: src_trail_id,
                            link_idx: link_idx,
                        };
                        let new_trail_id = result.step_trails.len() as StepTrailId;
                        result.step_trails.push(new_step_trail);
                        result.local_states[dst_id] = Some(dst_new_local_state);
                        result.start_trail_ids[dst_id] = Some(new_trail_id);
                        result.cost[dst_id] = dst_new_cost;
                        new_modified_vertices.insert(dst_id);
                    }
                }
            }
        }
        modified_vertices = new_modified_vertices;
    }

    result
}

impl GlobalState {
    pub fn collect(&mut self, item: Item, game_data: &GameData) {
        self.items[item as usize] = true;
        match item {
            Item::Missile => {
                self.max_missiles += 5;
            }
            Item::Super => {
                self.max_supers += 5;
            }
            Item::PowerBomb => {
                self.max_power_bombs += 5;
            }
            Item::ETank => {
                self.max_energy += 100;
            }
            Item::ReserveTank => {
                self.max_reserves += 100;
            }
            _ => {}
        }
        self.weapon_mask = game_data.get_weapon_mask(&self.items);
    }
}

pub fn get_spoiler_route(
    traverse_result: &TraverseResult,
    vertex_id: usize,
    reverse: bool,
) -> Vec<LinkIdx> {
    let mut trail_id = traverse_result.start_trail_ids[vertex_id].unwrap();
    let mut steps: Vec<LinkIdx> = Vec::new();
    while trail_id != -1 {
        let step_trail = &traverse_result.step_trails[trail_id as usize];
        steps.push(step_trail.link_idx);
        trail_id = step_trail.prev_trail_id;
    }
    if !reverse {
        steps.reverse();
    }
    steps
}