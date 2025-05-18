use bevy::prelude::*;

use crate::{
    components::{
        camera::CameraComponent,
        grid::{grid_height_offset::GridHeigthOffset, grid_position::GridPositionComponent},
    },
    resources::simulation_world::{ChunkPos, WorldChunks, chunks::CHUNK_SIZE},
};

pub fn render_world(
    mut query: Query<
        (
            &mut GridPositionComponent,
            &GridHeigthOffset,
            &mut Transform,
        ),
        With<CameraComponent>,
    >,
    world_chunks: Res<WorldChunks>,
) {

    
}
