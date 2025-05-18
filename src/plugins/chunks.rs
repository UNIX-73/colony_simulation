use bevy::prelude::*;

use crate::{resources::simulation_world::WorldChunks, systems::render::render_world};

pub struct ChunksPlugin;
impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldChunks::generate_test_world());
        app.add_systems(Startup, render_world);
    }
}
