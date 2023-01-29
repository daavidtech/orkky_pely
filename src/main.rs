use std::sync::mpsc;

use animations::{link_animation_players, AnimationStore, change_character_animation};
use gltf::{UnloadedAssets, unpack_gltf};
use bevy::{prelude::*, log::LogPlugin, window::CursorGrabMode, utils::HashSet, gltf::{Gltf, GltfMesh}};
use bevy_fps_controller::controller::{FpsControllerPlugin};
use bevy_rapier3d::{prelude::{RapierConfiguration, NoUserData, RapierPhysicsPlugin, RigidBody,Collider, ComputedColliderShape}};
use character::{Character};
use entities::{handle_map_changes, handle_needs_template, give_assets};
use map::MapChange;
use map_loader::{MapChangesReceiver};
use types::{You, MapTemplates, GltfRegister, AssetPacks};

use crate::{spawn::Spawner, types::AddCollidingMesh};

mod character;
mod gltf;
mod character_visuals;
mod animations;
mod npc;
mod spawn;
mod types;
mod player;
mod inspector;
mod map;
mod map_loader;
mod entities;

fn main() {
	// let map_loader = MapLoader::new("./maps/map.json");

	// let changes = map_loader.get_map_changes();

	let changes_receiver = map_loader::create_map_loader("./maps/map.json");


    App::new()
		.insert_resource(RapierConfiguration::default())
		.insert_resource(UnloadedAssets::default())
		.insert_resource(AnimationStore::default())
		.insert_resource(MapTemplates::default())
		.insert_resource(GltfRegister::default())
		.insert_resource(AssetPacks::default())
		.insert_resource(changes_receiver)
    	.add_plugins(DefaultPlugins.set(LogPlugin {
			level: bevy::log::Level::INFO,
			..Default::default()
		}))
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugin(FpsControllerPlugin)
		.add_startup_system(setup)
		.add_startup_system(initial_grab_cursor)
		.add_system(detect_animation_players)
		.add_system(detect_unique_characters)
		.add_system(manage_assets)
		.add_system(load_character_assets)
		.add_system(link_animation_players)
		.add_system(handle_your_keyboard_input)
		.add_system(handle_your_mouse_input)
		.add_system(change_character_animation)
		.add_system(handle_map_changes)
		.add_system(handle_needs_template)
		.add_system(unpack_gltf)
		.add_system(give_assets)
		.run();
}



// fn handle_map_changes(
// 	chnages_receiver: Res<MapChangesReceiver>,
// 	mut map_templates: ResMut<MapTemplates>, 
// 	mut done: Local<bool>,
// ) {	
// 	if *done {
// 		return;
// 	}

// 	let chnages_receiver = chnages_receiver.rx.lock().unwrap();

// 	let mut changes = vec![];

// 	loop {
// 		match chnages_receiver.try_recv() {
// 			Ok(change) => {
// 				log::info!("mapchange {:?}", change);

// 				match change {
// 					MapChange::NewMapEntity(entity) => {
						
// 					},
//         			MapChange::NewMapTemplate(template) => {
// 						map_templates.templates.insert(template.name.clone(), template);
// 					},
// 				}
// 			},
// 			Err(err) => {
// 				match err {
// 					mpsc::TryRecvError::Empty => {
// 						break;
// 					},
// 					mpsc::TryRecvError::Disconnected => {
// 						log::info!("changes disconnected");
	
// 						*done = true;
	
// 						return;
// 					},
// 				}
// 			}
// 		};
// 	}
	
// 	for change in changes {
// 		log::info!("change {:?}", change);

// 		match change {
// 			MapChange::NewMapEntity(entity) => {
// 				log::info!("add entity {:?}", entity);
// 			},
// 			_ => {}
// 		}
// 	}
// }

