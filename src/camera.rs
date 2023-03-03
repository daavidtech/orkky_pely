use bevy::prelude::*;

use crate::map::CameraType;
use crate::types::*;

// pub fn get_cameara_

pub fn handle_new_cameras(
	mut commands: Commands,
	mut action_queue: ResMut<ActionQueue>,
) {
	for map_camera in action_queue.new_cameras.drain(..) {
		commands.spawn(
			NeedsCamera {
				entity_id: map_camera.entity_id,
				camera_type: map_camera.camera_type
			}
		);
	}
}

pub fn give_camera(
	mut commands: Commands,
	needs_camera: Query<(Entity, &NeedsCamera)>,
	game_entities: Query<(Entity, &GameEntity)>,
	map_templates: ResMut<MapTemplates>, 
) {
	for (needs_cam_entity, needs_camera) in needs_camera.iter() {
		let (entity, game_entity) = match game_entities
			.iter()
			.find(|(_, game_entity)| {
				game_entity.entity_id == needs_camera.entity_id
			}) {
			Some((entity, game_entity)) => {
				(entity, game_entity)
			},
			None => {
				log::error!("could not find game entity for camera");

				continue;
			}
		};

		let template = match map_templates.templates.get(&game_entity.template) {
			Some(template) => {
				template
			},
			None => {
				log::error!("could not find template for camera entity");

				continue;
			}
		};

		log::info!("giving camera to {}", game_entity.entity_id);

		{
			let mut needs_camera_entity_commands = commands.entity(needs_cam_entity);
			needs_camera_entity_commands.remove::<NeedsCamera>();
		}

		let mut entity_commands = commands.entity(entity);

		let translation = match needs_camera.camera_type {
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

		entity_commands.with_children(|parent| {
			let mut entity_commands = parent.spawn((
				TransformBundle::from_transform(
					Transform {
						..Default::default()
					}
				),
				PlayerCamera::default()
			));

			entity_commands.with_children(|parent| {
				parent.spawn((
					Camera3dBundle {
						transform: Transform {
							translation: translation,
							..Default::default()
						},
						..Default::default()
					},
					EntityCamera {
						game_entity_id: game_entity.entity_id.clone(),
					}
				));
			});
		});
	}
}
