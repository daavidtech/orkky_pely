use bevy::prelude::*;

use crate::types::GameEntity;
use crate::types::Attacking;


pub fn handle_attack(
	mut commands: Commands,
	mut query: Query<(Entity, &mut GameEntity, &mut Attacking)>,
	time: Res<Time>,
) {
	for (entity, mut game_entity, mut attacking) in query.iter_mut() {
		attacking.timer.tick(time.delta());

		if attacking.timer.finished() {
			game_entity.attacking = false;

			let mut entity_commands = commands.entity(entity);
			entity_commands.remove::<Attacking>();
		}
	}
}
