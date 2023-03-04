use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use game_ui_plugin::GameUiPlugin;

use crate::animations::handle_start_animation;
use crate::animations::handle_stop_animation;
use crate::animations::detect_animation_players;
use crate::animations::link_animation_players;
use crate::collisions::add_collisions;
use crate::collisions::move_melee_hitbox;
use crate::console_plugin::ConsolePlugin;
use crate::despawn::despawn_screen;
use crate::game_ui_plugin;
use crate::gltf::unpack_gltf;
use crate::input_handling::keyboard_handler;
use crate::input_handling::mouse_handlers;
use crate::map_changes::*;
use crate::player_control::*;
use crate::types::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.add_plugin(RapierDebugRenderPlugin::default())
			.add_plugin(GameUiPlugin::default())
			.add_plugin(ConsolePlugin::default())
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
					.with_system(move_game_entity)
					.with_system(display_events)
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

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
	melee_hitboxes: Query<(Entity, &MeleeHitbox)>,
	game_entities: Query<&GameEntity>,
	template_map: Res<MapTemplates>,
	parents: Query<&Parent>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>
) {
    for collision_event in collision_events.iter() {
        // println!("Received collision event: {:?}", collision_event);
		match collision_event {
			CollisionEvent::Started(a, b, c) => {
				let parent = match parents.get(a.clone()) {
					Ok(p) => p,
					Err(_) => continue,
				};

				let m = match melee_hitboxes.get(parent.get()) {
					Ok((_, m)) => m,
					Err(_) => continue,
				};

				let game_entity = match game_entities.get(b.clone()) {
					Ok(g) => g,
					Err(_) => continue,
				};

				let template = match template_map.templates.get(&game_entity.template) {
					Some(t) => t,
					None => continue,
				};

				println!("Melee hitbox {:?} hit {:?}", m, b);

				if let Some(sound_effect) = &template.death_sound_effect {
					let music = asset_server.load(sound_effect);
					audio.play(music);
				}
			},
			CollisionEvent::Stopped(_, _, _) => {},
		}
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
