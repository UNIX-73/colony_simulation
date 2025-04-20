pub mod atributes;
pub mod base;
pub mod modifiers;

use atributes::CreatureAttributes;
use base::CreatureBaseStats;
use bevy::prelude::*;
use modifiers::{CreatureModifiers, StatModifier, definitions::HUMAN_MODIFIER};

#[derive(Component, Default)]
#[require(CreatureBaseStats, CreatureModifiers, CreatureAttributes)]
pub struct CreatureStats;

#[derive(Bundle)]
pub struct CreatureStatsBundle {
    pub base_stats: CreatureBaseStats,
    pub modifiers: CreatureModifiers,
    pub attributes: CreatureAttributes,
    pub stats: CreatureStats,
}
impl CreatureStatsBundle {
    pub fn human_bundle() -> Self {
        CreatureStatsBundle {
            base_stats: default(),
            modifiers: CreatureModifiers(vec![StatModifier::new(&HUMAN_MODIFIER)]),
            attributes: default(),
            stats: default(),
        }
    }
}
