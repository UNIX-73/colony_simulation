pub mod compressed_layer;
pub mod layer;

use compressed_layer::CompressedChunkLayer;

use super::surface::SurfaceBlock;

pub const CHUNK_SIZE: usize = 64;
pub const CHUNK_AREA: usize = CHUNK_SIZE.pow(2);

pub struct WorldChunk {
    pub surface_layer: CompressedChunkLayer<SurfaceBlock>,
}
