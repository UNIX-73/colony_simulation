use std::collections::HashSet;

use bevy::ecs::resource::Resource;
use chunks::WorldChunks;

use crate::components::grid::grid_position::GridPosition;

pub mod chunks;
pub mod surface;

///Width cargado de cells al rededor de la camara, TIENE QUE SER IMPAR
pub const RENDER_WIDTH: usize = 31;

#[derive(Resource)]
pub struct WorldChunksManager {
    pub chunks: WorldChunks,
    pub rendered: HashSet<GridPosition>,
}
impl WorldChunksManager {
    pub fn new_testing(world_size: u16) -> Self {
        Self {
            chunks: WorldChunks::generate_test_world(world_size),
            rendered: HashSet::new(),
        }
    }
}
