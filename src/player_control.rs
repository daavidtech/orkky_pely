use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::math::compute_new_yaw;
use crate::math::rotate_vec;
use crate::types::GameEntity;
use crate::types::PlayerCamera;
use crate::types::You;

pub fn handle_mouse_input(
	mut mouse_events: EventReader<MouseMotion>,
	mut set: ParamSet<(
		Query<(&mut Transform, &mut GameEntity, &You)>,
		Query<(&mut Transform, &mut PlayerCamera)>
	)>,
	//mut game_entities: Query<(&GameEntity, &mut Transform, &You)>,
	// mut cameras: Query<(&mut Transform, &mut PlayerCamera)>,
	mut yaw_changed: Local<f32>,
	mut pitch_changed: Local<f32>,
) {
	let mut mouse_delta = Vec2::ZERO;
	for mouse_event in mouse_events.iter() {
		mouse_delta += mouse_event.delta;
	}

	println!("mouse_delta: {:?}", mouse_delta);

	// if *yaw_changed != mouse_delta.x {
	// 	log::info!("yaw: {}", mouse_delta.);
	// 	*yaw_changed = mouse_delta.x;
	// }



	// for (game_entity, mut transform, _) in game_entities.iter_mut() {
	// 	transform.rotate_y(-mouse_delta.x * 0.01);
	// }

	// {
	// 	let mut q = set.p0();
	// 	let (mut transform, mut game_entity, _) = match q.get_single_mut() {
	// 		Ok(g) => g,
	// 		Err(_) => {
	// 			println!("no game entity found");
	// 			return;
	// 		},
	// 	};

	// 	transform.rotate_y(-mouse_delta.x * 0.01);

	// 	game_entity.yaw = compute_new_yaw(
	// 		game_entity.yaw, 
	// 		mouse_delta.x, 
	// 		0.01
	// 	);

	// 	// game_entity.yaw = ((game_entity.yaw - (mouse_delta.x * 0.01).to_degrees()).abs()) % 360.0;

	// 	//println!("delta x {}", game_entity.yaw);

	// 	// game_entity.yaw = ().to_degrees() % 360.0;

	// 	if *yaw_changed != game_entity.yaw {
	// 		log::info!("yaw: {}", game_entity.yaw);
	// 		*yaw_changed = game_entity.yaw;
	// 	}
	// }

	// {
	// 	let mut q = set.p1();
	// 	let (mut transform, _) = match q.get_single_mut() {
	// 		Ok(g) => g,
	// 		Err(_) => {
	// 			println!("no game entity found");
	// 			return;
	// 		},
	// 	};

	// 	transform.rotate_x(-mouse_delta.y * 0.01);
	// }

	// 	println!("mouse_delta: {:?}", mouse_delta);

	// 	transform.rotate_y(mouse_delta.y * 0.01);
	// }

	// for (mut camera, mut player_camera) in cameras.iter_mut() {
	// 	player_camera.pitch = player_camera.pitch - mouse_delta.y;
	// 	player_camera.yaw = player_camera.yaw - mouse_delta.x;

	// 	// if *pitch_changed != player_camera.pitch {
	// 	// 	log::info!("pitch: {}", player_camera.pitch);
	// 	// 	*pitch_changed = player_camera.pitch;
	// 	// }

	// 	// if *yaw_changed != player_camera.yaw {
	// 	// 	log::info!("yaw: {}", player_camera.yaw);
	// 	// 	*yaw_changed = player_camera.yaw;
	// 	// }

	// 	let rot = Quat::from_euler(
	// 		EulerRot::ZYX, 
	// 		0.0, 
	// 		player_camera.yaw * 0.01, 
	// 		player_camera.pitch * 0.01
	// 	);

	// 	camera.rotation = rot;
	// }
}

