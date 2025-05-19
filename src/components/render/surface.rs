use bevy::ecs::component::Component;

use crate::components::grid::{
    grid_height_offset::GridHeigthOffset, grid_position::GridPositionComponent,
};

#[derive(Component)]
#[require(GridPositionComponent, GridHeigthOffset)]
pub struct SurfaceComponent;
