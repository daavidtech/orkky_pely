use std::{f32::consts::{TAU, PI}};

use bevy::{prelude::*, log::LogPlugin, window::CursorGrabMode, input::keyboard::KeyboardInput};
use bevy_fps_controller::controller::{FpsControllerPlugin, FpsController, FpsControllerInput, LogicalPlayer, RenderPlayer};
use bevy_rapier3d::prelude::{RapierConfiguration, NoUserData, RapierPhysicsPlugin, GravityScale, Sleeping, AdditionalMassProperties, ActiveEvents, RigidBody, LockedAxes, Collider, Ccd, Velocity};
use character::{Character, CharacterType};

use crate::character::spawn_camera_person;

mod character;

#[derive(Default, Component)]
struct PlayerHands;

fn main() {
    App::new()
		.insert_resource(RapierConfiguration::default())
    	.add_plugins(DefaultPlugins.set(LogPlugin {
			level: bevy::log::Level::INFO,
			..Default::default()
		}))
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugin(FpsControllerPlugin)
		.add_startup_system(setup)
		.add_startup_system(initial_grab_cursor)
		.add_system(character_animations)
		// .add_system(move_animations)
		.run();
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

// fn move_animations(
// 	time: Res<Time>,
// 	mut set: ParamSet<(
// 		Query<&Transform, With<RenderPlayer>>,
// 		Query<&mut Transform, With<PlayerHands>>,
// 	)>
// ) {
// 	let transform = {
// 		let p0 = set.p0();
// 		let transform = p0.single();
// 		transform.clone()
// 	};

// 	let mut p1 = set.p1();
// 	let mut transform2 = p1.single_mut();

// 	transform2.translation.x = transform.translation.x;
// 	transform2.translation.y = transform.translation.y - 0.4;
// 	transform2.translation.z = transform.translation.z;

// 	let rot = transform.rotation;

// 	transform2.rotation = rot * Quat::from_rotation_y(PI);
// }

fn character_animations(
	input: Res<Input<KeyCode>>,
	mut query: Query<(&Character, &mut AnimationPlayer)>
) {
	input.get_just_pressed().for_each(|key| {
		match key {
			KeyCode::W | KeyCode::A | KeyCode::S | KeyCode::D => {
				log::info!("move key clicked");

				for (character, mut player) in query.iter_mut() {
					log::info!("animation player found");

					if character.character_type == CharacterType::Npc {
						continue;
					}

					log::info!("set walking animation for player");

					player.start(character.walk_animation.clone());
				}
			}
			_ => {}
		}
	});
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

	commands.spawn(SceneBundle {
        scene: asset_server.load("orkki.glb#Scene0"),
        transform: Transform::from_xyz(-15.0, 1.0, 0.0),
        ..default()
    });

	commands.spawn(SceneBundle {
        scene: asset_server.load("demon.glb#Scene0"),
        transform: Transform::from_xyz(-10.0, 1.0, 0.0),
        ..default()
    });

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

	spawn_camera_person(asset_server, commands);

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
