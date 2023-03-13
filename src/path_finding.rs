use pathfinding::prelude::bfs;

use crate::types::NavigationMeshComponent;
use crate::types::Point;

fn next_possible_points(p: &Point) -> Vec<Point> {
	vec![
		Point { x: p.x, z: p.z + 1, y: p.y },
		Point { x: p.x + 1, z: p.z + 1, y: p.y },
		Point { x: p.x + 1, z: p.z, y: p.y },
		Point { x: p.x + 1, z: p.z - 1, y: p.y },
		Point { x: p.x, z: p.z - 1, y: p.y },
		Point { x: p.x - 1, z: p.z - 1, y: p.y },
		Point { x: p.x - 1, z: p.z, y: p.y },
		Point { x: p.x - 1, z: p.z + 1, y: p.y },
	]
}

// DOTO handle multiple navigation meshes
pub fn find_path(
	navigation_mesh_components: &Vec<NavigationMeshComponent>,
	src: &Point,
	dst: &Point,
) -> Option<Vec<Point>> {
	let result = bfs(
		src, 
		|p| next_possible_points(p), 
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
}
