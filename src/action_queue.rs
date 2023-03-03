use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::templates::handle_update_map_templates;
use crate::types::ActionQueue;
use crate::types::*;

#[derive(Default)]
pub struct ActionQueuePlugin;

impl Plugin for ActionQueuePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ActionQueue::default())
			.add_system(handle_update_map_templates);
	}
}
