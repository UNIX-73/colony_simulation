use enum_map::{Enum, EnumMap, enum_map};
use once_cell::sync::Lazy;
use strum_macros::{Display, EnumCount, EnumIter};

use super::{ModifierKind, SingleStatModifier, StatModifierDef, StatTarget};
use crate::components::object::creature::stats::base::BaseStatType;

// Definiciones estáticas de StatModifierDef
pub const HUMAN_DEF: StatModifierDef = StatModifierDef {
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
    default_duration: Some(10.0),
    default_modifier: true,
    requires: None,
    excludes: None,
};

// Enum de modificadores con derivación Enum para enum-map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount, Enum, Display)]
pub enum StatModifier {
    Human,
}

// Mapa estático que liga cada variante a su definición
static MODIFIER_DEFINITIONS: Lazy<EnumMap<StatModifier, &'static StatModifierDef>> =
    Lazy::new(|| {
        enum_map! {
            StatModifier::Human => &HUMAN_DEF,
        }
    });

impl StatModifier {
    #[inline]
    pub fn definition(self) -> &'static StatModifierDef {
        MODIFIER_DEFINITIONS[self]
    }
}
