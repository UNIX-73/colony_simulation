use bevy::prelude::*;
use bitflags::bitflags;

// ===============================
// Definición de Tags para atributos
// ===============================

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AttributeTag: u32 {
        const MOVEMENT = 0b00000001;
        const COMBAT   = 0b00000010;
        const WORK     = 0b00000100;
        const SENSORY  = 0b00001000;
        const MEDICINE = 0b00010000;
        const MORALE   = 0b00100000;
        // Puedes seguir añadiendo más si quieres...
    }
}

// ===============================
// Enum de todos los atributos posibles
// ===============================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute {
    // Movimiento
    WalkSpeed,
    SprintSpeed,

    // Combate
    ShootingAccuracy,
    MeleeAccuracy,
    ReloadSpeed,

    // Trabajo
    MiningSpeed,
    ConstructionSpeed,
    HarvestingSpeed,

    // Medicina
    HealingSpeed,

    // Sensorial
    VisionRange,
    HearingRange,

    // Moral
    MoraleRecovery,
}

// ===============================
// Métodos asociados a Attribute
// ===============================

impl Attribute {
    pub fn tags(&self) -> AttributeTag {
        match self {
            // Movimiento
            Attribute::WalkSpeed | Attribute::SprintSpeed => AttributeTag::MOVEMENT,

            // Combate
            Attribute::ShootingAccuracy | Attribute::MeleeAccuracy | Attribute::ReloadSpeed => {
                AttributeTag::COMBAT
            }

            // Trabajo
            Attribute::MiningSpeed | Attribute::ConstructionSpeed | Attribute::HarvestingSpeed => {
                AttributeTag::WORK
            }

            // Medicina
            Attribute::HealingSpeed => AttributeTag::MEDICINE,

            // Sensorial
            Attribute::VisionRange | Attribute::HearingRange => AttributeTag::SENSORY,

            // Moral
            Attribute::MoraleRecovery => AttributeTag::MORALE,
        }
    }
}

// ===============================
// Componente con los valores actuales de cada atributo
// ===============================

#[derive(Component, Debug, Default)]
pub struct CreatureAttributes {
    pub walk_speed: f32,
    pub sprint_speed: f32,
    pub shooting_accuracy: f32,
    pub melee_accuracy: f32,
    pub reload_speed: f32,
    pub mining_speed: f32,
    pub construction_speed: f32,
    pub harvesting_speed: f32,
    pub healing_speed: f32,
    pub vision_range: f32,
    pub hearing_range: f32,
    pub morale_recovery: f32,
}
