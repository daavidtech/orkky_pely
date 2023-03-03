use std::sync::mpsc;

use bevy::gltf::*;
use bevy::prelude::*;
use bevy::prelude::AmbientLight;
use bevy_rapier3d::prelude::*;

use crate::light::spawn_map_light;
use crate::map::*;
use crate::map_loader::MapChangesReceiver;
use crate::shape::spawn_shape;
use crate::types::*;

pub fn handle_map_changes(
	mut commands: Commands,
	changes_receiver: Res<MapChangesReceiver>,
	mut map_templates: ResMut<MapTemplates>, 
	mut gltf_register: ResMut<GltfRegister>,
	mut done: Local<bool>,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut player_ids: ResMut<PlayerIds>,
	mut template_entities: ResMut<TemplateEntities>,
	mut action_queue: ResMut<ActionQueue>,
	query: Query<Entity, With<Camera3d>>
) {
	// if *done {
	// 	return;
	// }

	let changes_receiver = changes_receiver.rx.lock().unwrap();

	loop {
		match changes_receiver.try_recv() {
			Ok(change) => {
				log::info!("mapchange {:?}", change);

				match change {
					MapChange::NewMapEntity(entity) => {
						log::info!("Spawning map entity: {}", entity.template);

						let game_entity = GameEntity {
							entity_id: entity.entity_id.clone(),
							template: entity.template.clone(),
							..Default::default()
						};
					
						template_entities.add_entity(
							&entity.template, 
							&game_entity.entity_id);
					
						let mut new_component = commands.spawn((
							SpatialBundle {
								..Default::default()
							},
							game_entity
						));
					
						let scale = match entity.scale {
							Some(scale) => Vec3::splat(scale),
							None => Vec3::splat(1.0)
						};
					
						let translation = match entity.initial_position {
							Some(translation) => Vec3::from_slice(&translation),
							None => Vec3::default()
						};
					
						let entity_transform = Transform {
							scale: scale,
							translation: translation,
							..Default::default()
						};
					
						new_component.insert(entity_transform.clone());
					
						new_component.insert(
							NeedsTemplate {
								template: entity.template.clone(),
								map_enitity: entity.clone()
							}
						);
					
						if let Some(true) = entity.player {
							let player_id = player_ids.provide_player_id(&entity.entity_id);
					
							log::info!("[{}] entity is player {}", entity.entity_id, player_id);
					
							new_component.insert((
								You,
								KinematicCharacterController {
									snap_to_ground: Some(
										CharacterLength::Absolute(0.5)
									),
									..Default::default()
								},
								Collider::ball(0.5)
							));
						}
					},
        			MapChange::NewMapTemplate(template) => {
						match &template.asset {
							Some(asset_path) => {
								let asset: Handle<Gltf> = asset_server.load(asset_path);
				
								let unloaded_asset = UnloadedGltfAsset {
									asset: asset_path.clone(),
									gltf: asset
								};
				
								gltf_register.unloaded.push(unloaded_asset);
							},
							None => todo!(),
						}

						if let Some(prev_template) = map_templates.templates.get(&template.name) {

						}
				
						map_templates.templates.insert(template.name.clone(), template);
					},
					MapChange::NewMapShape(shape) => {
						spawn_shape(&mut commands, &mut meshes, &mut materials, shape);
					},
					MapChange::NewLight(light) => {
						spawn_map_light(&mut commands, light);
					},
					MapChange::NewAmbientLight(args) => {
						commands.insert_resource(AmbientLight {
							brightness: args.brightness,
							color: Color::hex(args.color).unwrap(),
						});
					},
        			MapChange::NewCamera(map_camera) => {
						commands.insert_resource(
							CurrentCamera {
								camera_type: map_camera.camera_type.clone(),
								entity_id: map_camera.entity_id.clone(),
							}
						);
						action_queue.new_cameras.push(map_camera);
					},
					MapChange::UpdateMapEntity(entity) => { {
						action_queue.update_entities.push(entity);
					}},
					MapChange::RemoveMapEntity(entity_id) => {
						action_queue.remove_entities.push(entity_id);
					},
					MapChange::UpdateMaptemplate(template) => {
						action_queue.update_templates.push(template);
					},
					MapChange::RemoveMapTemplate(template) => {
						action_queue.remove_templates.push(template);
					},
					MapChange::UpdateLight(_) => {},
					MapChange::UpdateAmbientLight(args) => {
						commands.insert_resource(AmbientLight {
							brightness: args.brightness,
							color: Color::hex(args.color).unwrap(),
						});
					},
					MapChange::UpdateMapShape(shape) => {
						action_queue.update_shapes.push(shape);
					},
					MapChange::RemoveMapShape(shape_id) => {
						action_queue.remove_shapes.push(shape_id);
					},
					MapChange::RemoveLight(_) => {},
					MapChange::RemoveAmbientLight => {
						commands.remove_resource::<AmbientLight>();
					},
					MapChange::UpdateCamera(camera) => {
						action_queue.update_cameras.push(camera);
					},
					MapChange::RemoveCamera => {
						for entity in query.iter() {
							commands.entity(entity)
								.despawn();
						}
					},			
				}
			},
			Err(err) => {
				match err {
					mpsc::TryRecvError::Empty => {
						break;
					},
					mpsc::TryRecvError::Disconnected => {
						log::info!("changes disconnected");
	
						*done = true;
	
						return;
					},
				}
			}
		};
	}
}
