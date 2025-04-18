use bevy::prelude::*;

#[derive(Component)]
pub struct ObjectName {
    name: String,
}
impl Default for ObjectName {
    fn default() -> Self {
        Self {
            name: "Unnamed".to_string(),
        }
    }
}
