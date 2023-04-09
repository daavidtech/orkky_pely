mod game_ui_plugin;
mod gltf;
mod animations;
mod npc;
mod types;
mod player;
mod map;
mod map_loader;
mod map_spawner;
mod collisions;
mod keymap;
mod input_handling;
mod splash_plugin;
mod menu_plugin;
mod game_plugin;
mod despawn;
mod cursor;
mod player_control;
mod math;
mod path_finding;
mod console_plugin;
mod attack;
mod death;
mod bullet;
mod throw;
mod game_over;
mod ingame_menu;
mod app;
mod constants;

use wasm_bindgen::prelude::*;

pub use app::run_app;
use map::Map;

#[wasm_bindgen]
pub fn run_orkkypely() {
	let map = include_str!("../config/map.json");
	let map = Map::parse(map).unwrap();

	app::run_app(map);
}
