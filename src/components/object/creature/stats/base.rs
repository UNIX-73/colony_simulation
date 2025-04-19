use bevy::prelude::*;

use super::{atributes::CreatureAttributes, modifiers::CreatureModifiers};

#[derive(Component, Debug, Clone, Copy, Default)]
#[require(CreatureModifiers, CreatureAttributes)]
pub struct CreatureBaseStats {
    pub strength: f32,     // Capacidad bruta muscular
    pub dexterity: f32,    // Precisión y finura
    pub endurance: f32,    // Capacidad de aguantar actividad
    pub intelligence: f32, // Razonamiento, aprendizaje
    pub perception: f32,   // Agudeza sensorial
    pub willpower: f32,    // Fortaleza mental
    pub charisma: f32,     // Habilidad social
    pub creativity: f32,   // Innovación y adaptabilidad
}

// Define los campos para poder referirnos a ellos fácilmente en los atributos
/// Este enum solo nombra cada stat base
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
impl BaseStatType {
    pub fn get_mut<'a>(&self, stats: &'a mut CreatureBaseStats) -> &'a mut f32 {
        match self {
            BaseStatType::Strength => &mut stats.strength,
            BaseStatType::Dexterity => &mut stats.dexterity,
            BaseStatType::Endurance => &mut stats.endurance,
            BaseStatType::Intelligence => &mut stats.intelligence,
            BaseStatType::Perception => &mut stats.perception,
            BaseStatType::Willpower => &mut stats.willpower,
            BaseStatType::Charisma => &mut stats.charisma,
            BaseStatType::Creativity => &mut stats.creativity,
        }
    }
}
