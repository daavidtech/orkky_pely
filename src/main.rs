
use bevy::DefaultPlugins;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use cursor::initial_grab_cursor;
use game_plugin::GamePlugin;
use keymap::Keymap;
use map_loader::create_map_loader;
use menu_plugin::MenuPlugin;
use splash_plugin::SplashPlugin;
use types::GameState;

mod game_ui_plugin;
mod gltf;
mod animations;
mod npc;
mod types;
mod player;
mod map;
mod map_loader;
mod map_changes;
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
mod console_plugin;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
	let keymap = Keymap::load("./config/keymap.json");

	let changes_receiver = create_map_loader("./config/map.json");

    App::new()
		.insert_resource(changes_receiver)
		.insert_resource(keymap)
		.add_plugins(DefaultPlugins.set(LogPlugin {
			level: bevy::log::Level::INFO,
			..Default::default()
		}))
		.add_startup_system(initial_grab_cursor)
		.add_state(GameState::Game)	
		
		.add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
		.add_plugin(GamePlugin)
		.add_plugin(WorldInspectorPlugin::new())
		.run();
}
