use std::f32::consts::PI;
use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_fps_controller::controller::FpsController;
use bevy_fps_controller::controller::FpsControllerInput;
use bevy_fps_controller::controller::LogicalPlayer;
use bevy_fps_controller::controller::RenderPlayer;
use bevy_rapier3d::prelude::*;

use crate::animations::AnimationEntityLink;
use crate::assets::UnloadedAssets;
use crate::character::Character;
use crate::types::You;


pub struct Spawner {
	you: bool,
	asset: Option<String>,
	idle_animation: Option<String>,
	walking_animation: Option<String>,
	running_animation: Option<String>,
	reload_animation: Option<String>,
	shooting_animation: Option<String>,
	transform: Transform,
	health: u32,
}

impl Spawner {
	pub fn new() -> Self {
		Self {
			you: false,
			asset: None,
			idle_animation: None,
			walking_animation: None,
			running_animation: None,
			reload_animation: None,
			shooting_animation: None,
			health: 100,
			transform: Transform::default()
		}
	}

	pub fn set_you(mut self, you: bool) -> Self {
		self.you = you;

		self
	}

	pub fn set_transform(mut self, transform: Transform) -> Self {
		self.transform = transform;

		self
	}

	pub fn set_asset(mut self, asset: &str) -> Self {
		self.asset = Some(asset.to_string());

		self
	}

	pub fn set_idle_animation(mut self, idle_animation: &str) -> Self {
		self.idle_animation = Some(idle_animation.to_string());

		self
	}

	pub fn set_walking_animation(mut self, walking_animation: &str) -> Self {
		self.walking_animation = Some(walking_animation.to_string());

		self
	}

	pub fn set_running_animation(mut self, running_animation: &str) -> Self {
		self.running_animation = Some(running_animation.to_string());

		self
	}

	pub fn set_reload_animation(mut self, reload_animation: &str) -> Self {
		self.reload_animation = Some(reload_animation.to_string());

		self
	}

	pub fn set_shooting_animation(mut self, shooting_animation: &str) -> Self {
		self.shooting_animation = Some(shooting_animation.to_string());

		self
	}

	pub fn set_health(mut self, health: u32) -> Self {
		self.health = health;

		self
	}

	pub fn spawn(
		&self, 
		mut commands: Commands,
		asset_server: Res<AssetServer>,
		mut unloaded_assets: ResMut<UnloadedAssets>,
	) {
		if let Some(asset) = &self.asset {
			let scene_name = format!("{}", asset);

			log::info!("loading {}", scene_name);

			let assetpack = asset_server.load(scene_name);

			unloaded_assets.0.push((asset.to_string(), assetpack));
		}
		

		commands.spawn((
			Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
			ActiveEvents::COLLISION_EVENTS,
			Velocity::zero(),
			RigidBody::Dynamic,
			Sleeping::disabled(),
			LockedAxes::ROTATION_LOCKED,
			AdditionalMassProperties::Mass(1.0),
			GravityScale(0.0),
			Ccd { enabled: true }, // Prevent clipping when going fast
			TransformBundle::from_transform(Transform::from_xyz(10.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)),
			LogicalPlayer(0),
			FpsControllerInput {
				pitch: -TAU / 12.0,
				yaw: TAU * 5.0 / 8.0,
				..default()
			},
			FpsController { ..default() }
		));

		let character = Character {
			asset_name: self.asset.clone(),
			idle_animation: self.idle_animation.clone(),
			walking_animation: self.walking_animation.clone(),
			running_animation: self.running_animation.clone(),
			reload_animation: self.reload_animation.clone(),
			shooting_animation: self.shooting_animation.clone(),
			health: self.health,
			..Default::default()
		};

		let mut entity = commands.spawn((
			SpatialBundle {
				transform: self.transform,
				..Default::default()
			},
			RenderPlayer(0),
			character,
		));
		
		entity.with_children(|parent| {
			match &self.asset {
				Some(asset) => {
					let scene = asset_server.load(asset.to_owned() + "#Scene0");

					parent.spawn(
						SceneBundle {
							scene: scene,
							transform: Transform {
								rotation: Quat::from_rotation_y(PI),
								translation: Vec3::new(0.0, -0.5, 0.0),
								..Default::default()
							},
							..default()
						}
					);
				},
				None => {
					log::warn!("No asset specified for character");
				},
			}

			if self.you {
				log::info!("spawning camera");

				parent.spawn((
					Camera3dBundle::default(),
				));
			}
		});

		if self.you {
			entity.insert(You);
		}
	}
}
