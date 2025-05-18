use strum_macros::{Display, EnumIter};

use crate::components::object::creature::stats::{
    base::{BaseStatType, CreatureBaseStats},
    modifiers::{ModifierValues, StatTarget},
};

use super::AttributeValue;

/// 1️⃣ Define todos los atributos posibles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, Display)]
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

    fn weights(&self) -> Option<&'static [(BaseStatType, f32)]> {
        match self {
            AttributeType::WalkSpeed => Some(&[
                (BaseStatType::Endurance, 0.8),
                (BaseStatType::Strength, 0.2),
            ]),
            AttributeType::SprintSpeed => Some(&[
                (BaseStatType::Endurance, 0.3),
                (BaseStatType::Strength, 0.7),
            ]),
            _ => None,
        }
    }

    pub fn resolve(self, base: &CreatureBaseStats, mods: &ModifierValues) -> AttributeValue {
        let weight_value = self
            .weights()
            .map_or(0.0, |weights| base.weighted_sum(weights));

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

        AttributeValue::new(default_value, non_default_value, weight_value, denied)
    }
}
