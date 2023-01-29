use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;

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
