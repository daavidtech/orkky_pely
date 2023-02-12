use bevy::prelude::Resource;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeymapKey {
	MouseLeft,
	MouseRight,
	W,
	A,
	S,
	D,
	Space,
	LShift,
	Ctrl,
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	None
}

impl Default for KeymapKey {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, Resource)]
pub struct Keymap {
	pub move_forward: KeymapKey,
	pub move_backward: KeymapKey,
	pub move_left: KeymapKey,
	pub move_right: KeymapKey,
	pub attack_1: KeymapKey,
	pub weapon_special_function: KeymapKey,
	pub inventory_1: KeymapKey,
	pub inventory_2: KeymapKey,
	pub inventory_3: KeymapKey,
	pub inventory_4: KeymapKey,
	pub inventory_5: KeymapKey,
	pub jump: KeymapKey,
	pub run: KeymapKey,
	pub crouch: KeymapKey,
}

impl Default for Keymap {
	fn default() -> Self {
		Self {
			move_forward: KeymapKey::W,
			move_backward: KeymapKey::S,
			move_left: KeymapKey::A,
			move_right: KeymapKey::D,
			attack_1: KeymapKey::MouseLeft,
			weapon_special_function: KeymapKey::MouseRight,
			inventory_1: KeymapKey::Key1,
			inventory_2: KeymapKey::Key2,
			inventory_3: KeymapKey::Key3,
			inventory_4: KeymapKey::Key4,
			inventory_5: KeymapKey::Key5,
			jump: KeymapKey::Space,
			run: KeymapKey::LShift,
			crouch: KeymapKey::Ctrl,
		}
	}
}

impl Keymap {
	pub fn save(&self, path: &str) {
		let json = serde_json::to_string_pretty(&self).unwrap();
		std::fs::write(path, json).unwrap();
	}

	pub fn load(path: &str) -> Keymap {
		let keymap = std::fs::read_to_string(path).unwrap();
		let keymap: Keymap = serde_json::from_str(&keymap).unwrap();
		keymap
	}
}
