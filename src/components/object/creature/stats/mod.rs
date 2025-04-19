pub mod atributes;
pub mod base;
pub mod modifiers;

use atributes::CreatureAttributes;
use base::CreatureBaseStats;
use bevy::prelude::*;
use modifiers::CreatureModifiers;

#[derive(Component)]
#[require(CreatureBaseStats, CreatureModifiers, CreatureAttributes)]
pub struct CreatureStats;

#[derive(Bundle)]
pub struct CreatureStatsBundle {
    pub base_stats: CreatureBaseStats,
    pub modifiers: CreatureModifiers,
    pub attributes: CreatureAttributes,
    pub stats: CreatureStats,
}
