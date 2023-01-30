use std::sync::mpsc;

use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::map::MapEntityCollider;
use crate::map::MapChange;
use crate::map::MapEntity;
use crate::map::MapTemplate;
use crate::map_loader::MapChangesReceiver;
use crate::types::AssetPacks;
use crate::types::GltfRegister;
use crate::types::MapTemplates;
use crate::types::NeedsAsset;
use crate::types::NeedsTemplate;
use crate::types::UnloadedGltfAsset;

fn handle_map_template(
	commands: &mut EntityCommands,
	template: &MapTemplate,
	entity: &MapEntity
) {
	match &template.asset {
		Some(asset) => {
			commands.insert(NeedsAsset {
				asset: asset.clone(),
			});
		},
		None => {}
	}
	
	match &entity.camera {
		Some(camera_type) => {
			commands.with_children(|parent| {
				let translation = if camera_type == "fps" {
					if let Some(translation) = template.fps_camera_location {
						Vec3::from_slice(&translation)
					} else {
						Vec3::default()
					}
				} else if camera_type == "third_person" {
					if let Some(translation) = template.third_person_camera_location {
						Vec3::from(translation)
					} else {
						Vec3::default()
					}
				} else {
					Vec3::default()
				};

				parent.spawn(
					Camera3dBundle {
						transform: Transform {
							translation: translation,
							..Default::default()
						},
						..Default::default()
					}
				);
			});
		},
		_ => {}
	}

	// Collider::cuboid(hx, hy, hz)

	match &template.collider {
		Some(collider) => {
			match collider {
				MapEntityCollider::AABB => {

				},
				MapEntityCollider::Capsule { a, b, radius } => {
					log::info!("spawning capsule collider: {:?} {:?} {:?}", a, b, radius);

					commands.insert((
						RigidBody::Dynamic,
						AdditionalMassProperties::Mass(1.0),
						Collider::capsule(Vec3::Y * *a, Vec3::Y * *b, *radius)
					));
				},
				MapEntityCollider::Cuboid { x, y, z } => {
					log::info!("spawning cuboid collider: {:?} {:?} {:?}", x, y, z);

					commands.insert((
						RigidBody::Dynamic,
						AdditionalMassProperties::Mass(1.0),
						Collider::cuboid(*x, *y, *z)
					));
				},
				_ => {}
			}
		},
		None => {},
	}
}

fn spaw_map_entity(
	commands: &mut Commands,
	map_templates: &MapTemplates,
	entity: &MapEntity
) {
	log::info!("Spawning map entity: {}", entity.template);

	let mut new_component = commands.spawn(
		SpatialBundle {
			..Default::default()
		}
	);

	let scale = match entity.scale {
		Some(scale) => Vec3::splat(scale),
		None => Vec3::splat(1.0)
	};

	let translation = match entity.initial_position {
		Some(translation) => Vec3::from_slice(&translation),
		None => Vec3::default()
	};

	new_component.insert(
		Transform {
			scale: scale,
			translation: translation,
			..Default::default()
		}
	);

	match map_templates.templates.get(&entity.template) {
		Some(template) => {
			handle_map_template(&mut new_component, template, entity);
		},
		None => {
			new_component.insert(
				NeedsTemplate {
					template: entity.template.clone(),
					map_enitity: entity.clone()
				}
			);
		}
	}
}

pub fn handle_needs_template(
	mut commands: Commands,
	template_map: Res<MapTemplates>,
	query: Query<(Entity, &NeedsTemplate)>
) {
	for (entity, needs_template) in query.iter() {
		match template_map.templates.get(&needs_template.template) {
			Some(template) => {
				let mut entity_commands = commands.entity(entity);
				
				handle_map_template(&mut entity_commands, template, &needs_template.map_enitity);

				entity_commands.remove::<NeedsTemplate>();
			},
			None => {}
		}
	}
}

pub fn handle_map_changes(
	mut commands: Commands,
	chnages_receiver: Res<MapChangesReceiver>,
	mut map_templates: ResMut<MapTemplates>, 
	mut gltf_register: ResMut<GltfRegister>,
	mut done: Local<bool>,
	asset_server: Res<AssetServer>
) {
	if *done {
		return;
	}

	let chnages_receiver = chnages_receiver.rx.lock().unwrap();

	loop {
		match chnages_receiver.try_recv() {
			Ok(change) => {
				log::info!("mapchange {:?}", change);

				match change {
					MapChange::NewMapEntity(entity) => {
						spaw_map_entity(&mut commands, &map_templates, &entity)
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

						map_templates.templates.insert(template.name.clone(), template);
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

pub fn give_assets(
	mut commands: Commands,
	query: Query<(Entity, &NeedsAsset)>,
	asset_packs: Res<AssetPacks>
) {
	for (entity, needs_asset) in query.iter() {
		match asset_packs.asset_packs.get(&needs_asset.asset) {
			Some(asset_pack) => {
				let scene = asset_pack.scenes.values().next().unwrap().clone();

				let mut entity_commands = commands.entity(entity);

				entity_commands.with_children(|parent| {
					parent.spawn(
						SceneBundle {
							scene: scene,
							..Default::default()
						}
					);
				});

				entity_commands.remove::<NeedsAsset>();
			},
			None => {
				log::info!("no asset {:?}", needs_asset.asset);
			}
		}
	}
}
