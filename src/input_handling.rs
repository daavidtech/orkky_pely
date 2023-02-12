use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

use crate::map::WeaponType;
use crate::types::GameEntity;
use crate::types::MeleeHitbox;
use crate::types::StartAnimation;
use crate::types::StopAnimation;
use crate::types::You;

pub fn keyboard_handler(
	keyboard_input: Res<Input<KeyCode>>,
	query: Query<(&You)>,
) {
	// match keyboard_input.get_pressed() {
	// 	KeyCode::W => {

	// 	}
	// 	_ => {}
	// }
	// for _ in query.iter() {
	// 	// do something
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
