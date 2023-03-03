use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::map::*;
use crate::types::*;


fn handle_map_template(
	entity_commands: &mut EntityCommands,
	template: &MapTemplate,
	entity: &MapEntity,
	game_entity: &mut GameEntity,
) {
	game_entity.asset = template.asset.clone();

	match &template.asset {
		Some(asset) => {
			entity_commands.insert(NeedsAsset {
				asset: asset.clone(),
				add_colliding_mesh: template.automatic_collision_mesh.unwrap_or_default(),
				initial_transform: template.initial_transform.clone(),
				initial_rotation_y: template.initial_rotation_y.clone(),
			});
		},
		None => {}
	}

	game_entity.iddle_animation = template.iddle_animation.clone();
	game_entity.walk_animation = template.walk_animation.clone();
	game_entity.run_animation = template.run_animation.clone();
	game_entity.reload_animation = template.reload_animation.clone();
	game_entity.shoot_animation = template.shoot_animation.clone();

	game_entity.weapons = template.weapons.clone();

	match &template.collider {
		Some(collider) => {
			match collider {
				MapEntityCollider::AABB => {

				},
				MapEntityCollider::Capsule { a, b, radius } => {
					log::info!("spawning capsule collider: {:?} {:?} {:?}", a, b, radius);

					entity_commands.insert((
						RigidBody::Dynamic,
						Collider::capsule(Vec3::Y * *a, Vec3::Y * *b, *radius)
					));
				},
				MapEntityCollider::Cuboid { x, y, z } => {
					log::info!("spawning cuboid collider: {:?} {:?} {:?}", x, y, z);

					let half_x = *x / 2.0;
					let half_y = *y / 2.0;
					let half_z = *z / 2.0;

					entity_commands.insert((
						Restitution::coefficient(0.0),
						Collider::cuboid(*x, *y, *z)
					));
				},
				_ => {}
			}
		},
		None => {},
	}

	match &template.physics {
		Some(physics) => {
			match physics {
				MapEntityPhysics::Dynamic => {
					entity_commands.insert(RigidBody::Dynamic);
				},
				MapEntityPhysics::Static => {
					entity_commands.insert(RigidBody::Fixed);
				}
			}
		},
		None => {}
	}

	if let Some(mass) = template.mass {
		entity_commands.insert(AdditionalMassProperties::Mass(mass));
	}
}

pub fn handle_needs_template(
	mut commands: Commands,
	template_map: Res<MapTemplates>,
	mut query: Query<(Entity, &NeedsTemplate, &mut GameEntity)>,
) {
	for (entity, needs_template, mut game_entity) in query.iter_mut() {
		match template_map.templates.get(&needs_template.template) {
			Some(template) => {
				let mut entity_commands = commands.entity(entity);
				
				handle_map_template(&mut entity_commands, template, &needs_template.map_enitity, &mut game_entity);

				entity_commands.remove::<NeedsTemplate>();
			},
			None => {}
		}
	}
}

pub fn handle_update_map_templates(
	mut action_queue: ResMut<ActionQueue>,
	mut map_templates: ResMut<MapTemplates>,
	current_camera: Option<Res<CurrentCamera>>,
	template_entities: ResMut<TemplateEntities>,
	mut query: Query<(&Camera3d, &EntityCamera, &mut Transform)>
) {
	for map_template in action_queue.update_templates.drain(..) {
		let old_map_template = match map_templates.templates.get(&map_template.name) {
			Some(old_map_template) => old_map_template.clone(),
			None => continue
		};

		let entities = match template_entities.entities.get(&map_template.name) {
			Some(entities) => entities.clone(),
			None => continue
		};

		for entity_id in entities {
			match map_templates.templates.get(&map_template.name) {
				Some(template) => {
					if map_template.third_person_camera_location != old_map_template.third_person_camera_location {
						for (camera, entity_camera, mut transform) in query.iter_mut() {
							if entity_camera.game_entity_id == entity_id {
								let current_camera = current_camera.unwrap();

								transform.translation = match current_camera.camera_type {
									Some(CameraType::FPS) => match template.fps_camera_location {
										Some(translation) => {
											Vec3::from_slice(&translation)
										},
										None => {
											Vec3::default()
										}
									},
									Some(CameraType::ThirdPerson) => match template.third_person_camera_location {
										Some(translation) => {
											Vec3::from_slice(&translation)
										},
										None => {
											Vec3::default()
										}
									},
									None => Vec3::default(),
								};
							}
						}
					}
				},
				None => {}
			}
		}
	}
}
