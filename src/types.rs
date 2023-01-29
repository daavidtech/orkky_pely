use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::map::MapEntity;
use crate::map::MapTemplate;



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
}

#[derive(Clone, Component)]
pub struct SetAnimation {
	pub animation: String,
}

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
	pub scenes: HashMap<String, Handle<Scene>>,
	pub animations: HashMap<String, Handle<AnimationClip>>
}

#[derive(Clone, Resource, Default)]
pub struct AssetPacks {
	pub asset_packs: HashMap<String, AssetPack>
}
