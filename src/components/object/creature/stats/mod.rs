pub mod atributes;
pub mod base;
pub mod modifiers;

use atributes::CreatureAttributes;
use base::CreatureBaseStats;
use bevy::prelude::* ;
use modifiers::{CreatureModifiers, CreatureModifiersTimers, definitions::StatModifier};

pub const MAX_STAT_VALUE: i32 = 250;

#[derive(Component, Default)]
#[require(
    CreatureBaseStats,
    CreatureModifiers,
    CreatureModifiersTimers,
    CreatureAttributes
)]
pub struct CreatureStats;
impl CreatureStats {
    pub fn new(
        modifiers: &[StatModifier],
    ) -> (
        Self,
        CreatureBaseStats,
        CreatureModifiers,
        CreatureAttributes,
        CreatureModifiersTimers,
    ) {
        let (modifiers, timers) = CreatureModifiers::new_with_timers(modifiers);

        let mut base = CreatureBaseStats::default();
        let mut attributes = CreatureAttributes::default();

        let values = &modifiers.get_modifier_values();
        base.resolve_base_stats(values);
        attributes.resolve_attributes(&base, values);

        (CreatureStats, base, modifiers, attributes, timers)
    }
}
