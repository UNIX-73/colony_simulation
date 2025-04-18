pub mod creature;
pub mod name;

use bevy::prelude::*;
use name::ObjectName;

use super::grid::grid_position::GridPosition;

#[derive(Component, Default)]
#[require(ObjectName, GridPosition)]
pub struct Object;
