use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::scene::Scene;
use bevy::scene::SceneBundle;
use bevy_fps_controller::controller::RenderPlayer;

use crate::gltf::UnloadedAssets;
use crate::types::You;

#[derive(Clone, PartialEq)]
pub enum CharacterType {
	You,
	Npc,
}

#[derive(Clone)]
pub enum MoveState {
	Idle,
	Walking,
	Running,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CurrentAnimation {
	IdleAnimation,
	WalkingAnimation,
	RunningAnimation,
	ShootingAnimation,
	ReloadAnimation,
}

impl Default for CurrentAnimation {
	fn default() -> Self {
		Self::IdleAnimation
	}
}

#[derive(Clone, Component, Debug, Default)]
pub struct Character {
	pub asset_name: Option<String>,
	pub idle_animation: Option<String>,
	pub walking_animation: Option<String>,
	pub running_animation: Option<String>,
	pub shooting_animation: Option<String>,
	pub reload_animation: Option<String>,
	pub current_animation: CurrentAnimation,
	pub moving: bool,
	pub running: bool,
	pub reloading: bool,
	pub shooting: bool,
	pub aiming: bool,
	pub health: u32,
	pub jump: bool,
}

pub fn spawn_character(
	mut commands: Commands,
	character: Character,
) {
	// let scene = character.scene.clone();

	// let id = commands.spawn((
	// 	SpatialBundle {
	// 		..Default::default()
	// 	},
    //     RenderPlayer(0),
	// 	character,
	// 	You
	// )).with_children(|parent| {
	// 	let child_schene_bundle_id = parent.spawn(
	// 		SceneBundle {
	// 			scene: scene,
	// 			transform: Transform {
	// 				rotation: Quat::from_rotation_y(PI),
	// 				translation: Vec3::new(0.0, -0.5, 0.0),
	// 				..Default::default()
	// 			},
	// 			..default()
	// 		}
	// 	).id();

	// 	log::info!("child_schene_bundle_id: {:?}", child_schene_bundle_id);

	// 	let camera_id = parent.spawn((
	// 		Camera3dBundle::default(),
	// 	)).id();

	// 	log::info!("camera_id: {:?}", camera_id);
	// }).id();

	// log::info!("Spawned character with id: {:?}", id);
}

pub fn spawn_camera_person(
	asset_server: Res<AssetServer>,
	mut unloaded: ResMut<UnloadedAssets>,
	mut commands: Commands
) {
	// let asset = asset_server.load("smg_fps_animations.glb");

	// unloaded.0.push(("smg_fps_animations.glb".to_string(), asset.clone()));

	// let you = Character {
	// 	scene: asset_server.load("smg_fps_animations.glb#Scene0"),
	// 	asset_name: "smg_fps_animations.glb".to_string(),
	// 	idle_animation: "Rig|KDW_DPose_Idle".to_string(),
	// 	walking_animation: "Rig|KDW_Walk".to_string(),
	// 	running_animation: "Rig|KDW_Run".to_string(),
	// 	reload_animation: "Rig|KDW_Reload_full".to_string(),
	// 	shooting_animation: "Rig|KDW_Shot".to_string(),
	// 	moving: false,
	// 	reloading: false,
	// 	shooting: false,
	// 	health: 100
	// };

	// spawn_character(commands, you);

	// commands.spawn((
	// 	SpatialBundle {
	// 		..Default::default()
	// 	},
    //     RenderPlayer(0)
	// )).with_children(|parent| {
	// 	parent.spawn((
	// 		SceneBundle {
	// 			scene: asset_server.load("smg_fps_animations.glb#Scene0"),
	// 			transform: Transform {
	// 				rotation: Quat::from_rotation_y(PI),
	// 				translation: Vec3::new(0.0, -0.5, 0.0),
	// 				..Default::default()
	// 			},
	// 			..default()
	// 		},
	// 		you,
	// 	));

	// 	parent.spawn((
	// 		Camera3dBundle::default(),
	// 	));
	// });
}
