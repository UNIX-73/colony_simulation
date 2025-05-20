use bevy::prelude::*;

use crate::{
    components::{
        camera::CameraComponent,
        grid::{
            grid_height_offset::GridHeigthOffset,
            grid_position::{GridPosition, GridPositionComponent},
        },
        render::surface::SurfaceComponent,
    },
    resources::simulation_world::{
        SURROUNDING_AREA_WIDTH, WorldChunksManager,
        chunks::{
            CHUNK_AREA, CHUNK_SIZE,
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
    blocks_query: Query<(Entity, &GridPositionComponent), With<SurfaceComponent>>,
    mut world_chunks: ResMut<WorldChunksManager>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Obtener posición de la cámara
    let camera_pos_comp = if let Ok(cam) = camera_query.single() {
        cam
    } else {
        eprintln!("No se encontró la cámara");
        return;
    };
    let cam_global = camera_pos_comp.cell_pos.position;

    // Área de renderizado centrada en la cámara
    let half = (SURROUNDING_AREA_WIDTH as isize) / 2;
    let mut idx = 0;

    // Cacheamos en vec en vez de hashmap porque probablemente no haya que cacheas mas de 3 o 4 chunks
    // pub struct ChunkData<T>([T; CHUNK_AREA])
    let mut cached_chunks: Vec<(ChunkPos, ChunkData<SurfaceBlock>)> = vec![];

    for dy in -half..=half {
        for dx in -half..=half {
            let x = cam_global.x + dx as i32;
            let y = cam_global.y + dy as i32;
            let cell_pos = GridPosition::new(x, y).get_chunk_cell_pos();
            let chunk_pos = GridPosition::new(x, y).get_chunk_pos();

            // Saltar si ya está renderizado
            if let Some(prev) = world_chunks.rendered[idx] {
                if prev.x == x && prev.y == y {
                    idx += 1;
                    continue;
                }
                // Si cambió, eliminar la entidad anterior
                if let Some((entity, _)) = blocks_query.iter().nth(idx) {
                    println!("despawned {}", entity);
                    commands.entity(entity).despawn();
                }
            }

            let block: SurfaceBlock;
            let cached_chunk = cached_chunks.iter().find(|ch| ch.0 == chunk_pos);

            if let Some((_, layer)) = cached_chunk {
                block = layer.get_pos(cell_pos).clone();
            } else {
                // No está en caché, cargar el chunk a caché
                let requested_chunk = world_chunks.chunks.surface_layer.get_chunk(chunk_pos);
                if let Some(chunk) = requested_chunk {
                    let unzip_data = chunk.unzip();

                    block = *unzip_data.get_pos(cell_pos);

                    let _ = cached_chunks.push((chunk_pos, unzip_data));
                } else {
                    //TODO: gestión de carga de chunks
                    continue;
                }
            }

            let mut color = (255, 255, 255);
            if let Some(block_color) = block.get_block_color() {
                color = block_color;
            }

            commands.spawn((
                GridPositionComponent::new(GridPosition::new_from_chunk_pos(chunk_pos, cell_pos)),
                GridHeigthOffset::new(Some(1.0)),
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb_u8(color.0, color.1, color.2))),
                Transform::default(), // Se ajusta en otro sistema del GridPositionComponent
            ));

            // Registrar posición para no rerender
            world_chunks.rendered[idx] = Some(GridPosition::new(x, y));
            idx += 1;
        }
    }
}
