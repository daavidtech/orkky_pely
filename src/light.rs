use bevy::prelude::*;

use crate::map::*;

pub fn spawn_map_light(
	commands: &mut Commands,
	light: Light
) {
	match light.light_type {
		LightType::Point(point) => {
			log::info!("Spawning point light: {:?}", point);

			let mut light_bundle = PointLightBundle {
				point_light: PointLight {
					color: Color::hex(point.color).unwrap(),
					..Default::default()
				},
				..Default::default()
			};

			if let Some(intensity) = point.intensity {
				light_bundle.point_light.intensity = intensity;
			}

			if let Some(range) = point.range {
				light_bundle.point_light.range = range;
			}

			if let Some(radius) = point.radius {
				light_bundle.point_light.radius = radius;
			}

			if let Some(shadows_enabled) = point.shadows_enabled {
				light_bundle.point_light.shadows_enabled = shadows_enabled;
			}

			if let Some(location) = point.location {
				light_bundle.transform = Transform::from_xyz(location[0], location[1], location[2]);
			}

			commands.spawn(light_bundle);
		}
	}
}
