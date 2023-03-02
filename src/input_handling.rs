use bevy::prelude::*;

use crate::map::WeaponType;
use crate::types::GameEntity;
use crate::types::MeleeHitbox;
use crate::types::StartAnimation;
use crate::types::StopAnimation;
use crate::types::You;

pub fn keyboard_handler(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(Entity, &mut GameEntity, &mut Transform, &You)>
) {
	let (_, mut game_entity, _, _) = match query.get_single_mut() {
		Ok(q) => q,
		Err(_) => {
			return;
		},
	};

	let mut just_move_backward = false;
	let mut just_move_rightward = false;
	let mut just_move_leftward = false;
	let mut just_move_forward = false;

	for key in keyboard_input.get_just_pressed() {
		match key {
			KeyCode::W => {
				game_entity.move_intent.move_forward = true;
				just_move_forward = true;
			},
			KeyCode::A => {
				game_entity.move_intent.move_leftward = true;
				just_move_leftward = true;
			},
			KeyCode::S => {
				game_entity.move_intent.move_backward = true;
				just_move_backward = true;
			},
			KeyCode::D => {
				game_entity.move_intent.move_rightward = true;
				just_move_rightward = true;
			},
			_ => {}
		}
	}

	for key in keyboard_input.get_just_released() {
		match key {
			KeyCode::W => {
				game_entity.move_intent.move_forward = false;
				just_move_forward = false;
			},
			KeyCode::A => {
				game_entity.move_intent.move_leftward = false;
				just_move_leftward = false;
			},
			KeyCode::S => {
				game_entity.move_intent.move_backward = false;
				just_move_backward = false;
			},
			KeyCode::D => {
				game_entity.move_intent.move_rightward = false;
				just_move_rightward = false;
			},
			_ => {}
		}
	}



	// let mut just_move_backward = false;
	// let mut just_move_rightward = false;
	// let mut just_move_leftward = false;
	// let mut just_move_forward = false;
	
	// {
	// 	let mut game_entities = set.p0();
	// 	let (_, mut game_entity, _, _) = match game_entities.get_single_mut() {
	// 		Ok(game_entity) => game_entity,
	// 		Err(_) => {
	// 			return;
	// 		},
	// 	};

	// 	for key in keyboard_input.get_just_pressed() {
	// 		match key {
	// 			KeyCode::W => {
	// 				game_entity.move_intent.move_forward = true;
	// 				just_move_forward = true;
	// 			},
	// 			KeyCode::A => {
	// 				game_entity.move_intent.move_leftward = true;
	// 				just_move_leftward = true;
	// 			},
	// 			KeyCode::S => {
	// 				game_entity.move_intent.move_backward = true;
	// 				just_move_backward = true;
	// 			},
	// 			KeyCode::D => {
	// 				game_entity.move_intent.move_rightward = true;
	// 				just_move_rightward = true;
	// 			},
	// 			_ => {}
	// 		}
	// 	}
	
	// 	for key in keyboard_input.get_just_released() {
	// 		match key {
	// 			KeyCode::W => {
	// 				game_entity.move_intent.move_forward = false;
	// 				just_move_forward = false;
	// 			},
	// 			KeyCode::A => {
	// 				game_entity.move_intent.move_leftward = false;
	// 				just_move_leftward = false;
	// 			},
	// 			KeyCode::S => {
	// 				game_entity.move_intent.move_backward = false;
	// 				just_move_backward = false;
	// 			},
	// 			KeyCode::D => {
	// 				game_entity.move_intent.move_rightward = false;
	// 				just_move_rightward = false;
	// 			},
	// 			_ => {}
	// 		}
	// 	}
	// }

	// if !just_move_forward && !just_move_backward && !just_move_leftward && !just_move_rightward {
	// 	return;
	// }

	// let angle = match (
	// 	just_move_forward,
	// 	just_move_backward,
	// 	just_move_leftward,
	// 	just_move_rightward,
	// ) {
	// 	(true, false, false, false) => 0.0_f32,
	// 	(false, true, false, false) => 180.0_f32,
	// 	(false, false, true, false) => 270.0_f32,
	// 	(false, false, false, true) => 90.0_f32,
	// 	(true, false, true, false) => 315.0_f32,
	// 	(true, false, false, true) => 45.0_f32,
	// 	(false, true, true, false) => 225.0_f32,
	// 	(false, true, false, true) => 135.0_f32,
	// 	_ => 0.0_f32,
	// };

	// let mut angle = angle.to_radians();

	// let camera_rotation = {
	// 	let mut cameras = set.p1();
	// 	let (player_camera, mut camera) = match cameras.get_single_mut() {
	// 		Ok(q) => q,
	// 		Err(_) => return,
	// 	};

	// 	camera.rotate_y(angle);

	// 	let camera_angle = camera.rotation.to_axis_angle().0;

	// 	println!("y angle {}", camera_angle.y.to_degrees());
	// 	println!("x angle {}", camera_angle.x.to_degrees());
	// 	println!("z angle {}", camera_angle.z.to_degrees());

	// 	// angle += camera_angle.y;

	// 	camera.rotation.clone()
	// };

	// {
	// 	let mut game_entities = set.p0();
	// 	let (_, _, mut transform, _) = match game_entities.get_single_mut() {
	// 		Ok(game_entity) => game_entity,
	// 		Err(_) => {
	// 			return;
	// 		},
	// 	};

	// 	transform.rotate_y(-angle);

	// 	//transform.rotate(camera_rotation);
	// }

	// let (camera_entity, mut camera) = match cameras.get_single_mut() {
	// 	Ok((camera_entity, camera)) => (camera_entity, camera),
	// 	Err(_) => return,
	// };

	// let (entity, mut game_entity) = match game_entities.get_single_mut() {
	// 	Ok(game_entity) => game_entity,
	// 	Err(_) => {
	// 		return;
	// 	},
	// };





	// {
	// 	let mut entity_commands = commands.entity(entity);
	// 	entity_commands.insert(
	// 		RotateThing {
	// 			y: -angle,
	// 		}
	// 	);
	// }

	// {
	// 	let mut camera_entity_commands = commands.entity(camera_entity);
	// 	camera_entity_commands.insert(
	// 		RotateThing {
	// 			y: angle,
	// 		}
	// 	);
	// }


}


