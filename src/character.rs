use bevy::prelude::*;

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
