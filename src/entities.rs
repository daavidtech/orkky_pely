use std::sync::mpsc;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::map::MapChange;
use crate::map::MapEntity;
use crate::map::MapTemplate;
use crate::map_loader::MapChangesReceiver;
use crate::types::MapTemplates;
use crate::types::NeedsAsset;
use crate::types::NeedsTemplate;

// fn handle_map_template(
// 	commands: &mut Commands,
// 	template: &MapTemplate,
// 	entity: &MapEntity
// ) {
// 	match &template.asset {
// 		Some(asset) => {
// 			commands.insert(NeedsAsset {
// 				asset: asset.clone(),
// 			});
// 		},
// 		None => {}
// 	}
	
// 	match &entity.camera {
// 		Some(camera_type) => {
// 			commands.with_children(|parent| {
// 				let translation = if camera_type == "fps" {
// 					if let Some(translation) = template.fps_camera_location {
// 						Vec3::from_slice(&translation)
// 					} else {
// 						Vec3::default()
// 					}
// 				} else if camera_type == "third_person" {
// 					if let Some(translation) = template.third_person_camera_location {
// 						Vec3::from(translation)
// 					} else {
// 						Vec3::default()
// 					}
// 				} else {
// 					Vec3::default()
// 				};

// 				parent.spawn(
// 					Camera3dBundle {
// 						transform: Transform {
// 							translation: translation,
// 							..Default::default()
// 						},
// 						..Default::default()
// 					}
// 				);
// 			});
// 		},
// 		_ => {}
// 	}
// }

// fn spaw_map_entity(
// 	commands: &mut Commands,
// 	map_templates: &MapTemplates,
// 	entity: &MapEntity
// ) {
// 	let mut new_component = commands.spawn(
// 		SpatialBundle {
// 			..Default::default()
// 		}
// 	);

// 	match entity.scale {
// 		Some(scale) => {
// 			new_component.insert(
// 				Transform {
// 					scale: Vec3::splat(scale),
// 					..Default::default()
// 				}
// 			);
// 		},
// 		None => {}
// 	}

// 	match map_templates.templates.get(&entity.template) {
// 		Some(template) => {
// 			handle_map_template(&mut new_component.commands(), template, entity);
// 		},
// 		None => {
// 			new_component.insert(NeedsTemplate {
// 				template: entity.template.clone(),
// 			});
// 		}
// 	}
// }

// pub fn handle_needs_template(
// 	commands: &mut Commands,
// 	template_map: Res<MapTemplates>,
// 	query: Query<(Entity, &NeedsTemplate)>
// ) {
// 	for (entity, needs_template) in query.iter() {
// 		match template_map.templates.get(&needs_template.template) {
// 			Some(template) => {
// 				handle_map_template(&mut commands, template, &MapEntity {
// 					template: needs_template.template.clone(),
// 					..Default::default()
// 				});

// 				commands.remove_one::<NeedsTemplate>(entity);
// 			},
// 			None => {}
// 		}
// 	}
// }

// pub fn handle_map_handles(
// 	commands: &mut Commands,
// 	chnages_receiver: Res<MapChangesReceiver>,
// 	mut map_templates: ResMut<MapTemplates>, 
// 	mut done: Local<bool>,
// ) {
// 	if *done {
// 		return;
// 	}

// 	let chnages_receiver = chnages_receiver.rx.lock().unwrap();

// 	let mut changes = vec![];

// 	loop {
// 		match chnages_receiver.try_recv() {
// 			Ok(change) => {
// 				log::info!("mapchange {:?}", change);

// 				match change {
// 					MapChange::NewMapEntity(entity) => {
// 						spaw_map_entity(commands, &map_templates, &entity)
// 					},
//         			MapChange::NewMapTemplate(template) => {
// 						map_templates.templates.insert(template.name.clone(), template);
// 					},
// 				}
// 			},
// 			Err(err) => {
// 				match err {
// 					mpsc::TryRecvError::Empty => {
// 						break;
// 					},
// 					mpsc::TryRecvError::Disconnected => {
// 						log::info!("changes disconnected");
	
// 						*done = true;
	
// 						return;
// 					},
// 				}
// 			}
// 		};
// 	}
	
// 	for change in changes {
// 		log::info!("change {:?}", change);

// 		match change {
// 			MapChange::NewMapEntity(entity) => {
// 				log::info!("add entity {:?}", entity);
// 			},
// 			_ => {}
// 		}
// 	}
// }
