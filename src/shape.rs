use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::map::*;

pub fn spawn_shape(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	shape: MapShape
) {
	log::info!("spawning shape: {:?}", shape);

	match shape.shape {
		MapShapeType::Cube(cube) => {
			commands.spawn(
				PbrBundle {
					mesh: meshes.add(Mesh::from(shape::Cube { size: cube.size })),
					..Default::default()
				}
			);
		},
		MapShapeType::Plane(plane) => {
			log::info!("spawning plane {:?}", plane);
			
			let mut plane_bundle = PbrBundle {
				mesh: meshes.add(Mesh::from(shape::Plane { size: plane.size })),
				material: materials.add(
					StandardMaterial {
						base_color: Color::rgb(0.3, 0.5, 0.3),
						..Default::default()
					}
				),
				..Default::default()
			};

			// if let Some(material) = plane.material {
			// 	plane_bundle.material = materials.add(Color::hex(material).unwrap().into());
			// }

			if let Some(location) = plane.location {
				plane_bundle.transform = Transform::from_xyz(location[0], location[1], location[2]);
			}

			commands.spawn(plane_bundle);
		},
		MapShapeType::Quad(quad) => {
			commands.spawn(
				PbrBundle {
					mesh: meshes.add(Mesh::from(shape::Quad { 
						size: Vec2::from_slice(&quad.size),
						..Default::default() 
					})),
					..Default::default()
				}
			);
		},
		MapShapeType::Circle(circle) => {
			commands.spawn(
				PbrBundle {
					mesh: meshes.add(Mesh::from(shape::Circle {
						radius: circle.radius,
						vertices: match circle.vertices {
							Some(vertices) => vertices,
							None => 32
						},
						..Default::default()
					})),
					..Default::default()
				}
			);
		},
		MapShapeType::Box(box_shape) => {
			let mut entity_commands = commands.spawn(
				PbrBundle {
					mesh: meshes.add(Mesh::from(shape::Box {
						min_x: box_shape.min_x,
						min_y: box_shape.min_y,
						min_z: box_shape.min_z,
						max_x: box_shape.max_x,
						max_y: box_shape.max_y,
						max_z: box_shape.max_z,
						..Default::default()
					})),
					material: materials.add(StandardMaterial {
						base_color: Color::hex("E6EED6").unwrap(),
						..default()
					}),
					..Default::default()
				}
			);

			if let Some(true) = box_shape.collider {
				let hx = (box_shape.max_x - box_shape.min_x) / 2.0;
				let hy = (box_shape.max_y - box_shape.min_y) / 2.0;
				let hz = (box_shape.max_z - box_shape.min_z) / 2.0;

				log::info!("spawning box collider: {:?}", (hx, hy, hz));

				entity_commands.insert((
					Collider::cuboid(hx, hy, hz),
					RigidBody::Fixed
				));
			}
		},
	}
}
