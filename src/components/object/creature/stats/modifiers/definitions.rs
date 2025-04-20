use super::{ModifierKind, SingleStatModifier, StatModifierDef, StatTarget};
use crate::components::object::creature::stats::base::BaseStatType;

pub const HUMAN_MODIFIER: StatModifierDef = StatModifierDef {
    name: "Human",
    changes: &[
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Strength),
            kind: ModifierKind::Add(5.0),
        },
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Dexterity),
            kind: ModifierKind::Add(5.0),
        },
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Endurance),
            kind: ModifierKind::Add(6.0),
        },
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Intelligence),
            kind: ModifierKind::Add(10.0),
        },
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Perception),
            kind: ModifierKind::Add(5.0),
        },
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Willpower),
            kind: ModifierKind::Add(7.0),
        },
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Charisma),
            kind: ModifierKind::Add(6.0),
        },
        SingleStatModifier {
            target: StatTarget::Base(BaseStatType::Creativity),
            kind: ModifierKind::Add(8.0),
        },
    ],
    default_duration: None, // Permanente
    default_modifier: true, // Base modifier
    requires: None,
    excludes: None,
};