pub fn rotate_asset(
	mut commands: Commands,
	// mut game_entities: Query<(Entity, &mut GameEntity, &mut Transform, &You)>,
	mut set: ParamSet<(
		Query<(Entity, &mut GameEntity, &mut Transform, &You)>,
		Query<(&Parent, &PlayerCamera, &mut Transform)>
	)>,
	mut last_angle: Local<f32>,
	mut last_camera_rot: Local<f32>
) {
	// let game_entity = {
	// 	let mut game_entities = set.p0();
	// 	let (_, mut game_entity, _, _) = match game_entities.get_single_mut() {
	// 		Ok(game_entity) => game_entity,
	// 		Err(_) => return,
	// 	};

	// 	game_entity.clone()
	// };

	// let move_intent = &game_entity.move_intent;

	// let moving = move_intent.move_forward || 
	// 	move_intent.move_rightward || 
	// 	move_intent.move_backward || 
	// 	move_intent.move_leftward;

	// let mut angle = match (
	// 	move_intent.move_forward, 
	// 	move_intent.move_rightward, 
	// 	move_intent.move_backward, 
	// 	move_intent.move_leftward
	// ) {
	// 	(true, true, false, false) => 45.0_f32,
	// 	(true, false, false, true) => 135.0_f32,
	// 	(false, false, true, true) => 225.0_f32,
	// 	(false, true, true, false) => 315.0_f32,
	// 	(true, false, false, false) => 0.0_f32,
	// 	(false, false, true, false) => 180.0_f32,
	// 	(false, true, false, false) => 270.0_f32,
	// 	(false, false, false, true) => 90.0_f32,
	// 	_ => 0.0_f32,
	// }.to_radians();

	// if moving {
		// let (player_camera, camera_transform) = {
		// 	let q = set.p1();
		// 	match q.get_single() {
		// 		Ok((_, player_gamera, camera_transform)) => (
		// 			player_gamera.clone(), 
		// 			camera_transform.clone()
		// 		),
		// 		Err(_) => return,
		// 	}
		// };

		// let (v, _) = camera_transform.rotation.to_axis_angle();

		// if *last_camera_rot != v.y {
		// 	log::info!("camera rot: {}", v.y);
		// 	*last_camera_rot = v.y;
		// }

		// let mut q = set.p0();
		// let mut game_entity_transform = match q.get_single_mut() {
		// 	Ok((_, _, transform, _)) => transform,
		// 	Err(_) => return,
		// };

		// angle += player_camera.yaw.to_radians();

		// // if *last_angle != angle {
		// // 	log::info!("angle: {}", angle);
		// // 	*last_angle = angle;
		// // }

		// let rot = Quat::from_euler(
		// 	EulerRot::ZYX, 
		// 	0.0, 
		// 	angle, 
		// 	0.0
		// );

		// game_entity_transform.rotation = rot;

		//for (_, game_entity, mut transform, _) in set.p0().iter_mut() {
			// let move_intent = &game_entity.move_intent;

			// let mut angle = match (
			// 	move_intent.move_forward, 
			// 	move_intent.move_rightward, 
			// 	move_intent.move_backward, 
			// 	move_intent.move_leftward
			// ) {
			// 	(true, true, false, false) => 45.0_f32,
			// 	(true, false, false, true) => 135.0_f32,
			// 	(false, false, true, true) => 225.0_f32,
			// 	(false, true, true, false) => 315.0_f32,
			// 	(true, false, false, false) => 0.0_f32,
			// 	(false, false, true, false) => 180.0_f32,
			// 	(false, true, false, false) => 270.0_f32,
			// 	(false, false, false, true) => 90.0_f32,
			// 	_ => 0.0_f32,
			// };

			// if move_intent.move_forward {
			// 	
			// }


		//}
	// } else {
		// for (_, player_camera, mut camera_transform) in set.p1().iter_mut() {
		// 	let rot = Quat::from_euler(
		// 		EulerRot::ZYX, 
		// 		0.0, 
		// 		player_camera.yaw * 0.01, 
		// 		player_camera.pitch * 0.01
		// 	);

		// 	camera_transform.rotation = rot;
		// }
	//}

	// let player_camera = {
	// 	let cameras = set.p1();
	// 	match cameras.get_single() {
	// 		Ok((_, player_camera, _)) => player_camera.clone(),
	// 		Err(_) => return,
	// 	}
	// };

	// {




	// 	let mut rot = match (
	// 		move_intent.move_forward, 
	// 		move_intent.move_rightward, 
	// 		move_intent.move_backward, 
	// 		move_intent.move_leftward
	// 	) {
	// 		(true, true, false, false) => {
	// 			45.0_f32
	// 		},
	// 		(false, true, false, false) => {
	// 			90.0_f32
	// 		},
	// 		(false, true, true, false) => {
	// 			135.0_f32
	// 		},
	// 		(false, false, true, false) => {
	// 			180.0_f32
	// 		},
	// 		(false, false, true, true) => {
	// 			225.0_f32
	// 		},
	// 		(false, false, false, true) => {
	// 			270.0_f32
	// 		},
	// 		(true, false, false, true) => {
	// 			315.0_f32
	// 		},
	// 		(true, false, false, false) => {
	// 			0.0_f32
	// 		},
	// 		_ => {
	// 			*last_angle
	// 		},
	// 	};

	// 	if *last_angle != rot {
	// 		log::info!("rot: {}", rot);
	// 		*last_angle = rot;
	// 	}

	// 	let cam_rot = Quat::from_euler(
	// 		EulerRot::ZYX, 
	// 		0.0, 
	// 		player_camera.yaw * 0.01, 
	// 		0.0
	// 	);

	// 	//if moving {
	// 		game_entity.yaw = rot;
	// 		//game_entity.look_at = Quat::from_rotation_y(rot.to_radians());
	// 		// transform.rotation = Quat::from_rotation_y(rot.to_radians());

	// 		transform.rotation = cam_rot;
	// 	//}	
	// }

	// for (_, game_entity, transform) in game_entities.iter_mut() {

	// }

	// let entity_camera = {
	// 	match set.p1().get_single() {
	// 		Ok((parent, _, transform)) => {
	// 			Some((parent.get(), transform.clone()))
	// 		},
	// 		Err(_) => {
	// 			None
	// 		},
	// 	}
	// };

	// for (parent, _, mut transform) in set.p0().iter_mut() {
	// 	let (_, mut game_entity) = match game_entities.get_mut(parent.get()) {
	// 		Ok(game_entity) => game_entity,
	// 		Err(_) => {
	// 			continue;
	// 		},
	// 	};

	// 	let move_intent = &game_entity.move_intent;

	// 	let moving = move_intent.move_forward || 
	// 		move_intent.move_rightward || 
	// 		move_intent.move_backward || 
	// 		move_intent.move_leftward;

	// 	let mut rot = match (
	// 		move_intent.move_forward, 
	// 		move_intent.move_rightward, 
	// 		move_intent.move_backward, 
	// 		move_intent.move_leftward
	// 	) {
	// 		(true, true, false, false) => {
	// 			Quat::from_rotation_y(45.0_f32.to_radians())
	// 		},
	// 		(false, true, false, false) => {
	// 			Quat::from_rotation_y(90.0_f32.to_radians())
	// 		},
	// 		(false, true, true, false) => {
	// 			Quat::from_rotation_y(135.0_f32.to_radians())
	// 		},
	// 		(false, false, true, false) => {
	// 			Quat::from_rotation_y(180.0_f32.to_radians())
	// 		},
	// 		(false, false, true, true) => {
	// 			Quat::from_rotation_y(225.0_f32.to_radians())
	// 		},
	// 		(false, false, false, true) => {
	// 			Quat::from_rotation_y(270.0_f32.to_radians())
	// 		},
	// 		(true, false, false, true) => {
	// 			Quat::from_rotation_y(315.0_f32.to_radians())
	// 		},
	// 		(true, false, false, false) => {
	// 			Quat::from_rotation_y(0.0_f32.to_radians())
	// 		},
	// 		_ => {
	// 			game_entity.look_at
	// 		},
	// 	};	
		
	// 	if moving {
	// 		if let Some((camera_parent, camera_transform)) = entity_camera {
	// 			if parent.get() == camera_parent {
	// 				let mut camera_rot = camera_transform.rotation;

	// 				camera_rot.x = 0.0;
	// 				camera_rot.z = 0.0;

	// 				rot *= camera_rot;
	// 			}
	// 		}
	// 	}

	// 	game_entity.look_at = rot;
	// 	transform.rotation = rot;
	// }


}

