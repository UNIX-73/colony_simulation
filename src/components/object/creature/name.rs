use bevy::prelude::*;

#[derive(Component)]
pub struct CreatureName {
   pub name: String,
}
impl Default for CreatureName {
    fn default() -> Self {
        Self {
            name: "Unnamed creature".to_string(),
        }
    }
}
