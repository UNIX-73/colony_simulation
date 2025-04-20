use bevy::prelude::*;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

use super::{
    MAX_STAT_VALUE,
    atributes::CreatureAttributes,
    modifiers::{CreatureModifiers, ModifierValues, StatTarget},
};

// Define los campos para poder referirnos a ellos fácilmente en los atributos
/// Este enum solo nombra cada stat base
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount)]
pub enum BaseStatType {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Perception,
    Willpower,
    Charisma,
    Creativity,
}

#[derive(Component, Debug, Clone, Default)]
pub struct BaseStatValue {
    pub total_value: f32,
    pub default_value: f32,
    pub non_default_value: f32,
    pub denied: bool,
}
impl BaseStatValue {
    pub fn new(default_value: f32, non_default_value: f32, denied: bool) -> Self {
        BaseStatValue {
            total_value: (default_value + non_default_value).clamp(0.0, MAX_STAT_VALUE as f32),
            default_value,
            non_default_value,
            denied,
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
#[require(CreatureModifiers, CreatureAttributes)]
pub struct CreatureBaseStats {
    stats: [BaseStatValue; BaseStatType::COUNT],
}
impl CreatureBaseStats {
    pub fn get(&self, stat: BaseStatType) -> &BaseStatValue {
        &self.stats[stat as usize]
    }

    pub fn get_mut(&mut self, stat: BaseStatType) -> &mut BaseStatValue {
        &mut self.stats[stat as usize]
    }

    pub fn get_weighted_value(&self, stat: BaseStatType, weight: f32) -> f32 {
        self.get(stat).total_value * weight
    }

    pub fn weighted_sum(&self, weights: &[(BaseStatType, f32)]) -> f32 {
        weights
            .iter()
            .map(|(stat, weight)| self.get_weighted_value(*stat, *weight))
            .sum()
    }

    pub fn resolve_base_stats(&mut self, mods: &ModifierValues) {
        for stat_type in BaseStatType::iter() {
            let mut default_value = 0.0_f32;
            let mut non_default_value = 0.0_f32;
            let mut denied = false;

            if let Some(adds) = mods.adds.get(&StatTarget::Base(stat_type)) {
                default_value += adds.default_values.unwrap_or(0.0);
                non_default_value += adds.non_default_values.unwrap_or(0.0);
            }
            if let Some(muls) = mods.muls.get(&StatTarget::Base(stat_type)) {
                default_value *= muls.default_values.map_or(1.0, |m| 1.0 + m);
                non_default_value *= muls.non_default_values.map_or(1.0, |m| 1.0 + m);
            }
            if mods.denies.contains(&StatTarget::Base(stat_type)) {
                denied = true;
            }

            *self.get_mut(stat_type) = BaseStatValue::new(default_value, non_default_value, denied);
        }
    }
}
