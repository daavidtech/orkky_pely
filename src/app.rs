use bevy::DefaultPlugins;
use bevy::log::LogPlugin;
use bevy::prelude::App;
use bevy::prelude::PluginGroup;

use crate::cursor::initial_grab_cursor;
use crate::game_over::GameOverPlugin;
use crate::game_plugin::GamePlugin;
use crate::gltf::asset_loading;
use crate::map::Map;
use crate::menu_plugin::MenuPlugin;
use crate::splash_plugin::SplashPlugin;
use crate::types::GameState;


pub fn run_app(map: Map) {
	App::new()
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
