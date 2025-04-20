use crate::components::object::creature::stats::{
    CreatureStats, atributes::CreatureAttributes, base::CreatureBaseStats,
    modifiers::CreatureModifiers,
};
use bevy::prelude::*;

pub fn update_modifier_timings(time: Res<Time>, mut query: Query<&mut CreatureModifiers>) {
    let delta_s = time.delta_secs_f64();

    for mut modifiers in &mut query {
        modifiers.0.retain_mut(|modifier| {
            if let Some(remaining) = &mut modifier.remaining {
                *remaining -= delta_s;

                *remaining > 0.0 //Si es false se elimina del vector, si no se mantiene
            } else {
                true // Se mantiene porque es permanente
            }
        })
    }
}

pub fn resolve_creature_stats(
    mut query: Query<
        (
            &mut CreatureBaseStats,
            &CreatureModifiers,
            &mut CreatureAttributes,
        ),
        With<CreatureStats>,
    >,
) {
    for (mut base, modifiers, mut attributes) in &mut query {
        let modifier_values = modifiers.get_modifier_values();

        attributes.resolve_attributes(&base, &modifier_values);
    }
}
