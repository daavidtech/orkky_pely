use std::f32::consts::PI;
use std::time::Duration;

use bevy::gltf::Gltf;
use bevy::gltf::GltfMesh;
use bevy::gltf::GltfNode;
use bevy::math;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::Collider;
use bevy_rapier3d::prelude::ColliderMassProperties;
use bevy_rapier3d::prelude::ComputedColliderShape;
use bevy_rapier3d::prelude::MassProperties;

use crate::types::AddCollidingMesh;
use crate::types::MeleeHitbox;

pub fn add_collisions(
	assets_gltf: Res<Assets<Gltf>>,
	assets_gltf_mesh: Res<Assets<GltfMesh>>,
	assets_gltf_nodes: Res<Assets<GltfNode>>,
	assets_mesh: Res<Assets<Mesh>>,
	query: Query<(Entity, &AddCollidingMesh)>,
	mut commands: Commands,
) {
	for (entity, add_collider_mesh) in query.iter() {
		log::info!("adding collision mesh for {:?}", entity);

		let pack = match assets_gltf.get(&add_collider_mesh.glft) {
			Some(pack) => {
				pack
			},
			None => continue,
		};

		let mut entity_commands = commands.entity(entity);

		for node in &pack.nodes {
			let node = match assets_gltf_nodes.get(node) {
				Some(n) => n,
				None => continue,
			};	

			log::info!("found node {:?}", node.transform);

			let mesh = match &node.mesh {
				Some(mesh) => mesh,
				None => continue,
			};

			let mesh = match assets_gltf_mesh.get(mesh) {
				Some(m) => m,
				None => continue,
			};

			log::info!("found mesh {:?}", mesh);

			for primite in &mesh.primitives {
				let mesh = assets_mesh.get(&primite.mesh).unwrap();

				let collider = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh);

				match collider {
					Some(collider) => {
						log::info!("found collider {:?}", collider);

						entity_commands.with_children(|parent| {
							parent.spawn((
								collider,
								TransformBundle {
									local: node.transform,
									..Default::default()
								}
							));
						});
					},
					None => {
						log::info!("mesh collider is invalid");
					}
				}
			}
		}

		commands.entity(entity).remove::<AddCollidingMesh>();
	}
}

pub struct MeleeHitboxTime {
	pub stopwatch: Stopwatch,
	pub spawned: bool,
}

pub fn move_melee_hitbox(
	mut commands: Commands,
	mut query: Query<(Entity, &mut Transform, &MeleeHitbox)>,
	time: Res<Time>,
	mut melee_hitbox_times: Local<HashMap<Entity, MeleeHitboxTime>>
) {
	for (entity, mut transform, hitbox) in query.iter_mut() {
		let hitbox_time = match melee_hitbox_times.get_mut(&entity) {
			Some(time) => time,
			None => {
				let time = MeleeHitboxTime {
					stopwatch: Stopwatch::new(),
					spawned: false,
				};

				melee_hitbox_times.insert(entity.clone(), time);

				continue;
			}
		};

		hitbox_time.stopwatch.tick(time.delta());
		let elapsed_seconds = hitbox_time.stopwatch.elapsed_secs();

		if elapsed_seconds < hitbox.delay {
			continue;
		}

		if elapsed_seconds - hitbox.delay > hitbox.dur {
			melee_hitbox_times.remove(&entity);
			commands.entity(entity).despawn_recursive();

			continue;
		}

		if hitbox_time.spawned == false {
			hitbox_time.spawned = true;

			let mut entity_commands = commands.entity(entity);

			entity_commands.with_children(|parent| {
				parent.spawn(
					(
						Collider::cuboid(0.5, 0.5, hitbox.radius / 2.0),
						ColliderMassProperties::Density(12.0),
						TransformBundle::from_transform(
							Transform {
								translation: Vec3::new(0.0, 0.0, hitbox.radius / 2.0),
								..Default::default()
							},
						)
					)
				);
			});

			transform.rotation = Quat::from_rotation_y((360.0 - hitbox.start_angle).to_radians());
		}

		let ratio = (elapsed_seconds - hitbox.delay) / hitbox.dur;

		let max_angle_change = if hitbox.end_angle > hitbox.start_angle {
			hitbox.end_angle - hitbox.start_angle
		} else {
			360.0 - hitbox.start_angle + hitbox.end_angle
		};

		let angle_change = max_angle_change * ratio;
		let new_angle = (hitbox.start_angle + angle_change) % 360.0;

		transform.rotation = Quat::from_rotation_y((90.0 + new_angle).to_radians());
	}
}
