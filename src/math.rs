use std::f32::consts::PI;

pub fn compute_new_angle(
	last_yaw: f32,
	x_delta: f32,
	sensitivity: f32,
) -> f32 {
	let delta = x_delta * sensitivity;
	let new_yaw = last_yaw + delta;
	let new_yaw = new_yaw % (2.0*PI);

	if new_yaw < 0.0 {
		2.0*PI + new_yaw
	} else {
		new_yaw
	}
}

// pub fn compute_new_pitch(
// 	last_pitch: f32,
// 	y_delta: f32,
// 	sensitivity: f32,
// ) -> f32 {
// 	let delta = y_delta * sensitivity;
// 	let new_pitch = last_pitch + delta;
// 	let new_pitch = new_pitch % (2.0*PI);

// 	if new_pitch < 0.0 {
// 		2.0*PI + new_pitch
// 	} else {
// 		new_pitch
// 	}
// }

pub fn rotate_vec(
	x: f32,
	y: f32,
	theta: f32,
) -> (f32, f32) {
	let cos_theta = theta.cos();
	let sin_theta = theta.sin();

	let new_x = cos_theta * x - sin_theta * y;
	let new_y = sin_theta * x + cos_theta * y;

	(new_x, new_y)
}

#[cfg(test)]
mod tests {
	use approx::assert_ulps_eq;

	use super::*;

	#[test]
	fn rotate_90_deg_clockwise() {
		let angle = 90.0_f32.to_radians();
		
		let x = 0.0;
		let y = -1.0;

		let (new_x, new_y) = rotate_vec(x, y, angle);

		assert_ulps_eq!(new_x, 1.0, epsilon = 0.0001);
		assert_ulps_eq!(new_y, 0.0, epsilon = 0.0001);
	}

	// #[test]
	// fn rotate_90_deg_counter_clockwise() {
	// 	let angle = -90.0_f32;
		
	// 	let x = 0.0;
	// 	let y = 1.0;

	// 	let (new_x, new_y) = rotate_vec(x, y, angle);

	// 	assert_ulps_eq!(new_x, -1.0, epsilon = 0.0001);
	// 	assert_ulps_eq!(new_y, 0.0, epsilon = 0.0001);
	// }

	#[test]
	fn test_compute_new_yaw_deg() {
		let last_yaw = 0.0;
		let x_delta = 90.0_f32.to_radians();
		let sensitivity = 1.0;
		let new_yaw = compute_new_angle(last_yaw, x_delta, sensitivity);
		assert_eq!(new_yaw, 90.0_f32.to_radians());
	
		let last_yaw = 0.0;
		let x_delta = -90.0_f32.to_radians();
		let sensitivity = 1.0;
		let new_yaw = compute_new_angle(last_yaw, x_delta, sensitivity);
		assert_eq!(new_yaw, 270.0_f32.to_radians());
	}

	#[test]
	fn test_compute_new_yaw_many_times() {
		let yaw = 0.0;
		let x_delta = 90.0_f32.to_radians();
		let sensitivity = 1.0;
		let yaw = compute_new_angle(yaw, x_delta, sensitivity);
		assert_eq!(yaw, 90.0_f32.to_radians());
		let yaw = compute_new_angle(yaw, x_delta, sensitivity);
		assert_eq!(yaw, 180.0_f32.to_radians());
	}
}
