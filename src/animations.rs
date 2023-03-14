use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::HashSet;

use crate::types::AssetPacks;
use crate::types::CurrentAnimation;
use crate::types::GameEntity;
use crate::types::StartAnimation;
use crate::types::StopAnimation;

// #[derive(Resource, Default)]
// pub struct AnimationStore {
// 	pub animations: HashMap<String, Handle<AnimationClip>>
// }

// impl AnimationStore {
// 	pub fn get_animation(&self, asset: &str, name: &str) -> Option<&Handle<AnimationClip>> {
// 		let key = format!("{}-{}", asset, name);

// 		self.animations.get(&key)
// 	}

// 	pub fn set_animation(&mut self, asset: &str, name: &str, animation: Handle<AnimationClip>) {
// 		let key = format!("{}-{}", asset, name);

// 		self.animations.insert(key, animation);
// 	}
// }

#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animation_players(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
			log::info!("Linking animation player to top parent {:?}", top_entity);

            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

// pub fn find_player(
// 	player_query: Query<&mut AnimationPlayer>,
// 	link: &AnimationEntityLink,
// ) -> &mut AnimationPlayer {
// 	let mut player = player_query.get_mut(link.0).unwrap();
// 	player
// }

// fn play_animation_player(
// 	player: &mut Mut<AnimationPlayer>,
// 	animation_store: &Res<AnimationStore>,
// 	asset: &Option<String>,
// 	animation: &Option<String>
// ) {
// 	match asset {
// 		Some(asset) => {
// 			match animation {
// 				Some(animation) => {
// 					match animation_store.get_animation(asset, animation.as_str()) {
// 						Some(animation_clip) => {
// 							player.start(animation_clip.clone()).repeat();
// 						},
// 						None => {
// 							log::warn!("Animation {} not found", animation);
// 						}
// 					}
// 				},
// 				None => {
// 					log::warn!("Animation not found");
// 				}
// 			}
// 		},
// 		None => {
// 			log::warn!("Asset not found");
// 		}
// 	}
// }

// pub fn change_character_animation(
// 	mut query: Query<(&mut Character, &AnimationEntityLink)>,
// 	mut player_query: Query<&mut AnimationPlayer>,
// 	animation_store: Res<AnimationStore>,
// ) {
// 	for (mut char, link) in query.iter_mut() {
// 		let mut player = match player_query.get_mut(link.0) {
// 			Ok(player) => player,
// 			Err(_) => continue,
// 		};

// 		if char.reloading {
// 			if char.current_animation == CurrentAnimation::ReloadAnimation {
// 				continue;
// 			}

// 			log::info!("chaning to reload animation");

// 			char.current_animation = CurrentAnimation::ReloadAnimation;

// 			play_animation_player(
// 				&mut player, 
// 				&animation_store,
// 				&char.asset_name,
// 				&char.reload_animation);
				
// 			continue;
// 		}

// 		if char.shooting {
// 			if char.current_animation == CurrentAnimation::ShootingAnimation {
// 				continue;
// 			}

// 			log::info!("chaning to shooting animation");

// 			char.current_animation = CurrentAnimation::ShootingAnimation;

// 			play_animation_player(
// 				&mut player, 
// 				&animation_store,
// 				&char.asset_name,
// 				&char.shooting_animation);
// 			continue;
// 		}

// 		// if char.aiming {
// 		// 	play_animation_player(player, animation_store, char.aiming_animation);
// 		// 	continue;
// 		// }

// 		if char.running {
// 			if char.current_animation == CurrentAnimation::RunningAnimation {
// 				continue;
// 			}

// 			log::info!("chaning to running animation");

// 			char.current_animation = CurrentAnimation::RunningAnimation;

// 			play_animation_player(
// 				&mut player, 
// 				&animation_store, 
// 				&char.asset_name,
// 				&char.running_animation);
// 			continue;
// 		}

// 		if char.moving {
// 			if char.current_animation == CurrentAnimation::WalkingAnimation {
// 				continue;
// 			}

// 			log::info!("chaning to walking animation");

// 			char.current_animation = CurrentAnimation::WalkingAnimation;

// 			play_animation_player(
// 				&mut player, 
// 				&animation_store, 
// 				&char.asset_name,
// 				&char.walking_animation);
// 			continue;
// 		}

// 		if char.current_animation == CurrentAnimation::IdleAnimation {
// 			continue;
// 		}

// 		log::info!("changing to idle animation");

// 		char.current_animation = CurrentAnimation::IdleAnimation;

// 		play_animation_player(
// 			&mut player, 
// 			&animation_store, 
// 			&char.asset_name,
// 			&char.idle_animation);




// 		// if let Some(idle_animation) = char.idle_animation {
// 		// 	match animation_store.0.get(idle_animation.as_str()) {
// 		// 		Some(animation) => {
// 		// 			player.start(animation.clone());
// 		// 		},
// 		// 		None => {
// 		// 			log::warn!("Animation ", char);
// 		// 		}
// 		// 	}
// 		// }

