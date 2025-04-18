pub mod name;

use bevy::prelude::*;

use super::Object;

#[derive(Component)]
#[require(Object)]
pub struct Creature;
