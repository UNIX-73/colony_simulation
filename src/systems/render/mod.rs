use std::collections::HashSet;

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
        RENDER_WIDTH, WorldChunksManager,
        chunks::layer::{ChunkLayerStorage, ChunkPos, chunk_data::ChunkData},
        surface::SurfaceBlock,
    },
};

pub fn render_surface(
    camera_query: Query<
        &GridPositionComponent,
        (With<CameraComponent>, Changed<GridPositionComponent>),
    >,
    blocks_query: Query<(Entity, &GridPositionComponent), With<SurfaceComponent>>,
    mut world_chunks: ResMut<WorldChunksManager>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    debug_assert!(RENDER_WIDTH % 2 != 0);
    // Obtener posición de la cámara
    let camera_pos_comp = if let Ok(cam) = camera_query.single() {
        cam
    } else {
        return;
    };
    let cam_global = camera_pos_comp.cell_pos.position;

    // Área de renderizado centrada en la cámara
    let half = (RENDER_WIDTH as isize) / 2;

    // Cacheamos en vec en vez de hashmap porque probablemente no haya que cacheas mas de 3 o 4 chunks
    // pub struct ChunkData<T>([T; CHUNK_AREA])
    let mut cached_chunks: Vec<(ChunkPos, ChunkData<SurfaceBlock>)> = vec![];

    // Cálculo de las posiciones en area visible
    let mut current_visible_positions = HashSet::new();
    for dy in -half..=half {
        for dx in -half..=half {
            current_visible_positions.insert(GridPosition::new(
                cam_global.x + dx as i32,
                cam_global.y + dy as i32,
            ));
        }
    }

    // Primero despawn de los que ya no están en el área visible
    for (entity, grid_pos_comp) in blocks_query.iter() {
        let pos = grid_pos_comp.cell_pos.position;
        if !current_visible_positions.contains(&pos) {
            commands.entity(entity).despawn();
            world_chunks.rendered.remove(&pos);
        }
    }

    for pos in current_visible_positions.iter() {
        if world_chunks.rendered.contains(&pos) {
            continue;
        }

        let x = pos.x;
        let y = pos.y;

        let cell_pos = GridPosition::new(x, y).get_chunk_cell_pos();
        let chunk_pos = GridPosition::new(x, y).get_chunk_pos();

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

        /*
           Establecer los componentes
        */
        let grid_pos_comp =
            GridPositionComponent::new(GridPosition::new_from_chunk_pos(chunk_pos, cell_pos));
        let grid_h_offset = GridHeigthOffset::new(Some(1.0));
        let mut new_translation = grid_pos_comp
            .cell_pos
            .to_transform_translation(&grid_h_offset);

        new_translation.z = -new_translation.z;
        let mut transform = Transform::default();
        transform.translation = new_translation;

        commands.spawn((
            SurfaceComponent,
            grid_pos_comp,
            grid_h_offset,
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(color.0, color.1, color.2))),
            transform, // Se ajusta en otro sistema del GridPositionComponent
        ));

        // Registrar posición para no rerender
        world_chunks.rendered.insert(*pos);
    }
}
