use std::{f32::consts::{TAU, PI}, time::Duration};

use bevy::{prelude::*, log::LogPlugin, window::CursorGrabMode, input::mouse::MouseMotion, ecs::event::ManualEventReader};
use bevy_fps_controller::controller::{FpsControllerPlugin, FpsController, FpsControllerInput, LogicalPlayer, RenderPlayer};
use bevy_rapier3d::prelude::{RapierConfiguration, NoUserData, RapierPhysicsPlugin, GravityScale, Sleeping, AdditionalMassProperties, ActiveEvents, RigidBody, LockedAxes, Collider, Ccd, Velocity};

#[derive(Debug)]
enum MoveDirection {
	Forward,
	Backward,
	Leftward,
	Rightward,
	None
}

#[derive(Debug)]
enum VerticalDirection {
	Up,
	Down,
	None
}

impl Default for VerticalDirection {
	fn default() -> Self {
		VerticalDirection::None
	}
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}


#[derive(Debug)]
enum HorizontalDirection {
	Left,
	Right,
	None
}

impl Default for HorizontalDirection {
	fn default() -> Self {
		HorizontalDirection::None
	}
}

#[derive(Resource, Default)]
struct Game {
	vertical_direction: VerticalDirection,
	horizontal_direction: HorizontalDirection,
}

impl Default for MoveDirection {
	fn default() -> Self {
		MoveDirection::None
	}
}

#[derive(Default, Component)]
struct PlayerHands;

// impl Default for PlayerHands {
// 	fn default() -> Self {
// 		let scene = SceneBundle {
// 			scene: asset_server.load("smg_fps_animations.glb#Scene0"),
// 			transform: Transform::from_xyz(10.0, 1.4, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
// 			..default()
// 		};

// 		PlayerHands {
// 			scene: scene 
// 		}
// 	}
// }

fn main() {
	// let builder = env_logger::builder()
	// 	.filter_level(log::LevelFilter::Debug)
	// 	.init();

    App::new()
		.init_resource::<Game>()
		.init_resource::<InputState>()
		.insert_resource(RapierConfiguration::default())
    	.add_plugins(DefaultPlugins.set(LogPlugin {
			level: bevy::log::Level::INFO,
			..Default::default()
		}))
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
		.add_plugin(FpsControllerPlugin)
		.add_startup_system(setup)
		.add_startup_system(initial_grab_cursor)
		.add_system(move_animations)
        // .add_system(handle_keyboard)
		// .add_system(move_camera)
		// .add_system(player_look)
		.run();
}

// fn handle_keyboard(
// 	keyboard_input: Res<Input<KeyCode>>,
// 	mut game: ResMut<Game>
// ) {
// 	log::debug!("handle_keyboard");

//     if keyboard_input.just_pressed(KeyCode::W) {
//         log::info!("W pressed");

// 		game.
//     }

// 	if keyboard_input.just_released(KeyCode::W) {
// 		log::info!("W released");

// 		game.move_direction = MoveDirection::None;
// 	}

//     if keyboard_input.just_pressed(KeyCode::S) {
//         log::info!("S pressed");

// 		game.move_direction = MoveDirection::Backward;
//     }

// 	if keyboard_input.just_released(KeyCode::S) {
// 		log::info!("S released");

// 		game.move_direction = MoveDirection::None;
// 	}

// 	if keyboard_input.just_pressed(KeyCode::A) {
// 		log::info!("A pressed");

// 		game.move_direction = MoveDirection::Leftward;
// 	}

// 	if keyboard_input.just_released(KeyCode::A) {
// 		log::info!("A released");

// 		game.move_direction = MoveDirection::None;
// 	}

// 	if keyboard_input.just_pressed(KeyCode::D) {
// 		log::info!("D pressed");

// 		game.move_direction = MoveDirection::Rightward;
// 	}

// 	if keyboard_input.just_released(KeyCode::D) {
// 		log::info!("D released");

// 		game.move_direction = MoveDirection::None;
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

