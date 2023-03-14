use bevy::gltf::Gltf;
use bevy::utils::HashMap;

use bevy::prelude::*;
use crate::map::*;

#[derive(Clone, Component)]
pub struct You;

#[derive(Clone, Component)]
pub struct AddCollidingMesh {
	pub glft: Handle<Gltf>,
}

#[derive(Clone, Component)]
pub struct NeedsTemplate {
	pub template: String,
	pub map_enitity: MapEntity
}

#[derive(Clone, Component)]
pub struct NeedsAsset {
	pub asset: String,
	pub add_colliding_mesh: bool,
	pub initial_transform: Option<[f32; 3]>,
	pub initial_rotation_y: Option<f32>,
}

#[derive(Clone, Component, Default)]
pub struct StartAnimation {
	pub asset: String,
	pub animation: String,
	pub repeat: bool,
}

#[derive(Clone, Component, Default)]
pub struct CurrentAnimation {
	pub asset: String,
	pub animation: String,
	pub repeat: bool,
}

#[derive(Clone, Component)]
pub struct StopAnimation;

#[derive(Clone, Resource, Default)]
pub struct MapTemplates {
	pub templates: HashMap<String, MapTemplate>
}

#[derive(Clone, Default)]
pub struct UnloadedGltfAsset {
	pub asset: String,
	pub gltf: Handle<Gltf>,
}

#[derive(Clone, Resource, Default)]
pub struct GltfRegister {
	pub unloaded: Vec<UnloadedGltfAsset>,
}

#[derive(Clone, Default)]
pub struct AssetPack {
	pub gltf: Handle<Gltf>,
	pub scenes: Vec<Handle<Scene>>,
	pub named_scenes: HashMap<String, Handle<Scene>>,
	pub animations: Vec<Handle<AnimationClip>>,
	pub named_animations: HashMap<String, Handle<AnimationClip>>
}

#[derive(Clone, Resource, Default)]
pub struct AssetPacks {
	pub asset_packs: HashMap<String, AssetPack>
}

#[derive(Clone, Default)]
pub struct MoveIntent {
	pub move_forward: bool,
	pub move_backward: bool,
	pub move_leftward: bool,
	pub move_rightward: bool,
}

#[derive(Clone, Component, Default)]
pub struct GameEntity {
	pub entity_id: String,
	pub template: String,
	pub current_weapon: usize,
	pub weapons: Vec<Weapon>,
	pub asset: Option<String>,
	pub idle_animation: Option<String>,
	pub walk_animation: Option<String>,
	pub jump_animation: Option<String>,
	pub run_animation: Option<String>,
	pub reload_animation: Option<String>,
	pub shoot_animation: Option<String>,
	pub max_health: f32,
	pub curr_health: f32,
	pub move_intent: MoveIntent,
	pub yaw: f32,
	pub pitch: f32,
	pub npc: bool,
	pub attacking: bool,
}

impl GameEntity {
	pub fn is_moving(&self) -> bool {
		self.move_intent.move_forward || 
		self.move_intent.move_backward || 
		self.move_intent.move_leftward || 
		self.move_intent.move_rightward
	}
}

#[derive(Clone, Resource, Default)]
pub struct EntityStore {
	pub entities: HashMap<String, GameEntity>
}

// #[derive(Clone, Component)]
// pub struct MapEntityId(pub String);


#[derive(Clone, Resource, Default)]
pub struct PlayerIds {
	player_ids: HashMap<String, u8>
}

impl PlayerIds {
	pub fn provide_player_id(&mut self, entity_id: &str) -> u8 {
		if let Some(player_id) = self.player_ids.get(entity_id) {
			return *player_id;
		}

		let player_id = self.player_ids.len() as u8;
		self.player_ids.insert(entity_id.to_string(), player_id);

		player_id
	}
}

#[derive(Clone, Component)]
pub struct StopAttack;


#[derive(Clone, Component, Debug)]
pub struct MeleeHitbox {
	pub delay: f32,
	pub dur: f32,
	pub radius: f32,
	pub start_angle: f32,
	pub end_angle: f32,
}


pub struct MoveToLocation {
	
}

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
}


// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

#[derive(Clone, Component)]
pub struct NeedsCamera {
	pub entity_id: String,
	pub camera_type: Option<CameraType>
}


#[derive(Clone, Component)]
pub struct LifeLost;

#[derive(Clone, Component)]
pub struct LifeLeft;
#[derive(Clone, Component, Default)]
pub struct PlayerCamera {
	pub yaw: f32,
	pub pitch: f32,
}

#[derive(Clone, Component, Default)]
pub struct EntityScene;


#[derive(Clone, Resource, Default)]
pub struct NewMapChanges {
	pub changes: Vec<MapChange>
}
#[derive(Component)]
pub struct Fps;

#[derive(Clone, Component, Default)]
pub struct Attacking {
	pub timer: Timer,
}
