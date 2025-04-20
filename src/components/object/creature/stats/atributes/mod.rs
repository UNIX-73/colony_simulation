pub mod definitions;

use std::fmt::{self, Display, Formatter};

use bevy::prelude::*;
use definitions::AttributeType;
use strum::IntoEnumIterator;

use super::{base::CreatureBaseStats, modifiers::ModifierValues};

#[derive(Clone)]
pub struct AttributeValue {
    pub total_value: f32,
    pub default_value: f32,
    pub non_default_value: f32,
    pub weigthed_value: f32, // Viene del cálculo de fórmula custom que tiene cada atributo
    pub denied: bool,
}
impl AttributeValue {
    pub fn new(
        default_value: f32,
        non_default_value: f32,
        weigthed_value: f32,
        denied: bool,
    ) -> Self {
        AttributeValue {
            total_value: default_value + non_default_value + weigthed_value,
            default_value,
            non_default_value,
            weigthed_value,
            denied,
        }
    }
}
impl Display for AttributeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Total: {:.2}, Default: {:.2}, Non-default: {:.2}, Weighted: {:.2}, Denied: {}",
            self.total_value,
            self.default_value,
            self.non_default_value,
            self.weigthed_value,
            self.denied
        )
    }
}
pub struct Attribute {
    attribute_type: AttributeType,
    value: AttributeValue,
}
impl Attribute {
    pub fn get_type(&self) -> AttributeType {
        self.attribute_type
    }
    pub fn get_value(&self) -> AttributeValue {
        self.value.clone()
    }
    pub fn update_value(&mut self, base: &CreatureBaseStats, mods: &ModifierValues) {
        self.value = self.attribute_type.resolve(base, mods);
    }
}

#[derive(Component, Default)]
pub struct CreatureAttributes {
    atributtes: Vec<Attribute>,
}
impl CreatureAttributes {
    pub fn get(&self) -> &Vec<Attribute> {
        &self.atributtes
    }

    pub fn resolve_attributes(&mut self, base: &CreatureBaseStats, mods: &ModifierValues) {
        self.atributtes = vec![];
        for attribute in AttributeType::iter() {
            let value = attribute.resolve(base, mods);

            if value.total_value > 0.0 || value.denied {
                self.atributtes.push(Attribute {
                    attribute_type: attribute.clone(),
                    value,
                });
            }
        }
    }
}
