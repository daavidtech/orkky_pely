use std::f32::consts::PI;

use bevy::prelude::AnimationClip;
use bevy::prelude::AssetServer;
use bevy::prelude::BuildChildren;
use bevy::prelude::Camera3dBundle;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Handle;
use bevy::prelude::Quat;
use bevy::prelude::Res;
use bevy::prelude::SpatialBundle;
use bevy::prelude::Transform;
use bevy::prelude::Vec3;
use bevy::prelude::default;
use bevy::scene::SceneBundle;
use bevy_fps_controller::controller::RenderPlayer;

#[derive(Clone, PartialEq)]
pub enum CharacterType {
	You,
	Npc,
}

#[derive(Clone, Component)]
pub struct Character {
	pub idle_animation: Handle<AnimationClip>,
	pub walk_animation: Handle<AnimationClip>,
	pub running_animation: Handle<AnimationClip>,
	pub reload_animation: Handle<AnimationClip>,
	pub moving: bool,
	pub character_type: CharacterType,
}

pub fn spawn_camera_person(
	asset_server: Res<AssetServer>,
	mut commands: Commands
) {
	let you = Character {
		idle_animation: asset_server.load("smg_fps_animations.glb#Animation0"),
		walk_animation: asset_server.load("smg_fps_animations.glb#Animation1"),
		running_animation: asset_server.load("smg_fps_animations.glb#Animation2"),
		reload_animation: asset_server.load("smg_fps_animations.glb#Animation3"),
		moving: false,
		character_type: CharacterType::You,
	};

	// commands.insert_resource(you.idle_animation.clone());
	// commands.insert_resource(you.walk_animation.clone());
	// commands.insert_resource(you.running_animation.clone());
	// commands.insert_resource(you.reload_animation.clone());

	commands.spawn((
		SpatialBundle {
			..Default::default()
		},
        RenderPlayer(0)
	)).with_children(|parent| {
		parent.spawn((
			SceneBundle {
				scene: asset_server.load("smg_fps_animations.glb#Scene0"),
				transform: Transform {
					rotation: Quat::from_rotation_y(PI),
					translation: Vec3::new(0.0, -0.5, 0.0),
					..Default::default()
				},
				..default()
			},
			you,
		));

		parent.spawn((
			Camera3dBundle {
				//transform: Transform::from_xyz(10.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
				// transform: Transform {
				// 	rotation: Quat::from_rotation_y(PI),
				// 	..Default::default()
				// },
				..default()
			},
			
		));
	});

	// commands
}
