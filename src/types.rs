use bevy::gltf::Gltf;
use bevy::prelude::*;



#[derive(Clone, Component)]
pub struct You;


#[derive(Clone, Component)]
pub struct AddCollidingMesh {
	pub glft: Handle<Gltf>,
}