fn add_collisions(
	assets_gltf: Res<Assets<Gltf>>,
	assets_gltf_mesh: Res<Assets<GltfMesh>>,
	assets_mesh: Res<Assets<Mesh>>,
	query: Query<(Entity, &AddCollidingMesh)>,
	mut commands: Commands,
) {
	for (entity, add_collider_mesh) in query.iter() {
		// commands.entity(item).remove::<AddCollidingMesh>();
		let pack = match assets_gltf.get(&add_collider_mesh.glft) {
			Some(pack) => {
				pack
			},
			None => continue,
		};

		for mesh in pack.meshes.iter() {
			let mesh = match assets_gltf_mesh.get(mesh) {
				Some(m) => m,
				None => continue,
			};

			log::info!("found mesh {:?}", mesh);

			for primite in &mesh.primitives {
				let mesh = assets_mesh.get(&primite.mesh).unwrap();

				let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh);

				match collider {
					Some(collider) => {
						log::info!("found collider {:?}", collider);

						commands.entity(entity).with_children(|parent| {
							parent.spawn(collider);
						});
					},
					None => {
						log::info!("mesh collider is invalid");
					}
				}
			}
		}

		commands.entity(entity).remove::<AddCollidingMesh>();
	}
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

fn handle_your_mouse_input(
	mouse_input: Res<Input<MouseButton>>,
	mut query: Query<(&mut Character, &You)>
) {
	let pressed = mouse_input.get_just_pressed();

	for key in pressed {
		match &key {
			MouseButton::Left => {
				for (mut char, _) in query.iter_mut() {
					char.shooting = true;
				}
			},
			MouseButton::Right => {
				for (mut char, _) in query.iter_mut() {
					char.aiming = true;
				}
			},
			_ => {}
		}
	}

	let released = mouse_input.get_just_released();

	for key in released {
		match &key {
			MouseButton::Left => {
				for (mut char, _) in query.iter_mut() {
					char.shooting = false;
				}
			},
			MouseButton::Right => {
				for (mut char, _) in query.iter_mut() {
					char.aiming = false;
				}
			},
			_ => {}
		}
	}
}

// fn handle_mouse(
// 	buttons: Res<Input<MouseButton>>,
// 	animation_store: Res<AnimationStore>,
// 	mut player_query: Query<&mut AnimationPlayer>,
// 	mut query: Query<(&Character, &AnimationEntityLink)>
// ) {
// 	let pressed = buttons.get_just_pressed();

// 	for it in pressed {
// 		match it {
// 			MouseButton::Left => {
// 				for (char, link) in query.iter_mut() {
// 					if let Ok(mut player) = player_query.get_mut(link.0) {
// 						match animation_store.0.get(format!("{}-{}", char.asset_name, char.shooting_animation).as_str()) {
// 							Some(animation) => {
// 								player.start(animation.clone()).repeat();
// 							}
// 							None => {
// 								log::info!("No animation found for {}", char.shooting_animation);
// 							}
// 						}
// 					}
// 				}
// 			},
// 			MouseButton::Right => {
// 				log::info!("Right mouse button pressed");
// 			},
// 			_ => {}
// 		}
// 	}
// }

// fn handle_keyboard(
// 	input: Res<Input<KeyCode>>,
// 	animation_store: Res<AnimationStore>,
// 	mut player_query: Query<&mut AnimationPlayer>,
// 	mut query: Query<(&Character, &AnimationEntityLink)>
// 	// mut query: Query<(&mut AnimationPlayer, &Character), With<You>>
// ) {
// 	let pressed = input.get_just_pressed();

// 	for it in pressed {
// 		match it {
// 			KeyCode::W | KeyCode::A | KeyCode::S | KeyCode::D => {
// 				for (char, link) in query.iter_mut() {
// 					if let Ok(mut player) = player_query.get_mut(link.0) {
// 						match animation_store.0.get(format!("{}-{}", char.asset_name, char.walking_animation).as_str()) {
// 							Some(animation) => {
// 								player.start(animation.clone()).repeat();
// 							}
// 							None => {
// 								log::info!("No animation found for {}", char.walking_animation);
// 							}
// 						}
// 					}
// 				}
// 			}
// 			KeyCode::R => {
// 				for (char, link) in query.iter_mut() {
// 					if let Ok(mut player) = player_query.get_mut(link.0) {
// 						match animation_store.0.get(format!("{}-{}", char.asset_name, char.reload_animation).as_str()) {
// 							Some(animation) => {
// 								player.start(animation.clone());
// 							}
// 							None => {
// 								log::info!("No animation found for {}", char.reload_animation);
// 							}
// 						}
// 					}
// 				}
// 			}
// 			_ => {}
// 		}
// 	}
// }

