use bevy::prelude::*;

use crate::{
    components::{
        camera::CameraComponent,
        grid::{
            grid_height_offset::GridHeigthOffset,
            grid_position::{GridPosition, GridPositionComponent},
        },
        render::surface::SurfaceComponent,
        scene::SceneComponent,
    },
    resources::simulation_world::{
        SURROUNDING_AREA_WIDTH, WorldChunksManager,
        chunks::{
            CHUNK_AREA, WorldChunks,
            layer::{
                ChunkLayerStorage, ChunkPos,
                chunk_data::{ChunkCellPos, ChunkData},
                rle_layer::RleChunkLayer,
            },
        },
        surface::SurfaceBlock,
    },
};

pub fn render_surface(
    camera_query: Query<&GridPositionComponent, With<CameraComponent>>,
    blocks_query: Query<
        (Entity, &GridPositionComponent, &GridHeigthOffset),
        With<SurfaceComponent>,
    >,
    mut world_chunks: ResMut<WorldChunksManager>,
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

    for chunk_pos in surrounding_chunks {
        if !world_chunks.rendered.contains(&Some(chunk_pos)) {
            let default_chunk =
                &RleChunkLayer::from_unzip(ChunkData::new([SurfaceBlock::Air; CHUNK_AREA]));

            let chunk = world_chunks
                .chunks
                .surface_layer
                .get_chunk(chunk_pos)
                .unwrap_or(default_chunk);

            for (idx, cell) in chunk.iter() {
                let pos = ChunkCellPos::from_idx(idx);

                commands.spawn((
                    GridPositionComponent::new(pos.as_grid_position()),
                    GridHeigthOffset::new(Some(1.0)),
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 0))),
                    Transform::from_xyz(0.0, 2.0, 0.0),
                ));
            }
        }
    }

    world_chunks.rendered = surrounding_chunks.map(|c| Some(c));
}
