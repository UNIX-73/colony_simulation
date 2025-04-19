use bevy::prelude::*;

use super::{grid_fractional_position::GridFractionalPosition, grid_position::GridPosition};

#[derive(Component)]
#[require(GridFractionalPosition)]
pub struct MovingTo {
    destination: GridPosition,
    progress: f32, //TODO: habra que implementar pathi
}
