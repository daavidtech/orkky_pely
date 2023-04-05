use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use game_ui_plugin::GameUiPlugin;

use crate::animations::ensure_animation;
use crate::animations::handle_start_animation;
use crate::animations::handle_stop_animation;
use crate::animations::detect_animation_players;
use crate::animations::link_animation_players;
use crate::attack::handle_attack;
use crate::bullet::BulletPlugin;
use crate::collisions::add_collisions;
use crate::collisions::move_melee_hitbox;
use crate::console_plugin::ConsolePlugin;
use crate::death::TargetPlugin;
use crate::despawn::despawn_screen;
use crate::game_ui_plugin;
use crate::gltf::unpack_gltf;

use crate::ingame_menu::GameMenuPlugin;
use crate::input_handling::keyboard_handler;
use crate::input_handling::mouse_handlers;
use crate::map_changes::*;
use crate::npc::NpcPlugin;
use crate::npc::handle_cycle;
use crate::player_control::*;
use crate::throw::TowerPlugin;
use crate::types::*;





pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.add_plugin(RapierDebugRenderPlugin::default())
			.add_plugin(GameUiPlugin::default())
			.add_plugin(ConsolePlugin::default())
			.add_plugin(GameMenuPlugin::default())
			.add_plugin(NpcPlugin)
			.add_plugin(TowerPlugin)
			.add_plugin(TargetPlugin)
			.add_plugin(BulletPlugin)
			.insert_resource(RapierConfiguration::default())
			.insert_resource(MapTemplates::default())
			.insert_resource(GltfRegister::default())
			.insert_resource(AssetPacks::default())
			.insert_resource(PlayerIds::default())
			.add_system_set(
				SystemSet::on_enter(GameState::Game)
					.with_system(setup)			
			)
			.add_system_set(
				SystemSet::on_update(GameState::Game)
					.with_system(handle_start_animation)
					.with_system(handle_stop_animation)
					.with_system(detect_animation_players)
					.with_system(link_animation_players)
					.with_system(handle_map_changes)
					.with_system(handle_needs_template)
					.with_system(unpack_gltf)
					.with_system(give_assets)
					.with_system(give_camera)
					.with_system(add_collisions)
					.with_system(keyboard_handler)
					.with_system(mouse_handlers)
					.with_system(move_melee_hitbox)
					.with_system(handle_mouse_input)
		         	//.with_system(meleehitbox_damage)
					.with_system(move_game_entity)
					.with_system(game_entity_bullet_contact)
					.with_system(handle_cycle)
					.with_system(ensure_animation)
					.with_system(handle_attack)
			)
			.add_system_set(
				SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
			);
	}
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

pub fn setup(
	mut commands: Commands,
	camera_2d: Query<Entity, With<Camera2d>>,
	camera_3d: Query<Entity, With<Camera3d>>,
) {
	for entity in &camera_2d {
		commands.entity(entity).despawn_recursive();
	}
	for entity in &camera_3d {
		commands.entity(entity).despawn_recursive();
	}
}

fn game_entity_bullet_contact(
	mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
	names: Query<&Name>,
	bullets: Query<(Entity, &BulletProperties)>,
	mut game_entities: Query<(Entity, &mut GameEntity)>,
	template_map: Res<MapTemplates>,
	parents: Query<&Parent>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut game_state: ResMut<State<GameState>>,
) {
    for collision_event in collision_events.iter() {
        // println!("Received collision event: {:?}", collision_event);
		match collision_event {
			CollisionEvent::Started(a, b, c) => {
				if a == b {
					log::info!("collision between same entities");
					continue;
				}

				match (names.get(*a), names.get(*b)) {
					(Ok(a_name), Ok(b_name)) => {
						log::info!("collision between {} and {}", a_name.as_str(), b_name.as_str());
					},
					(Ok(a_name), Err(_)) => {
						log::info!("collision between {} and unknown", a_name.as_str());
					},
					(Err(_), Ok(b_name)) => {
						log::info!("collision between unknown and {}", b_name.as_str());
					},
					_ => {},
				};
				
				let (bullent_entity, bullet) = match bullets.get(*a) {
					Ok(b) => b,
					Err(_) => match bullets.get(*b) {
						Ok(b) => b,
						Err(_) => continue,
					},
				};

				let (game_entity_entity, mut game_entity) = match game_entities.get_mut(*a) {
					Ok(g) => g,
					Err(_) => match game_entities.get_mut(*b) {
						Ok(g) => g,
						Err(_) => continue,
					},
				};	

				log::info!("game entity found");

				let template = match template_map.templates.get(&game_entity.template) {
					Some(t) => t,
					None => continue,
				};

				if template.name != "Troll" {
					continue;
				}

				log::info!("bullet {:?} hit {}", bullet, template.name);

				// if let Some(sound_effect) = &template.death_sound_effect {
				// 	let music = asset_server.load(sound_effect);
				// 	audio.play(music);
				// }

				game_entity.curr_health -= bullet.damage;
				
				if game_entity.curr_health <= 0.0 {
					log::info!("game entity dead");
					commands.entity(game_entity_entity).despawn_recursive();
					game_state.set(GameState::GameOver).unwrap();
				}

				let mut bullent_entity_command = commands.entity(bullent_entity);

				bullent_entity_command.remove::<BulletProperties>();
			},
			CollisionEvent::Stopped(_, _, _) => {},
		}
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
