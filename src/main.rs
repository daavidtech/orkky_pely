use std::{f32::consts::{TAU, PI}};

use animations::{link_animation_players, AnimationEntityLink};
use assets::{AssetManager, UnloadedAssets, AnimationStore};
use bevy::{prelude::*, log::LogPlugin, window::CursorGrabMode, input::keyboard::KeyboardInput, utils::{HashMap, HashSet}, gltf::Gltf, core_pipeline::core_2d::graph::input};
use bevy_fps_controller::controller::{FpsControllerPlugin, FpsController, FpsControllerInput, LogicalPlayer, RenderPlayer};
use bevy_rapier3d::prelude::{RapierConfiguration, NoUserData, RapierPhysicsPlugin, GravityScale, Sleeping, AdditionalMassProperties, ActiveEvents, RigidBody, LockedAxes, Collider, Ccd, Velocity};
use character::{Character, CharacterType, You};

use crate::character::spawn_camera_person;

mod character;
mod assets;
mod character_visuals;
mod animations;

fn main() {
    App::new()
		.insert_resource(RapierConfiguration::default())
		.insert_resource(UnloadedAssets::default())
		.insert_resource(AnimationStore::default())
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
		.add_system(handle_keyboard)
		.add_system(handle_mouse)
		.add_system(link_animation_players)
		.add_system(find_character_players)
		// .add_system(move_animations)
		.run();
}

fn find_character_players(
	mut player_query: Query<&mut AnimationPlayer>,
	mut query: Query<(&Character, &AnimationEntityLink)>
) {
	// for (char, link) in query.iter_mut() {
	// 	log::info!("Character: {:?}", char);
	// 	log::info!("Link: {:?}", link);

	// 	if let Ok(mut player) = player_query.get_mut(link.0) {
	// 		//Stuff

	// 		log::info!("Player found");
	// 	}
	// }
}

fn handle_mouse(
	buttons: Res<Input<MouseButton>>,
	animation_store: Res<AnimationStore>,
	mut player_query: Query<&mut AnimationPlayer>,
	mut query: Query<(&Character, &AnimationEntityLink)>
) {
	let pressed = buttons.get_just_pressed();

	for it in pressed {
		match it {
			MouseButton::Left => {
				for (char, link) in query.iter_mut() {
					if let Ok(mut player) = player_query.get_mut(link.0) {
						match animation_store.0.get(format!("{}-{}", char.asset_name, char.shooting_animation).as_str()) {
							Some(animation) => {
								player.start(animation.clone()).repeat();
							}
							None => {
								log::info!("No animation found for {}", char.shooting_animation);
							}
						}
					}
				}
			},
			MouseButton::Right => {
				log::info!("Right mouse button pressed");
			},
			_ => {}
		}
	}
}

