use crate::resources::simulation_world::{
    RENDER_WIDTH, WorldChunksManager,
    chunks::{WorldChunks, layer::ChunkPos},
};
use bevy::prelude::*;

pub struct ChunksPlugin;
impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldChunksManager::new_testing(200));
    }
}
