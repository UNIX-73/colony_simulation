use strum_macros::EnumIter;

use crate::components::object::creature::stats::{
    base::CreatureBaseStats,
    modifiers::{ModifierValues, StatTarget},
};

use super::AttributeValue;

/// 1️⃣ Define todos los atributos posibles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum AttributeType {
    WalkSpeed,
    SprintSpeed,
    ShootingAccuracy,
    MeleeAccuracy,
    MiningSpeed,
    ConstructionSpeed,
    HealingSpeed,
    VisionRange,
    MoraleRecovery,
}

/// 2️⃣ Métodos asociados: nombre y función de cálculo
impl AttributeType {
    pub fn name(self) -> &'static str {
        match self {
            AttributeType::WalkSpeed => "Walk Speed",
            AttributeType::SprintSpeed => "Sprint Speed",
            AttributeType::ShootingAccuracy => "Shooting Accuracy",
            AttributeType::MeleeAccuracy => "Melee Accuracy",
            AttributeType::MiningSpeed => "Mining Speed",
            AttributeType::ConstructionSpeed => "Construction Speed",
            AttributeType::HealingSpeed => "Healing Speed",
            AttributeType::VisionRange => "Vision Range",
            AttributeType::MoraleRecovery => "Morale Recovery",
        }
    }

    pub fn resolve(self, base: &CreatureBaseStats, mods: &ModifierValues) -> AttributeValue {
        // Cada caso llama a tu lógica específica,
        // aquí un par de ejemplos sencillos:
        let formula_value = match self {
            AttributeType::WalkSpeed => base.strength.total_value * 0.1 + base.endurance.total_value * 0.2,
            AttributeType::SprintSpeed => base.strength.total_value * 0.2 + base.endurance.total_value * 0.1,
            _ => 0.0,
        };

        let mut default_value = 0.0_f32;
        let mut non_default_value = 0.0_f32;
        let mut denied = false;

        if let Some(adds) = mods.adds.get(&StatTarget::Derived(self)) {
            default_value += adds.default_values.unwrap_or(0.0);
            non_default_value += adds.non_default_values.unwrap_or(0.0);
        }
        if let Some(muls) = mods.muls.get(&StatTarget::Derived(self)) {
            default_value *= muls.default_values.map_or(1.0, |m| 1.0 + m);
            non_default_value *= muls.non_default_values.map_or(1.0, |m| 1.0 + m);
        }
        if mods.denies.contains(&StatTarget::Derived(self)) {
            denied = true;
        }

        AttributeValue::new(default_value, non_default_value, formula_value, denied)
    }
}
