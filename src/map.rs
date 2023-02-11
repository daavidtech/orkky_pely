use bevy::prelude::Resource;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type")]
pub enum MapEntityCollider {
	AABB,
	Capsule {
		a: f32,
		b: f32,
		radius: f32
	},
	Cuboid {
		x: f32,
		y: f32,
		z: f32,

	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponType {
	Melee,
	Ranged
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
	pub weapon_type: WeaponType,
	pub animation: String,
	pub damage: Option<f32>,
	pub range: Option<f32>,
	pub speed: Option<f32>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapEntity {
	#[serde(default)]
	pub entity_id: String,
	pub template: String,
	pub initial_position: Option<[f32; 3]>,
	pub initial_rotation: Option<[f32; 3]>,
	pub scale: Option<f32>,
	pub player: Option<bool>,
	pub camera: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapEntityPhysics {
	Static,
	Dynamic
}

impl Default for MapEntityPhysics {
	fn default() -> Self {
		MapEntityPhysics::Static
	}
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
	pub run_speed: Option<f32>,
	pub collider: Option<MapEntityCollider>,
	pub mass: Option<f32>,
	pub physics: Option<MapEntityPhysics>,
	pub automatic_collision_mesh: Option<bool>,
	pub weapons: Option<Vec<Weapon>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCube {
	pub size: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapPlane {
	pub size: f32,
	pub material: Option<String>,
	pub location: Option<[f32; 3]>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapQuad {
	pub size: [f32; 2]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCircle {
	pub radius: f32,
	pub vertices: Option<usize>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapBox {
	pub min_x: f32,
	pub max_x: f32,
	pub min_y: f32,
	pub max_y: f32,
	pub min_z: f32,
	pub max_z: f32,
	pub collider: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapShape {
	Cube(MapCube),
	Plane(MapPlane),
	Quad(MapQuad),
	Circle(MapCircle),
	Box(MapBox)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointMapLight {
	pub color: String,
	pub intensity: Option<f32>,
	pub range: Option<f32>,
	pub radius: Option<f32>,
	pub shadows_enabled: Option<bool>,
	pub location: Option<[f32; 3]>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MapLight {
	Point(PointMapLight)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientLight {
	pub color: String,
	pub brightness: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
	pub entities: Option<Vec<MapEntity>>,
	pub templates: Option<Vec<MapTemplate>>,
	pub shapes: Option<Vec<MapShape>>,
	pub lights: Option<Vec<MapLight>>,
	pub ambient_light: Option<AmbientLight>,
	pub camera_entity: Option<String>
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
	NewMapShape(MapShape),
	NewLight(MapLight),
	NewAmbientLight(AmbientLight),
	NewCameraEntity(String)
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