fn handle_keyboard(
	input: Res<Input<KeyCode>>,
	animation_store: Res<AnimationStore>,
	mut player_query: Query<&mut AnimationPlayer>,
	mut query: Query<(&Character, &AnimationEntityLink)>
	// mut query: Query<(&mut AnimationPlayer, &Character), With<You>>
) {
	let pressed = input.get_just_pressed();

	for it in pressed {
		match it {
			KeyCode::W | KeyCode::A | KeyCode::S | KeyCode::D => {
				for (char, link) in query.iter_mut() {
					if let Ok(mut player) = player_query.get_mut(link.0) {
						match animation_store.0.get(format!("{}-{}", char.asset_name, char.walking_animation).as_str()) {
							Some(animation) => {
								player.start(animation.clone()).repeat();
							}
							None => {
								log::info!("No animation found for {}", char.walking_animation);
							}
						}
					}
				}
			}
			KeyCode::R => {
				for (char, link) in query.iter_mut() {
					if let Ok(mut player) = player_query.get_mut(link.0) {
						match animation_store.0.get(format!("{}-{}", char.asset_name, char.reload_animation).as_str()) {
							Some(animation) => {
								player.start(animation.clone());
							}
							None => {
								log::info!("No animation found for {}", char.reload_animation);
							}
						}
					}
				}
			}
			_ => {}
		}
	}
}

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
	mut commands: Commands,
	mut unloaded_assets: ResMut<UnloadedAssets>,
	mut animations: ResMut<AnimationStore>,
	assets_gltf: Res<Assets<Gltf>>,
	assets_animation_clips: Res<Assets<AnimationClip>>,
) {
	unloaded_assets.0.retain(|(name, p)| {
		if let Some(gltf) = assets_gltf.get(&p) {			
			log::info!("gltf loaded");

			for (animation_name, clip) in gltf.named_animations.iter() {
				log::info!("{} has animation {}", name, animation_name);	
				animations.0.insert(format!("{}-{}", name, animation_name), clip.clone());			
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

	let mut asset_manager = AssetManager::new();
	asset_manager.add_asset("smg_fps_animations", asset_server.load("smg_fps_animations.glb"));
	asset_manager.add_asset("fox", asset_server.load("fox.glb"));
	asset_manager.add_asset("orkki", asset_server.load("orkki.glb"));

	commands.insert_resource(asset_manager);

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
	.insert(Collider::cuboid(1.0, 10.0, 20.0))
	.insert(RigidBody::Fixed)
	.insert(Transform::IDENTITY);
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 6000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

	// commands.spawn(SceneBundle {
    //     scene: asset_server.load("orkki.glb#Scene0"),
    //     transform: Transform::from_xyz(-15.0, 1.0, 0.0),
    //     ..default()
    // });

	// commands.spawn(SceneBundle {
    //     scene: asset_server.load("demon.glb#Scene0"),
    //     transform: Transform::from_xyz(-10.0, 1.0, 0.0),
    //     ..default()
    // });

    commands.spawn((
        Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        RigidBody::Dynamic,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED,
        AdditionalMassProperties::Mass(1.0),
        GravityScale(0.0),
        Ccd { enabled: true }, // Prevent clipping when going fast
        TransformBundle::from_transform(Transform::from_xyz(10.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)),
        LogicalPlayer(0),
        FpsControllerInput {
            pitch: -TAU / 12.0,
            yaw: TAU * 5.0 / 8.0,
            ..default()
        },
        FpsController { ..default() }
    ));

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

	// commands.spawn((
	// 	SceneBundle {
	// 		scene: asset_server.load("smg_fps_animations.glb#Scene0"),
	// 		transform: Transform::from_xyz(10.0, 1.4, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
	// 		..default()
	// 	},
	// 	PlayerHands,
	// ));

	// commands.spawn((
    //     Camera3dBundle {
	// 		transform: Transform::from_xyz(10.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
	// 		..default()
	// 	},
    //     RenderPlayer(0)
    // ));

	// commands.spawn((
	// 	// SceneBundle {
	// 	// 	scene: asset_server.load("smg_fps_animations.glb#Scene0"),
	// 	// 	transform: Transform {
	// 	// 		rotation: Quat::from_rotation_y(PI),
	// 	// 		translation: Vec3::new(10.0,1.0, 10.0),
	// 	// 		..Default::default()
	// 	// 	},
	// 	// 	..default()
	// 	// },
	// 	SceneBundle {
	// 		scene: asset_server.load("smg_fps_animations.glb#Scene0"),
	// 		transform: Transform::from_xyz(10.0, 1.4, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
	// 		..default()
	// 	},
	// 	Character,
	// ));

	spawn_camera_person(asset_server,  unloaded, commands);

	// commands.spawn((
	// 	SceneBundle {
	// 		scene: asset_server.load("smg_fps_animations.glb#Scene0"),
	// 		transform: Transform::from_xyz(10.0, 1.4, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
	// 		..default()
	// 	},
	// 	PlayerHands,
	// ));

	// commands.spawn((
    //     Camera3dBundle {
	// 		transform: Transform::from_xyz(10.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
	// 		..default()
	// 	},
    //     RenderPlayer(0)
    // ));

    // camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}
