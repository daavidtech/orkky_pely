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

pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl NavigationMeshComponent {
    pub fn aabb(&self) -> AABB {
        let min_x = self.left_up.x.min(self.right_down.x).min(self.right_up.x).min(self.left_down.x);
        let max_x = self.left_up.x.max(self.right_down.x).max(self.right_up.x).max(self.left_down.x);

        let min_y = self.left_up.y.min(self.right_down.y).min(self.right_up.y).min(self.left_down.y);
        let max_y = self.left_up.y.max(self.right_down.y).max(self.right_up.y).max(self.left_down.y);

        let min_z = self.left_up.z.min(self.right_down.z).min(self.right_up.z).min(self.left_down.z);
        let max_z = self.left_up.z.max(self.right_down.z).max(self.right_up.z).max(self.left_down.z);

        AABB {
            min: Point {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            max: Point {
                x: max_x,
                y: max_y,
                z: max_z,
            },
        }
    }
}

pub fn aabb_collision(a: &AABB, b: &AABB) -> bool {
    a.min.x <= b.max.x && a.max.x >= b.min.x
        && a.min.y <= b.max.y && a.max.y >= b.min.y
        && a.min.z <= b.max.z && a.max.z >= b.min.z
}

pub fn shapes_collision(a: &NavigationMeshComponent, b: &NavigationMeshComponent) -> bool {
    let aabb_a = a.aabb();
    let aabb_b = b.aabb();
    aabb_collision(&aabb_a, &aabb_b)
}
	



fn next_possible_points(p: &Point, navigation_mesh_components: &Vec<NavigationMeshComponent>) -> Vec<Point> {
	let current_mesh = find_current_mesh(p, navigation_mesh_components);

	let relevant_meshes = navigation_mesh_components
		.iter()
		.filter(|mesh| shapes_collision(current_mesh, mesh));

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

		assert_eq!(shapes_collision(&curr, &mesh), true);
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
			left_up: Point { x: 0, z: 2, y: 0 },
			right_up: Point { x: 2, z: 2, y: 0 },
			left_down: Point { x: 0, z: 3, y: 0 },
			right_down: Point { x: 3, z: 3, y: 0 },
		};

		assert_eq!(shapes_collision(&curr, &mesh), false);
	}
}

