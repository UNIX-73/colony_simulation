use bevy::prelude::*;

use crate::systems::object::creature::stats::{resolve_creature_stats, update_modifier_timings};

pub struct CreatureStatsPlugin;
impl Plugin for CreatureStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_modifier_timings,
                resolve_creature_stats.after(update_modifier_timings),
            ),
        );
    }
}