pub fn move_asset(
	mut query: Query<(&mut Transform, &GameEntity)>,
	mut last_x: Local<f32>,
	mut last_y: Local<f32>,
) {
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
					(0.5, 0.5)
				},
				(false, true, false, false) => {
					(1.0, 0.0)
				},
				(false, true, true, false) => {
					(0.5, -0.5)
				},
				(false, false, true, false) => {
					(0.0, -1.0)
				},
				(false, false, true, true) => {
					(-0.5, -0.5)
				},
				(false, false, false, true) => {
					(-1.0, 0.0)
				},
				(true, false, false, true) => {
					(-0.5, 0.5)
				},
				(true, false, false, false) => {
					(0.0, 1.0)
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

			// if move_intent.move_forward {
			// 	move_dir += Vec3::new(0.0, 0.0, -1.0);
			// } else if move_intent.move_rightward {
			// 	move_dir += Vec3::new(1.0, 0.0, 0.0);
			// } else if move_intent.move_backward {
			// 	move_dir += Vec3::new(0.0, 0.0, 1.0);
			// } else if move_intent.move_leftward {
			// 	move_dir += Vec3::new(-1.0, 0.0, 0.0);
			// }

			// let (x, y) = rotate_vec(
			// 	move_dir.x, 
			// 	move_dir.z, 
			// 	-game_entity.yaw
			// );

			if *last_x != x {
				log::info!("x: {}", x);
				*last_x = x;
			}

			if *last_y != y {
				log::info!("y: {}", y);
				*last_y = y;
			}

			// move_dir.x = x;
			// move_dir.z = y;

			// move_dir = move_dir.normalize();

			transform.translation.x += x * 0.1;
			transform.translation.z += y * 0.1;
		}
	}
}
