use bevy::prelude::Component;

#[derive(Component)]
pub struct CharacterVisual {
	pub idle_animation: String,
	pub walking_animation: String,
	pub running_animation: String,
	pub shooting_animation: String,
	pub reload_animation: String,
}
