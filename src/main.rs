use app::run_app;
use bevy::{prelude::*, utils::FloatOrd};
use bevy::DefaultPlugins;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cursor::initial_grab_cursor;
use game_over::GameOverPlugin;
use game_plugin::GamePlugin;
use keymap::Keymap;
use map::Map;
use map_loader::create_map_loader;
use menu_plugin::MenuPlugin;
use splash_plugin::SplashPlugin;
use types::GameState;
use crate::types::*;

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

fn main() {
	let keymap = Keymap::load("./config/keymap.json");
	
	let map = Map::load("./config/map.json").unwrap();

	run_app(map);
}


