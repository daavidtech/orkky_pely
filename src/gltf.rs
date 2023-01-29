use bevy::gltf::Gltf;
use bevy::prelude::*;

use crate::types::AssetPack;
use crate::types::AssetPacks;
use crate::types::GltfRegister;


#[derive(Resource, Default)]
pub struct UnloadedAssets(pub Vec<(String, Handle<Gltf>)>);


pub fn unpack_gltf(
	mut gltf_register: ResMut<GltfRegister>,
	gltf_assets: Res<Assets<Gltf>>,
	mut asset_packs: ResMut<AssetPacks>
) {
	gltf_register.unloaded.retain(|gltf_asset| {
		if let Some(gltf) = gltf_assets.get(&gltf_asset.gltf) {
			let mut asset_pack = AssetPack::default();

			gltf.named_scenes.iter().for_each(|(scene_name, scene)| {
				asset_pack.scenes.insert(scene_name.to_string(), scene.to_owned());
			});

			gltf.named_animations.iter().for_each(|(name, animation)| {
				asset_pack.animations.insert(name.to_string(), animation.to_owned());
			});

			asset_packs.asset_packs.insert(gltf_asset.asset.clone(), asset_pack);

			return false;
		}

		true
	});
}
