use bevy::prelude::*;

use super::grid_position::GridPosition;

#[derive(Component)]
#[require(GridPosition)]
pub struct MovingTo {
    destination: GridPosition,
    progress: f32, //TODO: habra que implementar pathi
}
