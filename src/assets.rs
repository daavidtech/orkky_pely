use bevy::gltf::Gltf;
use bevy::prelude::AnimationClip;
use bevy::prelude::Handle;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

pub struct AssetPack {

}

#[derive(Clone)]
pub struct Animation {

}

impl Animation {
	pub fn new(asset: &str, name: &str) -> Self {
		Self {

		}
	}
}

#[derive(Resource, Default)]
pub struct UnloadedAssets(pub Vec<(String, Handle<Gltf>)>);


#[derive(Resource, Default)]
pub struct AssetPacks(pub Vec<Handle<Gltf>>);


#[derive(Resource)]
pub struct AssetManager {
	pub unloaded: Vec<(String, Handle<Gltf>)>,
	pub animations: HashMap<String, AnimationClip>,
}

impl AssetManager {
	pub fn new() -> Self {
		Self {
			unloaded: Vec::new(),
			animations: HashMap::new(),
		}
	}

	pub fn add_asset(&mut self, name: &str, asset: Handle<Gltf>) {
		self.unloaded.push((name.to_string(), asset));
	}
}

pub struct Scene {

}
