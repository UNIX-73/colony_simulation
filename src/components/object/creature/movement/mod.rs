pub mod destination;

use bevy::prelude::*;

use crate::components::grid::grid_fractional_position::GridFractionalPosition;

#[derive(Component, Default)]
#[require(GridFractionalPosition)]
pub struct CreatureMovement {
    walking_speed: f32,
    sprinting_speed: f32,
    swimming_speed: f32,
    flying_speed: f32,
}
