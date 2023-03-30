use bevy::prelude::Resource;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
	Melee,
	Ranged
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
	pub weapon_type: WeaponType,
	pub animation: Option<String>,
	pub damage: Option<f32>,
	pub range: Option<f32>,
	pub duration: Option<f32>,
	pub ammo: Option<usize>
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MapEntity {
	#[serde(default)]
	pub entity_id: String,
	pub template: String,
	pub initial_position: Option<[f32; 3]>,
	pub initial_rotation: Option<[f32; 3]>,
	pub scale: Option<f32>,
	pub player: Option<bool>,
	pub npc: Option<bool>,
	pub move_cycle: Option<Vec<[f32; 3]>>,
	pub max_health: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MapEntityPhysics {
	Static,
	Dynamic,
	Kinematic
}

impl Default for MapEntityPhysics {
	fn default() -> Self {
		MapEntityPhysics::Static
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapTemplate {
	pub name: String,
	pub asset: Option<String>,
	pub player_controllable: Option<bool>,
	pub iddle_animation: Option<String>,
	pub walk_animation: Option<String>,
	pub run_animation: Option<String>,
	pub jump_animation: Option<String>,
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
	pub friction: Option<f32>,
	pub physics: Option<MapEntityPhysics>,
	pub automatic_collision_mesh: Option<bool>,
	#[serde(default)]
	pub weapons: Vec<Weapon>,
	pub death_sound_effect: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapCube {
	pub size: f32
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapPlane {
	pub size: f32,
	pub material: Option<String>,
	pub location: Option<[f32; 3]>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapQuad {
	pub size: [f32; 2]
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapCircle {
	pub radius: f32,
	pub vertices: Option<usize>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapBox {
	pub min_x: f32,
	pub max_x: f32,
	pub min_y: f32,
	pub max_y: f32,
	pub min_z: f32,
	pub max_z: f32,
	pub collider: Option<bool>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MapShapeType {
	Cube(MapCube),
	Plane(MapPlane),
	Quad(MapQuad),
	Circle(MapCircle),
	Box(MapBox)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapShape {
	#[serde(default)]
	pub id: String,
	pub shape: MapShapeType
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointMapLight {
	pub color: String,
	pub intensity: Option<f32>,
	pub range: Option<f32>,
	pub radius: Option<f32>,
	pub shadows_enabled: Option<bool>,
	pub location: Option<[f32; 3]>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LightType {
	Point(PointMapLight)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Light {
	#[serde(default)]
	pub id: String,
	#[serde(rename = "type")]
	pub light_type: LightType
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbientLight {
	pub color: String,
	pub brightness: f32
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CameraType {
	FPS,
	ThirdPerson
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapCamera {
	pub camera_type: Option<CameraType>,
	pub entity_id: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Map {
	pub entities: Option<Vec<MapEntity>>,
	pub templates: Option<Vec<MapTemplate>>,
	pub shapes: Option<Vec<MapShape>>,
	pub lights: Option<Vec<Light>>,
	pub ambient_light: Option<AmbientLight>,
	pub camera: Option<MapCamera>
}

impl Map {
	pub fn load(path: &str) -> anyhow::Result<Map> {
		let map = std::fs::read_to_string(path)?;
		let map: Map = serde_json::from_str(&map)?;
		Ok(map)
	}
}

#[derive(Debug, Clone)]
pub enum MapChange {
	NewMapEntity(MapEntity),
	UpdateMapEntity(MapEntity),
	RemoveMapEntity(String),
	NewMapTemplate(MapTemplate),
	UpdateMaptemplate(MapTemplate),
	RemoveMapTemplate(String),
	NewMapShape(MapShape),
	UpdateMapShape(MapShape),
	RemoveMapShape(String),
	NewLight(Light),
	UpdateLight(Light),
	RemoveLight(String),
	NewAmbientLight(AmbientLight),
	UpdateAmbientLight(AmbientLight),
	RemoveAmbientLight,
	NewCamera(MapCamera),
	UpdateCamera(MapCamera),
	RemoveCamera
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

