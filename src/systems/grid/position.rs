use bevy::prelude::*;

use crate::components::grid::{
    grid_height_offset::GridHeigthOffset, grid_position::GridPositionComponent,
};

pub fn set_grid_transforms(
    mut query: Query<(&mut Transform, &GridPositionComponent, &GridHeigthOffset)>,
) {
    for (mut transform, grid_pos_component, height_offset) in &mut query {
        transform.translation = grid_pos_component.to_transform_translation(height_offset);
    }
}
