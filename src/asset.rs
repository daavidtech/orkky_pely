use bevy::prelude::*;

use crate::types::*;

pub fn give_assets(
	mut commands: Commands,
	query: Query<(Entity, &GameEntity, &NeedsAsset)>,
	asset_packs: Res<AssetPacks>
) {
	for (entity, game_entity, needs_asset) in query.iter() {
		match asset_packs.asset_packs.get(&needs_asset.asset) {
			Some(asset_pack) => {
				log::info!("[{}] giving asset: {:?}", game_entity.entity_id, needs_asset.asset);

				let mut entity_commands = commands.entity(entity);
				entity_commands.remove::<NeedsAsset>();

				let scene = if asset_pack.scenes.len() > 0 {
					asset_pack.scenes[0].clone()
				} else {
					log::info!("[{}] no scene found", game_entity.entity_id);
					
					continue;
				};	

				if needs_asset.add_colliding_mesh {
					log::info!("[{}] adding collision mesh", game_entity.entity_id);

					entity_commands.insert(
						AddCollidingMesh {
							glft: asset_pack.gltf.clone(),
						}
					);
				}

				entity_commands.with_children(|parent| {
					log::info!("[{}] assign scene", game_entity.entity_id);

					let mut entity_commands = parent.spawn((
						EntityScene,
						SpatialBundle::default()
					));

					entity_commands.with_children(|parent| {
						let mut bundle = SceneBundle {
							scene: scene,
							..Default::default()
						};
	
						if let Some(transform) = needs_asset.initial_transform {
							log::info!("[{}] initial transform {:?}", game_entity.entity_id, transform);
	
							bundle.transform.translation = Vec3::new(transform[0], transform[1], transform[2]);
						}
	
						if let Some(rotation) = needs_asset.initial_rotation_y {
							log::info!("[{}] initial rotation {:?}", game_entity.entity_id, rotation);
	
							bundle.transform.rotation = Quat::from_rotation_y(
								rotation.to_radians()
							);
						}

						parent.spawn(bundle);
					});
				});
			},
			None => {
				log::info!("[{}] no asset {:?}", game_entity.entity_id, needs_asset.asset);
			}
		}
	}
}
