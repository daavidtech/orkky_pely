use animations::{link_animation_players, AnimationStore, change_character_animation, handle_start_animation, handle_stop_animation};
use collisions::{add_collisions, move_melee_hitbox};
use gltf::unpack_gltf;
use bevy::{prelude::*, log::LogPlugin, window::CursorGrabMode, utils::HashSet, gltf::{Gltf, GltfMesh, GltfNode}, pbr::wireframe::{WireframePlugin}};
use bevy_fps_controller::controller::{FpsControllerPlugin};
use bevy_rapier3d::{prelude::{RapierConfiguration, NoUserData, RapierPhysicsPlugin, Collider, ComputedColliderShape}, render::RapierDebugRenderPlugin};
use character::{Character};
use input_handling::{keyboard_handler, mouse_handlers};
use keymap::Keymap;
use map_changes::{handle_map_changes, handle_needs_template, give_assets};
use types::{You, MapTemplates, GltfRegister, AssetPacks, PlayerIds};

mod character;
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

fn main() {
	let keymap = Keymap::load("./config/keymap.json");

	let changes_receiver = map_loader::create_map_loader("./config/map.json");

    App::new()
		.insert_resource(RapierConfiguration::default())
		.insert_resource(AnimationStore::default())
		.insert_resource(MapTemplates::default())
		.insert_resource(GltfRegister::default())
		.insert_resource(AssetPacks::default())
		.insert_resource(PlayerIds::default())
		.insert_resource(changes_receiver)
		.insert_resource(keymap)
    	.add_plugins(DefaultPlugins.set(LogPlugin {
			level: bevy::log::Level::INFO,
			..Default::default()
		}))
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugin(FpsControllerPlugin)
		.add_plugin(WireframePlugin)
		// .add_plugin(RapierDebugRenderPlugin::default())
		.add_startup_system(initial_grab_cursor)
		.add_system(handle_start_animation)
		.add_system(handle_stop_animation)
		.add_system(detect_animation_players)
		.add_system(link_animation_players)
		.add_system(handle_your_keyboard_input)
		.add_system(change_character_animation)
		.add_system(handle_map_changes)
		.add_system(handle_needs_template)
		.add_system(unpack_gltf)
		.add_system(give_assets)
		.add_system(add_collisions)
		.add_system(keyboard_handler)
		.add_system(mouse_handlers)
		.add_system(move_melee_hitbox)
		.run();
}

fn handle_your_keyboard_input(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Character, &You)>
) {
	let pressed = keyboard_input.get_just_pressed();

	for key in pressed {
		match &key {
			KeyCode::W | KeyCode::A | KeyCode::S | KeyCode::D => {
				for (mut char, _) in query.iter_mut() {
					char.moving = true;
				}
			},
			KeyCode::R => {
				for (mut char, _) in query.iter_mut() {
					char.reloading = true;
				}
			},
			KeyCode::Space => {
				for (mut char, _) in query.iter_mut() {
					char.jump = true;
				}
			},
			KeyCode::LShift => {
				for (mut char, _) in query.iter_mut() {
					char.running = true;
				}
			},
			_ => {}
		}
	}

	let released = keyboard_input.get_just_released();

	for key in released {
		match &key {
			KeyCode::W | KeyCode::A | KeyCode::S | KeyCode::D => {
				for (mut char, _) in query.iter_mut() {
					char.moving = false;
				}
			},
			KeyCode::R => {
				for (mut char, _) in query.iter_mut() {
					char.reloading = false;
				}
			},
			KeyCode::Space => {
				for (mut char, _) in query.iter_mut() {
					char.jump = false;
				}
			},
			KeyCode::LShift => {
				for (mut char, _) in query.iter_mut() {
					char.running = false;
				}
			},
			_ => {}
		}
	}
}

// fn handle_your_mouse_input(
// 	mut commands: Commands,
// 	mouse_input: Res<Input<MouseButton>>,
// 	mut query: Query<(Entity, &You)>
// ) {
// 	let pressed = mouse_input.get_just_pressed();

// 	let (entity, _) = query.single();
// 	let entity_commands = commands.entity(entity);

// 	for key in pressed {
// 		match &key {
// 			MouseButton::Left => {
// 				for (entity, _) in query.iter_mut() {
// 					char.shooting = true;
// 				}
// 			},
// 			MouseButton::Right => {
// 				for (mut char, _) in query.iter_mut() {
// 					char.aiming = true;
// 				}
// 			},
// 			_ => {}
// 		}
// 	}

// 	let released = mouse_input.get_just_released();

// 	for key in released {
// 		match &key {
// 			MouseButton::Left => {
// 				for (mut char, _) in query.iter_mut() {
// 					char.shooting = false;
// 				}
// 			},
// 			MouseButton::Right => {
// 				for (mut char, _) in query.iter_mut() {
// 					char.aiming = false;
// 				}
// 			},
// 			_ => {}
// 		}
// 	}
// }

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);
        }
        _ => {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn detect_animation_players(
	mut map: Local<HashSet<Entity>>,
	query: Query<(Entity, &AnimationPlayer)>
) {
	for (entity, _) in query.iter() {
		if !map.contains(&entity) {
			log::info!("new animation player found {:?}", entity);

			map.insert(entity);
		}
	}
}
