use bevy::asset::HandleId;
use bevy::gltf::Gltf;
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use bevy::utils::HashSet;


pub fn inspect_asset_packs(
	assets_gltf: Res<Assets<Gltf>>,
	mut inspected_asset_packs: Local<HashSet<HandleId>>
) {
	assets_gltf.iter().for_each(|(id, gltf)| {
		log::info!("inspecting gltf {:?}", id);

		for (mesh_name, _) in &gltf.named_meshes {
			log::info!("{:?} found named mesh {}", id, mesh_name);
		}

		for (material_name, _) in &gltf.named_materials {
			log::info!("{:?} found named material {}", id, material_name);
		}

		for (named_node, _) in &gltf.named_nodes {
			log::info!("{:?} found named node {}", id, named_node);
		}

		gltf.named_scenes.iter().for_each(|(scene_name, _)| {
			log::info!("{:?} found named scene {}", id, scene_name);
		});

		gltf.named_animations.iter().for_each(|(animation_name, _)| {
			log::info!("{:?} found named animation {}", id, animation_name);
		});

		// for mesh in gltf.meshes.iter() {
		// 	let mesh = match assets_gltf_mesh.get(mesh) {
		// 		Some(m) => m,
		// 		None => return,
		// 	};

		// 	log::info!("found mesh {:?}", mesh);

		// 	for primite in &mesh.primitives {
		// 		let mesh = assets_mesh.get(&primite.mesh).unwrap();

		// 		log::info!("found mesh {:?}", mesh);
		// 	}
		// }
		
		inspected_asset_packs.insert(id);
	});
}
