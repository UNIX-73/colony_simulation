use bevy::ecs::resource::Resource;
use chunks::{WorldChunks, layer::ChunkPos};

use crate::components::grid::grid_position::{GridCellPosition, GridPosition};

pub mod chunks;
pub mod surface;

///Width cargado de cells al rededor de la camara
pub const SURROUNDING_AREA_WIDTH: usize = 3;
pub const SURROUNDING_AREA: usize = SURROUNDING_AREA_WIDTH.pow(2);

#[derive(Resource)]
pub struct WorldChunksManager {
    pub chunks: WorldChunks,
    pub rendered: [Option<GridPosition>; SURROUNDING_AREA],
}
impl WorldChunksManager {
    pub fn new_testing(world_size: u16) -> Self {
        Self {
            chunks: WorldChunks::generate_test_world(world_size),
            rendered: [None; SURROUNDING_AREA],
        }
    }
}
