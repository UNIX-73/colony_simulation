use bevy::prelude::*;

use crate::components::grid::grid_position::GridPosition;

use super::CreatureMovement;

#[derive(Component)]
#[require(CreatureMovement)]
pub struct CreatureDestination {
    pub destination_cell: GridPosition,
}
