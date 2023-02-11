use bevy::gltf::Gltf;
use bevy::prelude::*;

use crate::types::AssetPack;
use crate::types::AssetPacks;
use crate::types::GltfRegister;

pub fn unpack_gltf(
	mut gltf_register: ResMut<GltfRegister>,
	gltf_assets: Res<Assets<Gltf>>,
	mut asset_packs: ResMut<AssetPacks>
) {
	gltf_register.unloaded.retain(|gltf_asset| {
		if let Some(gltf) = gltf_assets.get(&gltf_asset.gltf) {
			let mut asset_pack = AssetPack { 
				gltf: gltf_asset.gltf.clone(),
				..Default::default()
			};

			gltf.scenes.iter().enumerate().for_each(|(index, scene)| {
				log::info!("{} unnamed scene {}", gltf_asset.asset, index);

				asset_pack.scenes.push(scene.clone());
			});

			gltf.named_scenes.iter().for_each(|(scene_name, scene)| {
				log::info!("{} named scene {}", gltf_asset.asset, scene_name);

				asset_pack.named_scenes.insert(scene_name.to_string(), scene.to_owned());
			});

			gltf.named_animations.iter().for_each(|(name, animation)| {
				log::info!("{} named animation {}", gltf_asset.asset, name);
				asset_pack.named_animations.insert(name.to_string(), animation.to_owned());
			});

			gltf.animations.iter().enumerate().for_each(|(index, animation)| {
				log::info!("{} unnamed animation {}", gltf_asset.asset, index);

				asset_pack.animations.push(animation.clone());
			});

			asset_packs.asset_packs.insert(gltf_asset.asset.clone(), asset_pack);

			return false;
		}

		true
	});
}
