use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::character::Character;
use crate::character::CurrentAnimation;

#[derive(Resource, Default)]
pub struct AnimationStore {
	pub animations: HashMap<String, Handle<AnimationClip>>
}

impl AnimationStore {
	pub fn get_animation(&self, asset: &str, name: &str) -> Option<&Handle<AnimationClip>> {
		let key = format!("{}-{}", asset, name);

		self.animations.get(&key)
	}

	pub fn set_animation(&mut self, asset: &str, name: &str, animation: Handle<AnimationClip>) {
		let key = format!("{}-{}", asset, name);

		self.animations.insert(key, animation);
	}
}

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

fn play_animation_player(
	player: &mut Mut<AnimationPlayer>,
	animation_store: &Res<AnimationStore>,
	asset: &Option<String>,
	animation: &Option<String>
) {
	match asset {
		Some(asset) => {
			match animation {
				Some(animation) => {
					match animation_store.get_animation(asset, animation.as_str()) {
						Some(animation_clip) => {
							player.start(animation_clip.clone()).repeat();
						},
						None => {
							log::warn!("Animation {} not found", animation);
						}
					}
				},
				None => {
					log::warn!("Animation not found");
				}
			}
		},
		None => {
			log::warn!("Asset not found");
		}
	}
}

pub fn change_character_animation(
	mut query: Query<(&mut Character, &AnimationEntityLink)>,
	mut player_query: Query<&mut AnimationPlayer>,
	animation_store: Res<AnimationStore>,
) {
	for (mut char, link) in query.iter_mut() {
		let mut player = match player_query.get_mut(link.0) {
			Ok(player) => player,
			Err(_) => continue,
		};

		if char.reloading {
			if char.current_animation == CurrentAnimation::ReloadAnimation {
				continue;
			}

			log::info!("chaning to reload animation");

			char.current_animation = CurrentAnimation::ReloadAnimation;

			play_animation_player(
				&mut player, 
				&animation_store,
				&char.asset_name,
				&char.reload_animation);
				
			continue;
		}

		if char.shooting {
			if char.current_animation == CurrentAnimation::ShootingAnimation {
				continue;
			}

			log::info!("chaning to shooting animation");

			char.current_animation = CurrentAnimation::ShootingAnimation;

			play_animation_player(
				&mut player, 
				&animation_store,
				&char.asset_name,
				&char.shooting_animation);
			continue;
		}

		// if char.aiming {
		// 	play_animation_player(player, animation_store, char.aiming_animation);
		// 	continue;
		// }

		if char.running {
			if char.current_animation == CurrentAnimation::RunningAnimation {
				continue;
			}

			log::info!("chaning to running animation");

			char.current_animation = CurrentAnimation::RunningAnimation;

			play_animation_player(
				&mut player, 
				&animation_store, 
				&char.asset_name,
				&char.running_animation);
			continue;
		}

		if char.moving {
			if char.current_animation == CurrentAnimation::WalkingAnimation {
				continue;
			}

			log::info!("chaning to walking animation");

			char.current_animation = CurrentAnimation::WalkingAnimation;

			play_animation_player(
				&mut player, 
				&animation_store, 
				&char.asset_name,
				&char.walking_animation);
			continue;
		}

		if char.current_animation == CurrentAnimation::IdleAnimation {
			continue;
		}

		log::info!("changing to idle animation");

		char.current_animation = CurrentAnimation::IdleAnimation;

		play_animation_player(
			&mut player, 
			&animation_store, 
			&char.asset_name,
			&char.idle_animation);




		// if let Some(idle_animation) = char.idle_animation {
		// 	match animation_store.0.get(idle_animation.as_str()) {
		// 		Some(animation) => {
		// 			player.start(animation.clone());
		// 		},
		// 		None => {
		// 			log::warn!("Animation ", char);
		// 		}
		// 	}
		// }

		// if let Some(walking_animation) = char.walking_animation {
		// 	match animation_store.0.get(walking_animation.as_str()) {
		// 		Some(animation) => {
		// 			player.start(animation.clone());
		// 		},
		// 		None => {
		// 			log::warn!("Animation ", char);
		// 		}
		// 	}
		// }

		
	}
}
