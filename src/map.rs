use bevy::prelude::Resource;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapEntity {
	pub template: String,
	pub position: Option<[f32; 3]>,
	pub rotation: Option<[f32; 3]>,
	pub scale: Option<f32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapTemplate {
	pub name: String,
	pub asset: Option<String>,
	pub player_controllable: Option<bool>,
	pub iddle_animation: Option<String>,
	pub walk_animation: Option<String>,
	pub run_animation: Option<String>,
	pub reload_animation: Option<String>,
	pub shoot_animation: Option<String>,
	pub fps_camera_location: Option<[f32; 3]>,
	pub third_person_camera_location: Option<[f32; 3]>,
	pub initial_rotation_y: Option<f32>,
	pub initial_rotation_x: Option<f32>,
	pub initial_rotation_z: Option<f32>,
	pub initial_transform: Option<[f32; 3]>,
	pub walk_speed: Option<f32>,
	pub run_speed: Option<f32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
	pub entities: Vec<MapEntity>,
	pub templates: Vec<MapTemplate>
}

impl Map {
	pub fn load(path: &str) -> Map {
		let map = std::fs::read_to_string(path).unwrap();
		let map: Map = serde_json::from_str(&map).unwrap();
		map
	}
}

#[derive(Debug, Clone)]
pub enum MapChange {
	NewMapEntity(MapEntity),
	NewMapTemplate(MapTemplate),
}

#[derive(Debug, Clone, Resource)]
pub struct MapChanges {
	pub changes: Vec<MapChange>
}

impl MapChanges {
	pub fn new() -> MapChanges {
		MapChanges {
			changes: Vec::new()
		}
	}
}
