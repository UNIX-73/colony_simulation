use bevy::prelude::*;

use crate::components::grid::{
    grid_position::GridPositionComponent, movement::GridMovementComponent,
};

pub fn grid_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut GridPositionComponent, &GridMovementComponent)>,
) {
    for (entity, mut position, movement) in query.iter_mut() {
        let current = position.cell_pos.get_decimal_position();
        let target = movement.target.get_decimal_position();
        let delta = target - current;

        // Si ya estamos muy cerca, terminamos el movimiento
        if delta.length_squared() < 1e-4 {
            position.cell_pos.set(movement.target.clone());
            commands.entity(entity).remove::<GridMovementComponent>();
            continue;
        }

        let dir = delta.normalize();
        let step_distance = movement.speed * time.delta_secs();

        // Si nos pasamos del target con este paso, colocamos directamente en el objetivo
        if delta.length() <= step_distance {
            position.cell_pos.set(movement.target.clone());
            commands.entity(entity).remove::<GridMovementComponent>();
        } else {
            position
                .cell_pos
                .move_in_grid(dir, movement.speed, time.delta_secs());
        }
    }
}
