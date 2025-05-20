use layer::{
    ChunkPos, WorldLayerChunks,
    chunk_data::{ChunkCellPos, ChunkData},
    rle_layer::RleChunkLayer,
};

use crate::utils::memory_size::MemorySize;

use super::surface::SurfaceBlock;

pub mod layer;

pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_AREA: usize = CHUNK_SIZE.pow(2);

pub struct WorldChunks {
    pub surface_layer: WorldLayerChunks<SurfaceBlock, RleChunkLayer<SurfaceBlock>>,
}
impl WorldChunks {
    pub fn new() -> WorldChunks {
        WorldChunks {
            surface_layer: WorldLayerChunks::new(),
        }
    }

    pub fn generate_test_world(size: u16) -> WorldChunks {
        let mut world = Self::new();

        let half_size = size as i32 / 2;
        let mut surface_layer =
            WorldLayerChunks::<SurfaceBlock, RleChunkLayer<SurfaceBlock>>::new();

        let mut chunk_data: ChunkData<SurfaceBlock> =
            ChunkData::new([SurfaceBlock::Air; CHUNK_AREA]);

        let mut idx = 0_usize;
        for cell in chunk_data.get_mut().iter_mut() {
            let cell_pos = ChunkCellPos::new(idx);

            let border = [cell_pos.x(), cell_pos.y()];
            if border.contains(&0_usize) || border.contains(&(CHUNK_SIZE - 1)) {
                *cell = SurfaceBlock::Water;
            } else {
                *cell = SurfaceBlock::new_random();
            }

            idx += 1;
        }

        for y in -half_size..=half_size {
            for x in -half_size..=half_size {
                surface_layer.set_chunk(ChunkPos::new(x, y), chunk_data.clone());
            }
        }

        world.surface_layer = surface_layer;
        world
    }

    pub fn memory_usage(&self) -> MemorySize {
        self.surface_layer.memory_usage()
    }
}
