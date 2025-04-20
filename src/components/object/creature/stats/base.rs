use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{
    atributes::CreatureAttributes,
    modifiers::{CreatureModifiers, ModifierValues, StatTarget},
};

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
            total_value: default_value + non_default_value,
            default_value,
            non_default_value,
            denied,
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
#[require(CreatureModifiers, CreatureAttributes)]
pub struct CreatureBaseStats {
    pub strength: BaseStatValue,     // Capacidad bruta muscular
    pub dexterity: BaseStatValue,    // Precisión y finura
    pub endurance: BaseStatValue,    // Capacidad de aguantar actividad
    pub intelligence: BaseStatValue, // Razonamiento, aprendizaje
    pub perception: BaseStatValue,   // Agudeza sensorial
    pub willpower: BaseStatValue,    // Fortaleza mental
    pub charisma: BaseStatValue,     // Habilidad social
    pub creativity: BaseStatValue,   // Innovación y adaptabilidad
}
impl CreatureBaseStats {
    pub fn get(&self, stat_type: BaseStatType) -> BaseStatValue {
        match stat_type {
            BaseStatType::Strength => self.strength.clone(),
            BaseStatType::Dexterity => self.dexterity.clone(),
            BaseStatType::Endurance => self.endurance.clone(),
            BaseStatType::Intelligence => self.intelligence.clone(),
            BaseStatType::Perception => self.perception.clone(),
            BaseStatType::Willpower => self.willpower.clone(),
            BaseStatType::Charisma => self.charisma.clone(),
            BaseStatType::Creativity => self.creativity.clone(),
        }
    }
    pub fn get_as_ref_mut(&mut self, stat_type: BaseStatType) -> &mut BaseStatValue {
        match stat_type {
            BaseStatType::Strength => &mut self.strength,
            BaseStatType::Dexterity => &mut self.dexterity,
            BaseStatType::Endurance => &mut self.endurance,
            BaseStatType::Intelligence => &mut self.intelligence,
            BaseStatType::Perception => &mut self.perception,
            BaseStatType::Willpower => &mut self.willpower,
            BaseStatType::Charisma => &mut self.charisma,
            BaseStatType::Creativity => &mut self.creativity,
        }
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

            *self.get_as_ref_mut(stat_type) =
                BaseStatValue::new(default_value, non_default_value, denied);
        }
    }
}

// Define los campos para poder referirnos a ellos fácilmente en los atributos
/// Este enum solo nombra cada stat base
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
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
