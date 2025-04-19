pub mod movement;
pub mod name;
pub mod stats;

use bevy::prelude::*;

use super::Object;

#[derive(Component, Default)]
#[require(Object, )]
pub struct Creature;
