use bevy::prelude::*;

use crate::components::grid::{grid_position::GridPosition, moving_to::MovingTo};

pub fn set_grid_transforms(mut query: Query<(&mut Transform, &GridPosition), Without<MovingTo>>) {
    for (mut transform, grid_pos) in &mut query {
        transform.translation = grid_pos.to_transform_position();
    }
}