// 		// if let Some(walking_animation) = char.walking_animation {
// 		// 	match animation_store.0.get(walking_animation.as_str()) {
// 		// 		Some(animation) => {
// 		// 			player.start(animation.clone());
// 		// 		},
// 		// 		None => {
// 		// 			log::warn!("Animation ", char);
// 		// 		}
// 		// 	}
// 		// }

		
// 	}
// }

pub fn handle_start_animation(
	mut commands: Commands,
	mut query: Query<(
		Entity, 
		&GameEntity, 
		&StartAnimation, 
		&AnimationEntityLink
	)>,
	asset_packs: Res<AssetPacks>,
	mut player_query: Query<&mut AnimationPlayer>,
) {
	for (entity, game_entity, start_animation, link) in query.iter_mut() {
		let mut player = match player_query.get_mut(link.0) {
			Ok(player) => player,
			Err(_) => continue,
		};

		let asset_pack = match asset_packs.asset_packs.get(&start_animation.asset) {
			Some(asset_pack) => asset_pack,
			None => {
				log::warn!("Asset pack {} not found", start_animation.asset);
				continue;
			}
		};

		match asset_pack.named_animations.get(&start_animation.animation) {
			Some(animation) => {
				log::info!("[{}] starting animation {}", game_entity.entity_id, start_animation.animation);
				player.start(animation.clone());
			},
			None => {
				log::warn!("[{}] animation {} not found", game_entity.entity_id, start_animation.animation);

				if asset_pack.animations.len() > 0 {
					let animation = asset_pack.animations.get(0).unwrap();
					player.start(animation.clone()).repeat();
				}
			}
		}

		if start_animation.repeat {
			log::info!("[{}] repeating animation", game_entity.entity_id);
			player.repeat();
		}

		let mut entity_command = commands.entity(entity);
		entity_command.remove::<StartAnimation>();

		entity_command.insert(CurrentAnimation {
			animation: start_animation.animation.clone(),
			asset: start_animation.asset.clone(),
			repeat: start_animation.repeat,
		});
	}
}

pub fn handle_stop_animation(
	mut commands: Commands,
	mut query: Query<(
		Entity, 
		&GameEntity, 
		&StopAnimation, 
		&AnimationEntityLink
	)>,
	mut player_query: Query<&mut AnimationPlayer>,
) {
	for (entity, game_entity, _, link) in query.iter_mut() {
		let mut player = match player_query.get_mut(link.0) {
			Ok(player) => player,
			Err(_) => continue,
		};

		log::info!("[{}] stopping animation", game_entity.entity_id);

		player.stop_repeating();

		let mut entity_command = commands.entity(entity);

		entity_command.remove::<StopAnimation>();
		entity_command.remove::<CurrentAnimation>();
	}
}

pub fn detect_animation_players(
	mut map: Local<HashSet<Entity>>,
	query: Query<(Entity, &AnimationPlayer)>
) {
	for (entity, _) in query.iter() {
		if !map.contains(&entity) {
			log::info!("new animation player found {:?}", entity);

			map.insert(entity);
		}
	}
}


pub fn ensure_animation(
	mut commands: Commands,
	query: Query<(Entity, &GameEntity, Option<&CurrentAnimation>)>,
) {
	for (entity, game_entity, current_animation) in query.iter() {
		let mut entity_commands = commands.entity(entity);

		let asset = match &game_entity.asset {
			Some(asset) => asset,
			None => continue,
		};

		if game_entity.attacking {
			let current_weapon = match game_entity.weapons.get(game_entity.current_weapon) {
				Some(current_weapon) => current_weapon,
				None => continue,
			};

			if let Some(attack_animation) = &current_weapon.animation {
				let cond = match current_animation {
					Some(current_animation) => current_animation.animation != *attack_animation,
					None => true,
				};

				if cond {
					log::info!("[{}] changing to attack animation", game_entity.entity_id);

					entity_commands.insert(StartAnimation {
						asset: asset.clone(),
						animation: attack_animation.clone(),
						repeat: false,
					});
				}
			}

			continue;
		}

		if game_entity.is_moving() {
			if let Some(walk_animation) = &game_entity.walk_animation {
				let cond = match current_animation {
					Some(current_animation) => current_animation.animation != *walk_animation,
					None => true,
				};

				if cond {
					log::info!("[{}] changing to walk animation", game_entity.entity_id);

					entity_commands.insert(StartAnimation {
						asset: asset.clone(),
						animation: walk_animation.clone(),
						repeat: true,
					});
				}
			}
		} else {
			if let Some(idle_animation) = &game_entity.idle_animation {
				let cond = match current_animation {
					Some(current_animation) => current_animation.animation != *idle_animation,
					None => true,
				};

				if cond {
					log::info!("[{}] changing to idle animation", game_entity.entity_id);

					entity_commands.insert(StartAnimation {
						asset: asset.clone(),
						animation: idle_animation.clone(),
						repeat: true,
					});
				}
			}
		}
	}
}
