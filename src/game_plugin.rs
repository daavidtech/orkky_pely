use bevy::prelude::*;
use bevy_fps_controller::controller::FpsControllerPlugin;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;

use crate::animations::handle_start_animation;
use crate::animations::handle_stop_animation;
use crate::animations::detect_animation_players;
use crate::animations::link_animation_players;
use crate::collisions::add_collisions;
use crate::collisions::move_melee_hitbox;
use crate::despawn::despawn_screen;
use crate::gltf::unpack_gltf;
use crate::input_handling::keyboard_handler;
use crate::input_handling::mouse_handlers;
use crate::map_changes::give_assets;
use crate::map_changes::give_camera;
use crate::map_changes::handle_map_changes;
use crate::map_changes::handle_needs_template;
use crate::player_control::handle_mouse_input;
use crate::player_control::move_asset;
use crate::player_control::rotate_asset;
use crate::types::AssetPacks;
use crate::types::GameState;
use crate::types::GltfRegister;
use crate::types::MapTemplates;
use crate::types::MeleeHitbox;
use crate::types::PlayerIds;
use crate::types::RotateThing;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
			.add_plugin(FpsControllerPlugin)
			.add_plugin(RapierDebugRenderPlugin::default())
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
					.with_system(rotate_asset)
					.with_system(move_asset)
					.with_system(display_events)
					.with_system(rotate)
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

	commands.spawn((
		Camera3dBundle::default(),
	));
}

fn display_events(
	mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
	query: Query<(Entity, &MeleeHitbox)>,
	parents: Query<&Parent>,
) {
    for collision_event in collision_events.iter() {
        // println!("Received collision event: {:?}", collision_event);
		match collision_event {
			CollisionEvent::Started(a, b, c) => {
				let parent = match parents.get(a.clone()) {
					Ok(p) => p,
					Err(_) => continue,
				};

				let m = match query.get(parent.get()) {
					Ok((_, m)) => m,
					Err(_) => continue,
				};

				println!("Melee hitbox {:?} hit {:?}", m, b);
			},
			CollisionEvent::Stopped(_, _, _) => {},
		}


		match collision_event {
			CollisionEvent::Started(a, b, c) => {
				let parent = match parents.get(b.clone()) {
					Ok(p) => p,
					Err(_) => continue,
				};

				let m = match query.get(parent.get()) {
					Ok((_, m)) => m,
					Err(_) => continue,
				};

				println!("Melee hitbox {:?} hit {:?}", m, b);
			},
			CollisionEvent::Stopped(_, _, _) => {},
		}
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}


pub fn rotate(
	mut commands: Commands,
	mut query: Query<(Entity, &RotateThing, &mut Transform)>
) {
	for (entity, rotate, mut transform) in query.iter_mut() {
		transform.rotate_y(rotate.y.to_radians());

		let mut entity_commands = commands.entity(entity);

		entity_commands.remove::<RotateThing>();
	}
}