fn load_character_assets(
	query: Query<&Character>,
	asset_server: Res<AssetServer>
) {
	for character in query.iter() {
		// let asset = asset_server.load(&character.asset);
	}
}

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

fn detect_unique_characters(
	mut map: Local<HashSet<Entity>>,
	query: Query<(Entity, &Character)>
) {
	for (entity, _) in query.iter() {
		if !map.contains(&entity) {
			log::info!("new character found {:?}", entity);

			map.insert(entity);
		}
	}
}

fn manage_assets(
	mut unloaded_assets: ResMut<UnloadedAssets>,
	mut animations: ResMut<AnimationStore>,
	assets_gltf: Res<Assets<Gltf>>,
) {
	unloaded_assets.0.retain(|(name, p)| {
		if let Some(gltf) = assets_gltf.get(&p) {			
			log::info!("gltf loaded");

			for (animation_name, clip) in gltf.named_animations.iter() {
				log::info!("{} has animation {}", name, animation_name);	

				animations.set_animation(name, animation_name, clip.clone());

				// animations.0.insert(format!("{}-{}", name, animation_name), clip.clone());			
			}

			return false;
		}

		true
	});
}

// fn check_asset(
// 	mut commands: Commands,
// 	my: Res<MyAssetPack>,
// 	assets_gltf: Res<Assets<Gltf>>,
// 	mut once: Local<bool>
// ) {
// 	if *once {
// 		return;
// 	}

// 	if let Some(gltf) = assets_gltf.get(&my.0) {
// 		log::info!("gltf loaded");

// 		for (name, clip) in gltf.named_animations.iter() {
// 			log::info!("animation found {}", name);
// 		}

// 		*once = true;
// 	}
// }

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
	mut unloaded: ResMut<UnloadedAssets>,
	asset_server: Res<AssetServer>
) {
	log::info!("startup setup");

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
	.insert(Collider::cuboid(4.0, 10.0, 4.0))
	.insert(RigidBody::Fixed)
	.insert(Transform::IDENTITY);
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
	commands.insert_resource(AmbientLight {
		brightness: 0.5,
		color: Color::hex("E6EED6").unwrap(),
	});

	commands.spawn_empty()
	.insert(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Box {
			min_x: -20.0,
			max_x: 20.0,
			min_y: -0.25,
			max_y: 0.25,
			min_z: -20.0,
			max_z: 20.0,
		})),
		material: materials.add(StandardMaterial {
			base_color: Color::hex("E6EED6").unwrap(),
			..default()
		}),
		transform: Transform::IDENTITY,
		..default()
	})
	.insert(Collider::cuboid(20.0, 0.25, 20.0))
	.insert(RigidBody::Fixed)
	.insert(Transform::IDENTITY);

	let castle_asset: Handle<Gltf> = asset_server.load("castle.glb");
	let castle_scene = asset_server.load("castle.glb#Scene0");

	commands.spawn((
		SceneBundle {
			scene: castle_scene,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 0.0),
				scale: Vec3::splat(0.25),
				..Default::default()
			},
			..default()
		},
		// AddCollidingMesh {
		// 	glft: castle_asset,
		// }
	));

	Spawner::new()
		.set_you(true)
		.set_asset("smg_fps_animations.glb")
		.set_idle_animation("Rig|KDW_DPose_Idle")
		.set_walking_animation("Rig|KDW_Walk")
		.set_running_animation("Rig|KDW_Run")
		.set_reload_animation("Rig|KDW_Reload_full")
		.set_shooting_animation("Rig|KDW_Shot")
		.spawn(commands, asset_server, unloaded);
}
