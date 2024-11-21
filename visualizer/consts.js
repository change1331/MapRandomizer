const offsets = {
	"Morph Ball Room: Left Item": [2, 2],
	"Morph Ball Room: Right Item": [4, 2],
	"Cathedral: Hidden Item": [2, 1],
	"The Precious Room: Top Right Hidden Item": [1, 0],
	"Mama Turtle Room: Hidden Right Item": [2, 1],
	"Mama Turtle Room: Top Item": [1, 0],
	"Brinstar Reserve Tank Room: Middle Visible Item": [0.75, 0],
	"Brinstar Reserve Tank Room: Right Hidden Item": [1.25, 0],
	"Brinstar Reserve Tank Room: Chozo Item": [0, 0],
	"Lower Norfair Fireflea Room: Firefleas Item": [2, 5],
	"Main Street: Morph Tunnel Item": [1, 2],
	"Main Street: Speed Blocked Item": [0, 3],
	"Gauntlet Energy Tank Room: Item": [5, 0],
	"Wrecked Ship Main Shaft: Item": [0, 5],
	"Pseudo Plasma Spark Room: Hidden Item": [2, 2],
	"Assembly Line: Item": [2, 0],
	"Warehouse Kihunter Room: Hidden Item": [2, 0],
	"Billy Mays Room: Pedestal Item": [0.25, 0],
	"Billy Mays Room: Hidden Item": [-0.25, 0],
	"Norfair Reserve Tank Room: Hidden Platform Item": [0.5, 0],
	"Green Hill Zone: Item (Through the Pipe)": [3, 1],
	"Crocomire's Room: Item": [7, 0],
	"Blue Brinstar Energy Tank Room: Hidden Ceiling Item": [1, 2],
	"Blue Brinstar Energy Tank Room: Right Item": [2, 2],
	"Three Musketeers' Room: Hidden Item": [0, 2],
	"Alpha Power Bomb Room: Chozo Item": [1, 0],
	"Alpha Power Bomb Room: Hidden Left Item": [0, 0],
	"Aqueduct: Top Right Right Item": [5, 0],
	"Aqueduct: Top Right Left Item": [4, 0],
	"Screw Attack Room: Item": [0, 2],
	"Bubble Mountain: Bottom Right Item": [1, 3],
	"Watering Hole: Left Item": [-0.25, 2],
	"Watering Hole: Right Item": [0.25, 2],
	"Speed Booster Hall: Hidden Item": [11, 1],
	"West Sand Hole: Top Left - Right Item": [0.25, 0],
	"West Sand Hole: Top Left - Left Item": [-0.25, 0],
	"East Sand Hole: Top Left Item": [0, 0],
	"East Sand Hole: Right Item": [1, 1],
	"Post Crocomire Missile Room: Item": [3, 0],
	"Double Chamber: Item": [1, 0],
	"Beta Power Bomb Room: Item": [0, 1],
	"West Ocean: Bottom Left Underwater Item": [0, 5],
	"West Ocean: Left Morph Maze Item": [0, 2],
	"West Ocean: Top Hidden Item (Above Trippers)": [1, 0],
	"Early Supers Room: Lower Item": [1, 1],
	"Early Supers Room: Top Left Item": [0, 0],
	"Plasma Room: Item": [1, 2],
	"Green Pirates Shaft: Left Item": [-0.25, 1],
	"Green Pirates Shaft: Right Item": [0.25, 1],
	"Spring Ball Room: Item": [1, 1],
	"Pink Brinstar Power Bomb Room: Item": [0, 1],
	"Pit Room: Item": [0, 1],
	"Mickey Mouse Room: Item": [2, 1],
	"Grapple Beam Room: Item": [0, 2],
	"Golden Torizo's Room: Hidden Right Item": [1, 0],
	"Golden Torizo's Room: Top Left Item": [0, 0],
	"Wrecked Ship East Super Room: Item (Behind the Bomb Wall)": [3, 0],
	"Botwoon Energy Tank Room: Item": [3, 0],
	"Terminator Room: Item": [0, 2],
	"Post Crocomire Jump Room: Item": [4, 0],
	"Green Bubbles Missile Room: Item": [1, 0],
	"Lower Norfair Spring Ball Maze Room: Item": [2, 0],
	"Crateria Power Bomb Room: Item": [1, 0],
	"Green Brinstar Main Shaft: Right Etecoon Shaft - Item": [3, 7],
	"Crateria Super Room: Item": [3, 0],
	"Spore Spawn Super Room: Item": [1, 8],
	"Hi Jump Energy Tank Room: Top Right Item": [1, 0],
	"Hi Jump Energy Tank Room: Top Left Item": [0, 0],
	"Big Pink: Top Item (Above Hopper Pit)": [2, 3],
	"Big Pink: Middle Item": [2, 6],
	"Big Pink: Bottom Chozo Item": [2, 7],
	"Bowling Alley: Bottom Chozo Item": [3, 2],
	"Bowling Alley: Top Right Item": [5, 0],
};
let item_plm = {
	"ETank": 0,
	"Missile": 1,
	"Super": 2,
	"PowerBomb": 3,
	"Bombs": 4,
	"Charge": 5,
	"Ice": 6,
	"HiJump": 7,
	"SpeedBooster": 8,
	"Wave": 9,
	"Spazer": 10,
	"SpringBall": 11,
	"Varia": 12,
	"Gravity": 13,
	"XRayScope": 14,
	"Plasma": 15,
	"Grapple": 16,
	"SpaceJump": 17,
	"ScrewAttack": 18,
	"Morph": 19,
	"ReserveTank": 20,
	"WallJump": 21,
};
let itemtypes = {
	"majors": ["Varia",
			"Gravity",
			"Morph",
			"SpaceJump",
			"ScrewAttack",
			"WallJump",
			"Bombs",
			"HiJump",
			"SpeedBooster",
			"SpringBall",
			"Grapple",
			"XRayScope",
			"Charge",
			"Ice",
			"Wave",
			"Spazer",
			"Plasma"],
	"health": ["ETank", "ReserveTank"],
	"ammo": ["Super","PowerBomb"],
	"missiles": ["Missile"]
}
let item_rank = {
	"Varia": 1,
	"Gravity": 2,
	"Morph": 3,
	"SpaceJump": 4,
	"ScrewAttack": 5,
	"WallJump": 6,
	"Bombs": 7,
	"HiJump": 8,
	"SpeedBooster": 9,
	"SpringBall": 10,
	"Grapple": 11,
	"XRayScope": 12,
	"Charge": 13,
	"Ice": 14,
	"Wave": 15,
	"Spazer": 16,
	"Plasma": 17,
	"ETank": 18,
	"ReserveTank": 19,
	"Super": 20,
	"PowerBomb": 21,
	"Missile": 22,
};
let roomFlags = {
	"Bomb Torizo Room": ["f_DefeatedBombTorizo", "Defeat Bomb Torizo", 0,0],
	"Botwoon's Room": ["f_DefeatedBotwoon", "Defeat Botwoon",0.5,0],
	"Crocomire's Room": ["f_DefeatedCrocomire", "Defeat Crocomire",4,0],
	"Draygon's Room": ["f_DefeatedDraygon", "Defeat Draygon",0.5,0.5],
	"Golden Torizo's Room": ["f_DefeatedGoldenTorizo", "Defeat Golden Torizo",0.5,1],
	"Kraid Room": ["f_DefeatedKraid", "Defeat Kraid",0.5,0.5],
	"Mother Brain Room": ["f_DefeatedMotherBrain", "Defeat Mother Brain",1.5,0],
	"Phantoon's Room": ["f_DefeatedPhantoon", "Defeat Phantoon",0,0],
	"Ridley's Room": ["f_DefeatedRidley", "Defeat Ridley",0,0.5],
	"Spore Spawn Room": ["f_DefeatedSporeSpawn", "Defeat Spore Spawn",0,1.5],
	"Metroid Room 1": ["f_KilledMetroidRoom1", "Clear Metroid Room 1",2.5,0],
	"Metroid Room 2": ["f_KilledMetroidRoom2", "Clear Metroid Room 2",0,0.5],
	"Metroid Room 3": ["f_KilledMetroidRoom3", "Clear Metroid Room 3",2.5,0],
	"Metroid Room 4": ["f_KilledMetroidRoom4", "Clear Metroid Room 4",0,0.5],
	"Glass Tunnel": ["f_MaridiaTubeBroken", "Break Maridia Tube",0,1],
	"Shaktool Room": ["f_ShaktoolDoneDigging", "Clear Shaktool Room",1.5,0],
	"Acid Statue Room": ["f_UsedAcidChozoStatue", "Use Acid Statue",0.5,0.5],
	"Bowling Alley": ["f_UsedBowlingStatue", "Use Bowling Statue",4,1],
	"Pit Room": ["f_ClearedPitRoom", "Clear Pit Room",1,0],
	"Baby Kraid Room": ["f_ClearedBabyKraidRoom", "Clear Baby Kraid Room",2.5,0],
	"Plasma Room": ["f_ClearedPlasmaRoom", "Clear Plasma Room",0.5,1],
	"Metal Pirates Room": ["f_ClearedMetalPiratesRoom", "Clear Metal Pirates Room",1,0],
	"Parlor And Alcatraz": ["f_ZebesAwake", "Awaken Zebes",2, 0.5],
	"Climb": ["f_ZebesAwake", "Awaken Zebes",1,4],
	"Construction Zone": ["f_ZebesAwake", "Awaken Zebes",0,1.5],
	"The Final Missile": ["f_ZebesAwake", "Awaken Zebes",0,0],
	"Morph Ball Room": ["f_ZebesAwake", "Awaken Zebes",5,2],
	"Blue Brinstar Energy Tank Room": ["f_ZebesAwake", "Awaken Zebes",0,2.5],
};
let flagtypes = {
	"bosses":["f_DefeatedKraid","f_DefeatedPhantoon",
		"f_DefeatedDraygon","f_DefeatedRidley","f_DefeatedMotherBrain"],
	"minibosses":["f_DefeatedBombTorizo","f_DefeatedSporeSpawn","f_DefeatedCrocomire",
		"f_DefeatedBotwoon","f_DefeatedGoldenTorizo"],
	"misc":["f_KilledMetroidRoom1","f_KilledMetroidRoom2","f_KilledMetroidRoom3",
		"f_KilledMetroidRoom4","f_MaridiaTubeBroken","f_ShaktoolDoneDigging",
		"f_UsedAcidChozoStatue","f_UsedBowlingStatue","f_ClearedPitRoom",
		"f_ClearedBabyKraidRoom","f_ClearedPlasmaRoom","f_ClearedMetalPiratesRoom",
		"f_ZebesAwake"]
}