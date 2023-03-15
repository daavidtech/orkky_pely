use pathfinding::prelude::bfs;

use crate::types::NavigationMeshComponent;
use crate::types::Point;

fn find_current_mesh<'a>(p: &Point, navigation_mesh_components: &'a Vec<NavigationMeshComponent>) -> &'a NavigationMeshComponent {
	navigation_mesh_components
		.iter()
		.find(|mesh| {
			p.x >= mesh.left_up.x && p.x <= mesh.right_down.x &&
			p.z >= mesh.left_up.z && p.z <= mesh.right_down.z
		}).unwrap()
}

fn is_connected_mesh(curr: &NavigationMeshComponent, mesh: &NavigationMeshComponent) -> bool {
	true
}

fn next_possible_points(p: &Point, navigation_mesh_components: &Vec<NavigationMeshComponent>) -> Vec<Point> {
	let current_mesh = find_current_mesh(p, navigation_mesh_components);

	let relevant_meshes = navigation_mesh_components
		.iter()
		.filter(|mesh| is_connected_mesh(current_mesh, mesh));

	let mut possible_points = vec![
		Point { x: p.x, z: p.z + 1, y: p.y },
		Point { x: p.x + 1, z: p.z + 1, y: p.y },
		Point { x: p.x + 1, z: p.z, y: p.y },
		Point { x: p.x + 1, z: p.z - 1, y: p.y },
		Point { x: p.x, z: p.z - 1, y: p.y },
		Point { x: p.x - 1, z: p.z - 1, y: p.y },
		Point { x: p.x - 1, z: p.z, y: p.y },
		Point { x: p.x - 1, z: p.z + 1, y: p.y },
	];

	possible_points
}

// DOTO handle multiple navigation meshes
pub fn find_path(
	mesh_components: &Vec<NavigationMeshComponent>,
	src: &Point,
	dst: &Point,
) -> Option<Vec<Point>> {
	let result = bfs(
		src, 
		|p| next_possible_points(p, mesh_components), 
		|p| *p == *dst
	);

	result
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn find_simple_straight_path() {
		let navigation_mesh = vec![
			NavigationMeshComponent {
				left_up: Point { x: 0, z: 0, y: 0 },
				right_down: Point { x: 5, z: 5, y: 0 },
				right_up: Point { x: 5, z: 0, y: 0 },
				left_down: Point { x: 0, z: 5, y: 0 },
			}
		];

		let src = Point { x: 1, z: 1, y: 0 };
		let target = Point { x: 4, z: 4, y: 0 };

		let path = find_path(&navigation_mesh, &src, &target);

		assert_eq!(path.unwrap(), vec![
			Point { x: 1, z: 1, y: 0 }, 
			Point { x: 2, z: 2, y: 0 }, 
			Point { x: 3, z: 3, y: 0 }, 
			Point { x: 4, z: 4, y: 0 }
		]);
	}

	#[test]
	fn find_path_to_another_mesh() {
		let navigation_mesh = vec![
			NavigationMeshComponent {
				left_up: Point { x: 0, z: 0, y: 0 },
				right_down: Point { x: 5, z: 5, y: 0 },
				right_up: Point { x: 5, z: 0, y: 0 },
				left_down: Point { x: 0, z: 5, y: 0 },
			},
			NavigationMeshComponent {
				left_up: Point { x: 0, z: 5, y: 0 },
				right_down: Point { x: 5, z: 10, y: 0 },
				right_up: Point { x: 5, z: 5, y: 0 },
				left_down: Point { x: 0, z: 10, y: 0 },
			}
		];

		let src = Point { x: 2, z: 3, y: 0 };
		let target = Point { x: 2, z: 7, y: 0 };

		let path = find_path(&navigation_mesh, &src, &target);

		assert_eq!(path.unwrap(), vec![
			Point { x: 2, z: 3, y: 0 },
			Point { x: 2, z: 4, y: 0 },
			Point { x: 2, z: 5, y: 0 },
			Point { x: 2, z: 6, y: 0 },
			Point { x: 2, z: 7, y: 0 }
		]);
	}

	#[test]
	fn find_path_around_object() {
		let navigation_mesh = vec![
			NavigationMeshComponent {
				left_up: Point { x: 0, z: 0, y: 0 },
				right_up: Point { x: 1, z: 0, y: 0 },
				left_down: Point { x: 0, z: 1, y: 0 },
				right_down: Point { x: 1, z: 1, y: 0 },
			},
			NavigationMeshComponent {
				left_up: Point { x: 1, z: 0, y: 0 },
				right_up: Point { x: 2, z: 0, y: 0 },
				left_down: Point { x: 1, z: 3, y: 0 },
				right_down: Point { x: 2, z: 3, y: 0 },
			},
			NavigationMeshComponent {
				left_up: Point { x: 0, z: 2, y: 0 },
				right_up: Point { x: 1, z: 2, y: 0 },
				left_down: Point { x: 0, z: 3, y: 0 },
				right_down: Point { x: 1, z: 3, y: 0 },
			},
		];

		let src = Point { x: 0, z: 0, y: 0 };
		let target = Point { x: 0, z: 3, y: 0 };

		let path = find_path(&navigation_mesh, &src, &target);

		assert_eq!(path.unwrap(), vec![
			Point { x: 0, z: 0, y: 0 },
			Point { x: 1, z: 1, y: 0 },
			Point { x: 1, z: 2, y: 0 },
			Point { x: 0, z: 3, y: 0 },
		]);
	}

	#[test]
	fn test_is_connected() {
		let curr = NavigationMeshComponent {
			left_up: Point { x: 0, z: 0, y: 0 },
			right_up: Point { x: 1, z: 0, y: 0 },
			left_down: Point { x: 0, z: 1, y: 0 },
			right_down: Point { x: 1, z: 1, y: 0 },
		};
		let mesh = NavigationMeshComponent {
			left_up: Point { x: 0, z: 1, y: 0 },
			right_up: Point { x: 1, z: 1, y: 0 },
			left_down: Point { x: 0, z: 2, y: 0 },
			right_down: Point { x: 2, z: 2, y: 0 },
		};

		assert_eq!(is_connected_mesh(&curr, &mesh), true);
	}

	#[test]
	fn test_is_not_connected() {
		let curr = NavigationMeshComponent {
			left_up: Point { x: 0, z: 0, y: 0 },
			right_up: Point { x: 1, z: 0, y: 0 },
			left_down: Point { x: 0, z: 1, y: 0 },
			right_down: Point { x: 1, z: 1, y: 0 },
		};
		let mesh = NavigationMeshComponent {
			left_up: Point { x: 0, z: 1, y: 0 },
			right_up: Point { x: 1, z: 1, y: 0 },
			left_down: Point { x: 0, z: 3, y: 0 },
			right_down: Point { x: 2, z: 3, y: 0 },
		};

		assert_eq!(is_connected_mesh(&curr, &mesh), false);
	}
}
