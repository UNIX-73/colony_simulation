use bevy::prelude::*;

use crate::components::grid::{
    grid_height_offset::GridHeigthOffset, grid_position::GridPositionComponent,
};

pub fn set_grid_transforms(
    mut query: Query<
        (
            &mut Transform,
            &mut GridPositionComponent,
            &mut GridHeigthOffset,
        ),
        (Changed<GridPositionComponent>,),
    >,
) {
    for (mut transform, grid_pos_component, height_offset) in &mut query {
        let mut new_translation = grid_pos_component
            .cell_pos
            .to_transform_translation(&height_offset);

        // Conversión de z porque si no el axis visual del grid(y) no corresponde con el visual
        new_translation.z = -new_translation.z;
        transform.translation = new_translation;
    }
}
