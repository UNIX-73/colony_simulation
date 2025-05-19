use bevy::ecs::resource::Resource;
use chunks::{WorldChunks, layer::ChunkPos};

pub mod chunks;
pub mod surface;

///Width cargado de chunks al rededor de la camara, tiene que ser  impar para tener en cuenta el propio chunk de la camara
pub const SURROUNDING_AREA_WIDTH: usize = 3;
pub const SURROUNDING_AREA: usize = SURROUNDING_AREA_WIDTH.pow(2);

#[derive(Resource)]
pub struct WorldChunksManager {
    pub chunks: WorldChunks,
    pub rendered: [Option<ChunkPos>; SURROUNDING_AREA],
}
impl WorldChunksManager {
    pub fn new_testing(world_size: u16) -> Self {
        let mut starting_chunks = [ChunkPos::new(0, 0); SURROUNDING_AREA];

        Self {
            chunks: WorldChunks::generate_test_world(world_size),
            rendered: [None; SURROUNDING_AREA],
        }
    }
}
