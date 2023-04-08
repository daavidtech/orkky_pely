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

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
	let keymap = Keymap::load("./config/keymap.json");
	
	let map = Map::load("./config/map.json").unwrap();

    App::new()
		.insert_resource(keymap)
		.insert_resource(map)
		.add_plugins(DefaultPlugins.set(LogPlugin {
			level: bevy::log::Level::INFO,
			..Default::default()
		}))
		
		.add_startup_system(initial_grab_cursor)
		.add_state::<GameState>()	
		.add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
		.add_plugin(GamePlugin)
		.add_plugin(GameOverPlugin)
		.add_startup_system(asset_loading)
		// .add_plugin(WorldInspectorPlugin::new())
		.run();
}
fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}


