pub mod name;
pub mod stats;

use bevy::prelude::*;
use stats::CreatureStats;

use super::Object;

#[derive(Component, Default)]
#[require(Object, CreatureStats)]
pub struct Creature;
