use crate::components::object::creature::stats::{
    CreatureStats,
    atributes::CreatureAttributes,
    base::CreatureBaseStats,
    modifiers::{CreatureModifiers, CreatureModifiersTimers},
};
use bevy::prelude::*;

pub fn update_modifier_timings(
    time: Res<Time>,
    mut query: Query<(&mut CreatureModifiers, &mut CreatureModifiersTimers)>,
) {
    let delta_s = time.delta_secs_f64();

    for (mut creature_mods, mut creature_timers) in &mut query {
        let expired: Vec<_> = creature_timers
            .0
            .iter_mut()
            .filter_map(|(&modifier, remaining)| {
                if modifier.definition().default_duration.is_some() {
                    *remaining -= delta_s;
                    if *remaining <= 0.0 {
                        return Some(modifier);
                    }
                }
                None
            })
            .collect();

        for m in expired {
            creature_timers.0.remove(&m);
            creature_mods.0.retain(|&x| x != m);
        }
    }
}

pub fn resolve_creature_stats(
    mut query: Query<
        (
            &mut CreatureBaseStats,
            &mut CreatureModifiers,
            &mut CreatureAttributes,
        ),
        (With<CreatureStats>, Changed<CreatureModifiers>),
    >,
) {
    for (mut base, modifiers, mut attributes) in &mut query {
        let mods = modifiers.get_modifier_values();
        base.resolve_base_stats(&mods);
        attributes.resolve_attributes(&base, &mods);

        println!("Modifiers changed")
    }
}