pub fn mouse_handlers(
	mut commands: Commands,
	mouse_input: Res<Input<MouseButton>>,
	query: Query<(Entity, &You, &GameEntity)>,
) {
	let pressed = mouse_input.get_just_pressed();

	let (entity, game_entity) = match query.get_single() {
		Ok((entity, _, game_entity)) => (entity, game_entity),
		Err(_) => return,
	};

	let mut entity_commands = commands.entity(entity);

	for p in pressed {
		if let MouseButton::Left = p {
				log::info!("left mouse button pressed");

				// entity_commands.insert(StartAttack);

				entity_commands.commands().spawn(
					TransformBundle::from_transform(
						Transform {
							translation: Vec3::new(0.0, 5.0, 0.0),
							..Default::default()
						},
					)
				);

				entity_commands.with_children(|parent| {
					parent.spawn((
						MeleeHitbox {
							delay: 0.6,
							dur: 1.0,
							radius: 4.0,
							start_angle: 310.0,
							end_angle: 130.0,
						},
						TransformBundle::from_transform(
							Transform {
								translation: Vec3::new(0.0, 1.5, 0.0),
								..Default::default()
							},
						)
					));					
				});

				match game_entity.weapons.get(game_entity.current_weapon) {
					Some(weapon) => {
						match weapon.weapon_type {
							WeaponType::Melee => {
								log::info!("melee weapon");
							},
							WeaponType::Ranged => {
								log::info!("ranged weapon");
							},
						}

						match (&game_entity.asset, &weapon.animation) {
							(Some(asset), Some(animation)) => {
								log::info!("asset: {}", asset);
								log::info!("animation: {}", animation);

								entity_commands.insert(
									StartAnimation {
										asset: asset.clone(),
										animation: animation.clone(),
										repeat: true,
										..Default::default()
									}
								);
							},
							_ => {}
						}
					},
					None => {
						log::info!("no weapon equipped");
					}
				};
			}
	}

	let released = mouse_input.get_just_released();

	for r in released {
		match r {
			MouseButton::Left => {
				log::info!("left mouse button released");

				entity_commands.insert(StopAnimation);
			},
			_ => {}
		}
	}
}
