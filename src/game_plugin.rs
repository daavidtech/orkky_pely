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
use crate::collisions::add_collisions;
use crate::collisions::move_melee_hitbox;
use crate::console_plugin::ConsolePlugin;
use crate::despawn::despawn_screen;
use crate::game_ui_plugin;
use crate::gltf::unpack_gltf;
use crate::input_handling::keyboard_handler;
use crate::input_handling::mouse_handlers;
use crate::map_changes::*;
use crate::npc::NpcPlugin;
use crate::npc::handle_cycle;
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
			.add_plugin(NpcPlugin)
			.insert_resource(RapierConfiguration::default())
			.insert_resource(MapTemplates::default())
			.insert_resource(GltfRegister::default())
			.insert_resource(AssetPacks::default())
			.insert_resource(PlayerIds::default())
			.add_system(setup.in_schedule(OnEnter(GameState::Game)))
			.add_systems((
				handle_map_changes,
				handle_needs_template,
				unpack_gltf,
				give_assets,
				give_camera,
				add_collisions,
				keyboard_handler,
				mouse_handlers,
				move_melee_hitbox,
				handle_mouse_input,
				move_game_entity,
				display_events,
				handle_cycle,
				ensure_animation,
				handle_attack,
			).in_set(OnUpdate(GameState::Game)))
			.add_system(despawn_screen::<OnGameScreen>.in_schedule(OnExit(GameState::Game)));
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
