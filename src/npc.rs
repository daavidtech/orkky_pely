use bevy::prelude::App;
use bevy::prelude::Plugin;
use crate::types::*;



use bevy::prelude::*;


pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_targets)
            .add_system(target_death);
    }
}

fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}

fn move_targets(mut commands: Commands, mut npc: Query<(Entity, &TargetPosition, &mut Transform)>, time: Res<Time>) {
    for (entity, target, mut transform) in &mut npc {    
        let y = 0.0;
        let x = target.x - transform.translation.x;
        let z = target.z - transform.translation.z;
         
        let speed = 10.0;
        let distance = Vec3::new(x, y, z);
        let distance_abs = distance.abs();
        let deltadistace = speed * time.delta_seconds();


        transform.translation += distance.normalize() * deltadistace;

        if distance_abs.x < 0.1 && distance_abs.y < 0.1 {
            
            let mut entity_commands = commands.entity(entity);
            entity_commands.remove::<TargetPosition>();
		}      
    }
}

 
pub fn handle_cycle(
	mut commands: Commands,
	mut npcs: Query<(Entity, &mut MoveCycle, &GameEntity), Without<TargetPosition>>,
) {
	for (entity, mut cycle, game_entity) in &mut npcs {
		let mut entity_commands = commands.entity(entity);

		let point = cycle.get_next();

		log::info!("[{}] Moving to {:?}", game_entity.entity_id, point);

		let target_position = TargetPosition {
			x: point.x as f32,
			z: point.z as f32,
		};

		entity_commands.insert(target_position);
	}
}
