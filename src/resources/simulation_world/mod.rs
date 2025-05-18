use bevy::ecs::resource::Resource;
use rand::prelude::*;
use std::collections::HashMap;

use chunks::{WorldChunk, layer::ChunkLayer};
use surface::SurfaceBlock;

pub mod chunks;
pub mod surface;

pub const WORLD_SIZE: usize = 128;
pub const WORLD_AREA: usize = WORLD_SIZE.pow(2);

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct ChunkPos(pub i32, pub i32);

#[derive(Resource)]
pub struct WorldChunks(pub HashMap<ChunkPos, WorldChunk>);

impl WorldChunks {
    pub fn generate_test_world() -> Self {
        let mut loaded = HashMap::new();
        let mut rng = rand::rng();

        for cx in -2..=2 {
            for cz in -2..=2 {
                let surface_layer =
                    ChunkLayer::new_with_fn(|_x, _y| match rng.random_range(0..4) {
                        0 => SurfaceBlock::Dirt,
                        1 => SurfaceBlock::Granite,
                        2 => SurfaceBlock::Water,
                        _ => SurfaceBlock::Air,
                    })
                    .zip();

                let chunk = WorldChunk {
                    surface_layer,
                    // Añade otras capas aquí si las tienes
                };

                loaded.insert(ChunkPos(cx, cz), chunk);
            }
        }

        WorldChunks(loaded)
    }
}
