use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::types::EntityScene;
use crate::types::GameEntity;
use crate::types::PlayerCamera;

pub fn handle_mouse_input(
	mut mouse_events: EventReader<MouseMotion>,
	mut cameras: Query<(&mut Transform, &mut PlayerCamera)>,
) {
	let mut mouse_delta = Vec2::ZERO;
	for mouse_event in mouse_events.iter() {
		mouse_delta += mouse_event.delta;
	}

	for (mut camera, mut player_camera) in cameras.iter_mut() {
		player_camera.pitch = player_camera.pitch - mouse_delta.y;
		player_camera.yaw = player_camera.yaw - mouse_delta.x;

		let rot = Quat::from_euler(
			EulerRot::ZYX, 
			0.0, 
			player_camera.yaw * 0.01, 
			player_camera.pitch * 0.01
		);

		camera.rotation = rot;
	}
}

pub fn handle_move_intent(
	mut commands: Commands,
	mut game_entities: Query<(Entity, &mut GameEntity)>,
	mut set: ParamSet<(
		Query<(&Parent, &EntityScene, &mut Transform)>,
		Query<(&Parent, &PlayerCamera, &mut Transform)>
	)>,
) {
	let entity_camera = {
		match set.p1().get_single() {
			Ok((parent, _, transform)) => {
				Some((parent.get(), transform.clone()))
			},
			Err(_) => {
				None
			},
		}
	};

	for (parent, _, mut transform) in set.p0().iter_mut() {
		let (_, mut game_entity) = match game_entities.get_mut(parent.get()) {
			Ok(game_entity) => game_entity,
			Err(_) => {
				continue;
			},
		};

		let move_intent = &game_entity.move_intent;

		let moving = move_intent.move_forward || 
			move_intent.move_rightward || 
			move_intent.move_backward || 
			move_intent.move_leftward;

		let rot = match (
			move_intent.move_forward, 
			move_intent.move_rightward, 
			move_intent.move_backward, 
			move_intent.move_leftward
		) {
			(true, true, false, false) => {
				Quat::from_rotation_y(45.0_f32.to_radians())
			},
			(false, true, false, false) => {
				Quat::from_rotation_y(90.0_f32.to_radians())
			},
			(false, true, true, false) => {
				Quat::from_rotation_y(135.0_f32.to_radians())
			},
			(false, false, true, false) => {
				Quat::from_rotation_y(180.0_f32.to_radians())
			},
			(false, false, true, true) => {
				Quat::from_rotation_y(225.0_f32.to_radians())
			},
			(false, false, false, true) => {
				Quat::from_rotation_y(270.0_f32.to_radians())
			},
			(true, false, false, true) => {
				Quat::from_rotation_y(315.0_f32.to_radians())
			},
			(true, false, false, false) => {
				Quat::from_rotation_y(0.0_f32.to_radians())
			},
			_ => game_entity.look_at,
		};	

		game_entity.look_at = rot;	
		
		if let Some((camera_parent, camera_transform)) = entity_camera {
			if parent.get() == camera_parent {


				camera_transform.rotation.to_euler(euler)

				rot.rotate_y(camera_transform.rotation.y);
				transform.rotation += Quat::from_rotation_y(camera_transform.rotation.y);
			}
		}

		transform.rotation = rot;
	}


}
