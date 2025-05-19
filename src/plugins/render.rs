use crate::systems::render::render_world;
use bevy::prelude::*;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_world);
    }
}
