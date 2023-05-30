use bevy::prelude::*;
use bevy::utils::FloatOrd;
use bevy_rapier3d::prelude::*;

use crate::*;
use crate::types::BulletProperties;
use crate::types::GameAssets;
use crate::types::Lifetime;
use crate::types::Target;
use crate::types::Tower;




pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
				commands
					.spawn(SceneBundle {
						scene: bullet_assets.bullet_scene.clone(),
						transform: Transform::from_translation(transform.translation()),
						..Default::default()
					})
					.insert(Lifetime {
						timer: Timer::from_seconds(30.0, TimerMode::Once),
					})
					.insert(Collider::ball(0.1))
					.insert(RigidBody::Dynamic)
					.insert(BulletProperties {
						damage: 5.0,
					})
					.insert(Name::new("Bullet"))
					.insert(ActiveEvents::COLLISION_EVENTS)
					.insert(Velocity {
						linvel: direction.normalize() * 50.0,
						..Default::default()
					});
            }
        }
    }
}


