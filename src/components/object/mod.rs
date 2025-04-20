pub mod creature;
pub mod name;

use bevy::prelude::*;
use name::ObjectName;

use super::grid::grid_position::GridPositionComponent;

#[derive(Component, Default)]
#[require(ObjectName, GridPositionComponent)]
pub struct Object;
