use bevy::ecs::resource::Resource;
use chunks::{WorldChunks, layer::ChunkPos};

pub mod chunks;
pub mod surface;

///Width cargado de chunks al rededor de la camara, tiene que ser  impar para tener en cuenta el propio chunk de la camara
pub const SURROUNDING_AREA_WIDTH: usize = 3;

#[derive(Resource)]
pub struct WorldChunksManager {
    pub chunks: WorldChunks,
    pub rendered: [ChunkPos; SURROUNDING_AREA_WIDTH * SURROUNDING_AREA_WIDTH],
}
impl WorldChunksManager {
    pub fn new_testing(world_size: u16) -> Self {
        let mut starting_chunks =
            [ChunkPos::new(0, 0); SURROUNDING_AREA_WIDTH * SURROUNDING_AREA_WIDTH];
        let half_area = (SURROUNDING_AREA_WIDTH / 2) as i32;

        let mut i = 0;
        //+1 para añadir el chunk central
        for y in -half_area..half_area + 1 {
            for x in -half_area..half_area + 1 {
                starting_chunks[i] = ChunkPos::new(x, y);
                i += 1;
            }
        }

        Self {
            chunks: WorldChunks::generate_test_world(world_size),
            rendered: starting_chunks,
        }
    }
}