fn player_look(
	windows: Res<Windows>,
	motion: Res<Events<MouseMotion>>,
	mut state: ResMut<InputState>,
	mut query: Query<&mut Transform, With<Camera3d>>,
) {
	if let Some(window) = windows.get_primary() {
		let mut delta_state = state.as_mut();
		for mut transform in query.iter_mut() {
			for ev in delta_state.reader_motion.iter(&motion) {
                // match window.cursor_grab_mode() {
                //     CursorGrabMode::None => (),
                //     _ => {
                //         // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                //         let window_scale = window.height().min(window.width());
                //         delta_state.pitch -=
                //             (0.00012 * ev.delta.y * window_scale).to_radians();
                //         delta_state.yaw -=
                //             (0.00012 * ev.delta.x * window_scale).to_radians();
                //     }
                // }

				// Using smallest of height or width ensures equal vertical and horizontal sensitivity
				let window_scale = window.height().min(window.width());
				delta_state.pitch -=
					(0.00012 * ev.delta.y * window_scale).to_radians();
				delta_state.yaw -=
					(0.00012 * ev.delta.x * window_scale).to_radians();

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
		}
	}
}

fn move_camera(
	mut query: Query<&mut Transform, With<Camera3d>>,
	mut state: ResMut<InputState>,
	keyboard_input: Res<Input<KeyCode>>,
	time: Res<Time>,
	game: Res<Game>,
) {

	for (mut transform) in query.iter_mut() {
		// match game.move_direction {
		// 	MoveDirection::Forward => {
		// 		transform.translation.z -= 0.1;
		// 	},
		// 	MoveDirection::Backward => {
		// 		transform.translation.z += 0.1;
		// 	},
		// 	MoveDirection::Leftward => {
		// 		transform.translation.x -= 0.1;
		// 	},
		// 	MoveDirection::Rightward => {
		// 		transform.translation.x += 0.1;
		// 	},
		// 	_ => {}
		// }

		let mut velocity = Vec3::ZERO;
		let local_z = transform.local_z();
		let forward = -Vec3::new(local_z.x, 0., local_z.z);
		let right = Vec3::new(local_z.z, 0., -local_z.x);

		for key in keyboard_input.get_pressed() {
			// match window.cursor_grab_mode() {
			// 	CursorGrabMode::None => (),
			// 	_ => match key {
			// 		KeyCode::W => velocity += forward,
			// 		KeyCode::S => velocity -= forward,
			// 		KeyCode::A => velocity -= right,
			// 		KeyCode::D => velocity += right,
			// 		KeyCode::Space => velocity += Vec3::Y,
			// 		KeyCode::LShift => velocity -= Vec3::Y,
			// 		_ => (),
			// 	},
			// }
			
			match key {
				KeyCode::W => velocity += forward,
				KeyCode::S => velocity -= forward,
				KeyCode::A => velocity -= right,
				KeyCode::D => velocity += right,
				KeyCode::Space => velocity += Vec3::Y,
				KeyCode::LShift => velocity -= Vec3::Y,
				_ => (),
			}
		}

		velocity = velocity.normalize_or_zero();

		transform.translation += velocity * time.delta_seconds() * 10.0;
	}
}

fn move_animations(
	time: Res<Time>,
	mut set: ParamSet<(
		Query<&Transform, With<RenderPlayer>>,
		Query<&mut Transform, With<PlayerHands>>,
	)>
) {
	let transform = {
		let p0 = set.p0();
		let transform = p0.single();
		transform.clone()
	};

	let mut p1 = set.p1();
	let mut transform2 = p1.single_mut();

	transform2.translation.x = transform.translation.x;
	transform2.translation.y = transform.translation.y - 0.4;
	transform2.translation.z = transform.translation.z;

	let rot = transform.rotation;

	transform2.rotation = rot * Quat::from_rotation_y(PI);
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

	commands.spawn((
		SceneBundle {
			scene: asset_server.load("smg_fps_animations.glb#Scene0"),
			transform: Transform::from_xyz(10.0, 1.4, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
		PlayerHands,
	));

	commands.spawn((
        Camera3dBundle {
			transform: Transform::from_xyz(10.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		},
        RenderPlayer(0)
    ));

    // camera
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}
