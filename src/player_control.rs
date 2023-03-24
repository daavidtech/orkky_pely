use std::f32::consts::PI;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::console_plugin::Console;
use crate::math::compute_new_angle;
use crate::math::rotate_vec;
use crate::types::GameEntity;
use crate::types::PlayerCamera;
use crate::types::You;

pub fn handle_mouse_input(
	mut mouse_events: EventReader<MouseMotion>,
	console: Res<Console>,
	mut set: ParamSet<(
		Query<(&mut Transform, &mut GameEntity, &You)>,
		Query<(&mut Transform, &mut PlayerCamera)>
	)>,
	mut yaw_changed: Local<f32>,
	mut pitch_changed: Local<f32>,
) {
	if console.active {
		return;
	}

	let mut mouse_delta = Vec2::ZERO;
	for mouse_event in mouse_events.iter() {
		mouse_delta += mouse_event.delta;
	}

	let pitch = {
		let mut q = set.p0();
		let (mut transform, mut game_entity, _) = match q.get_single_mut() {
			Ok(g) => g,
			Err(_) => {
				// println!("no game entity found");
				return;
			},
		};

		game_entity.yaw = compute_new_angle(
			game_entity.yaw, 
			mouse_delta.x, 
			0.01
		);
		game_entity.pitch = compute_new_angle(
			game_entity.pitch, 
			-mouse_delta.y, 
			0.01
		);

		// if *yaw_changed != game_entity.yaw {
		// 	log::info!("yaw: {}", game_entity.yaw);
		// 	*yaw_changed = game_entity.yaw;
		// }

		// if *pitch_changed != game_entity.pitch {
		// 	log::info!("pitch: {}", game_entity.pitch);
		// 	*pitch_changed = game_entity.pitch;
		// }

		transform.rotation = Quat::from_rotation_y(2.0*PI - game_entity.yaw);

		game_entity.pitch
	};

	{
		let mut q = set.p1();
		let (mut transform, _) = match q.get_single_mut() {
			Ok(g) => g,
			Err(_) => {
				// println!("no game entity found");
				return;
			},
		};

		transform.rotation = Quat::from_rotation_x(pitch);
	}
}

pub fn move_game_entity(
	mut query: Query<(&mut Transform, &GameEntity)>,
	console: Res<Console>,
) {
	if console.active {
		return;
	}

	for (mut transform, game_entity) in query.iter_mut() {
		let move_intent = &game_entity.move_intent;

		let moving = move_intent.move_forward || 
			move_intent.move_rightward || 
			move_intent.move_backward || 
			move_intent.move_leftward;

		if moving {
			let mut move_dir  = Vec3::ZERO;

			let (x, y) = match (
				move_intent.move_forward, 
				move_intent.move_rightward, 
				move_intent.move_backward, 
				move_intent.move_leftward
			) {
				(true, true, false, false) => {
					(0.5, -0.5)
				},
				(false, true, false, false) => {
					(1.0, 0.0)
				},
				(false, true, true, false) => {
					(0.5, 0.5)
				},
				(false, false, true, false) => {
					(0.0, 1.0)
				},
				(false, false, true, true) => {
					(-0.5, 0.5)
				},
				(false, false, false, true) => {
					(-1.0, 0.0)
				},
				(true, false, false, true) => {
					(-0.5, -0.5)
				},
				(true, false, false, false) => {
					(0.0, -1.0)
				},
				_ => {
					continue;
				},
			};

			let (x, y) = rotate_vec(
				x, 
				y, 
				game_entity.yaw
			);

			let speed = match game_entity.running {
				true => 0.3,
				false => 0.1,
			};

			transform.translation.x += x * speed;
			transform.translation.z += y * speed;
		}
	}
}
