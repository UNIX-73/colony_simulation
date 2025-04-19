use bevy::prelude::*;

#[derive(Component)]
pub struct ObjectName<'a> {
    name: &'a str,
}
impl<'a> Default for ObjectName<'a> {
    fn default() -> Self {
        Self { name: "Unnamed" }
    }
}
