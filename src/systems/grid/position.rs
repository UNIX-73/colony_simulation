use bevy::prelude::*;

use crate::components::grid::{
    grid_fractional_position::GridFractionalPosition, grid_height_offset::GridHeigthOffset,
    grid_position::GridPosition,
};

// Para los objetos que no implementan el componente de posición fraccional
pub fn set_grid_transforms(
    mut query: Query<
        (&mut Transform, &GridPosition, &GridHeigthOffset),
        Without<GridFractionalPosition>,
    >,
) {
    for (mut transform, grid_pos, h_offset) in &mut query {
        transform.translation = grid_pos.to_transform_position_3d(h_offset);
    }
}

pub fn set_grid_fractional_transforms(
    mut query: Query<(
        &mut Transform,
        &GridPosition,
        &GridFractionalPosition,
        &GridHeigthOffset,
    )>,
) {
    for (mut transform, grid_pos, fractional_position, height_offset) in &mut query {
        transform.translation =
            grid_pos.to_fractional_transform_position_3d(fractional_position, height_offset)
    }
}
