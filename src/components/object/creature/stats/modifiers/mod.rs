pub mod definitions;

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use super::{atributes::definitions::AttributeType, base::BaseStatType};

// ===============================
// A qué stat se aplica el modificador
// ===============================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatTarget {
    Base(BaseStatType),     // Modifica un stat base (fuerza, percepción, etc.)
    Derived(AttributeType), // Modifica un atributo derivado (velocidad de movimiento, precisión, etc.) — TODO: Definir Attribute
}

// ===============================
// Tipo de modificación aplicada
// ===============================

#[derive(Debug, Clone)]
pub enum ModifierKind {
    Add(f32),      // Suma o resta un valor fijo (+3 fuerza, -10% percepción, etc.)
    Multiply(f32), // Multiplica el valor (x0.8 velocidad, x1.5 precisión, etc.)
    Deny,          // Desactiva completamente el stat (ej: paralizado)
}

// ===============================
// Modificación específica a un stat concreto
// ===============================

#[derive(Debug, Clone)]
pub struct SingleStatModifier {
    pub target: StatTarget, // Qué stat modifica
    pub kind: ModifierKind, // Tipo de operación
}

// ===============================
// Definición base de un modificador (puede modificar múltiples stats)
// ===============================

#[derive(Debug)]
pub struct StatModifierDef {
    pub name: &'static str, // Nombre ("Human", "Broken Leg", etc.)
    pub changes: &'static [SingleStatModifier], // Cambios que realiza
    pub default_duration: Option<f64>, // None = permanente
    pub default_modifier: bool, // Si es "default" (como raza) y no se muestra individualmente en UI
    pub requires: Option<&'static [StatModifierDef]>, // Requiere otros modifiers para poder existir
    pub excludes: Option<&'static [StatModifierDef]>, // Excluye otros modifiers de poder aplicarse a un creature
}

// ===============================
// Instancia activa de un modificador en una criatura
// ===============================
#[derive(Debug, Clone)]
pub struct StatModifier {
    pub def: &'static StatModifierDef, // Referencia a la definición base
    pub remaining: Option<f64>,        // Tiempo restante (si temporal)
}
impl StatModifier {
    pub fn new(definition: &'static StatModifierDef) -> Self {
        StatModifier {
            def: definition,
            remaining: definition.default_duration,
        }
    }
}

pub struct StatTargetValues {
    pub default_values: Option<f32>,
    pub non_default_values: Option<f32>,
}
pub struct ModifierValues {
    /// Para cada target, cuánto sumar (sum of all Add)
    pub adds: HashMap<StatTarget, StatTargetValues>,
    /// Para cada target, cuánto multiplicar (product of all Multiply)
    pub muls: HashMap<StatTarget, StatTargetValues>,
    /// Targets que han sido “denied” y deben quedarse a 0
    pub denies: HashSet<StatTarget>,
}

// ===============================
// Componente que guarda los modificadores activos en una criatura
// ===============================

#[derive(Component, Debug, Default)]
pub struct CreatureModifiers(pub Vec<StatModifier>);
impl CreatureModifiers {
    pub fn get_modifier_values(&self) -> ModifierValues {
        let mut modifier_values = ModifierValues {
            adds: HashMap::new(),
            muls: HashMap::new(),
            denies: HashSet::new(),
        };

        for modifier in &self.0 {
            let is_base = modifier.def.default_modifier;

            for change in modifier.def.changes {
                let target: &StatTarget = &change.target;

                match change.kind {
                    ModifierKind::Add(v) => {
                        let entry = modifier_values.adds.entry(target.clone()).or_insert(
                            StatTargetValues {
                                default_values: None,
                                non_default_values: None,
                            },
                        );

                        let slot = if is_base {
                            &mut entry.default_values
                        } else {
                            &mut entry.non_default_values
                        };

                        *slot = Some(slot.unwrap_or(0.0) + v);
                    }

                    ModifierKind::Multiply(v) => {
                        let entry = modifier_values.muls.entry(target.clone()).or_insert(
                            StatTargetValues {
                                default_values: Some(1.0),
                                non_default_values: Some(1.0),
                            },
                        );

                        let slot = if is_base {
                            &mut entry.default_values
                        } else {
                            &mut entry.non_default_values
                        };

                        *slot = Some(slot.unwrap_or(1.0) * v);
                    }

                    ModifierKind::Deny => {
                        modifier_values.denies.insert(target.clone());
                    }
                }
            }
        }

        modifier_values
    }
}
