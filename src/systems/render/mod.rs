use bevy::prelude::*;

use crate::{
    components::{
        camera::CameraComponent,
        grid::{grid_height_offset::GridHeigthOffset, grid_position::GridPositionComponent},
        render::surface::SurfaceComponent,
    },
    resources::simulation_world::{
        chunks::{layer::ChunkPos, WorldChunks},
        WorldChunksManager, SURROUNDING_AREA_WIDTH,
    },
};

pub fn render_world(
    camera_query: Query<&GridPositionComponent, With<CameraComponent>>,
    blocks_query: Query<
        (Entity, &GridPositionComponent, &GridHeigthOffset),
        With<SurfaceComponent>,
    >,
    world_chunks: Res<WorldChunksManager>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    debug_assert!(SURROUNDING_AREA_WIDTH % 2 != 0);

    let camera_position: &GridPositionComponent;
    if let Ok(camera) = camera_query.single() {
        camera_position = camera;
    } else {
        println!("No camera found");
        return;
    }

    let camera_chunk = camera_position.cell_pos.get_chunk_pos();

    let mut surrounding_chunks = [camera_chunk; SURROUNDING_AREA_WIDTH * SURROUNDING_AREA_WIDTH];

    let half_area = (SURROUNDING_AREA_WIDTH / 2) as i32;

    let mut i = 0;
    //+1 para añadir el chunk central
    for y in -half_area..half_area + 1 {
        for x in -half_area..half_area + 1 {
            surrounding_chunks[i] = ChunkPos::new(x + camera_chunk.x(), y + camera_chunk.y());
            i += 1;
        }
    }

    for row in 0..SURROUNDING_AREA_WIDTH {
        let start = row * SURROUNDING_AREA_WIDTH;
        let end = start + SURROUNDING_AREA_WIDTH;
        let row_slice = &surrounding_chunks[start..end];
        println!("{:?}", row_slice);
    }

    for chunk_pos in surrounding_chunks {}
}
