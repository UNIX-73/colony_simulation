pub mod definitions;

use bevy::prelude::*;

use super::{atributes::Attribute, base::BaseStatType};

// ===============================
// A qué stat se aplica el modificador
// ===============================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatTarget {
    Base(BaseStatType), // Modifica un stat base (fuerza, percepción, etc.)
    Derived(Attribute), // Modifica un atributo derivado (velocidad de movimiento, precisión, etc.) — TODO: Definir Attribute
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
    pub name: &'static str,               // Nombre ("Human", "Broken Leg", etc.)
    pub changes: Vec<SingleStatModifier>, // Cambios que realiza
    pub default_duration: Option<f64>,    // None = permanente
    pub base_modifier: bool, // Si es "base" (como raza) y no se muestra individualmente en UI
}

// ===============================
// Instancia activa de un modificador en una criatura
// ===============================

#[derive(Debug, Clone)]
pub struct StatModifier {
    pub def: &'static StatModifierDef, // Referencia a la definición base
    pub remaining: Option<f64>,        // Tiempo restante (si temporal)
}

// ===============================
// Componente que guarda los modificadores activos en una criatura
// ===============================

#[derive(Component, Debug, Default)]
pub struct CreatureModifiers(pub Vec<StatModifier>);
