use bevy::prelude::*;
use std::fmt;

use crate::{
    resources::simulation_world::chunks::{
        CHUNK_SIZE,
        layer::{ChunkPos, chunk_data::ChunkCellPos},
    },
    utils::math::{div_floor, mod_floor},
};

use super::{GRID_FLOOR_HEIGHT, GRID_SIZE, grid_height_offset::GridHeigthOffset};

#[derive(Default, Clone, Copy)]

pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn new_from_chunk_pos(chunk: ChunkPos, cell: ChunkCellPos) -> Self {
        let x = chunk.x * CHUNK_SIZE as i32 + cell.x() as i32;
        let y = chunk.y * CHUNK_SIZE as i32 + cell.y() as i32;
        GridPosition { x, y }
    }

    /// Devuelve la posición del chunk al que pertenece esta celda
    pub fn get_chunk_pos(&self) -> ChunkPos {
        ChunkPos::new(
            div_floor(self.x, CHUNK_SIZE as i32),
            div_floor(self.y, CHUNK_SIZE as i32),
        )
    }

    /// Devuelve la posición interna en el chunk de esta celda
    pub fn get_chunk_cell_pos(&self) -> ChunkCellPos {
        let local_x = mod_floor(self.x, CHUNK_SIZE as i32) as usize;
        let local_y = mod_floor(self.y, CHUNK_SIZE as i32) as usize;
        ChunkCellPos::from_xy(local_x as u32, local_y as u32)
    }
}

#[derive(Default, Clone, Copy)]
pub struct FractionalGridPosition {
    pub x: f32, // de -0.5 a 0.5
    pub y: f32, // de -0.5 a 0.5
}
impl FractionalGridPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn clamp(&mut self) {
        self.x = self.x.clamp(-0.5, 0.5);
        self.y = self.y.clamp(-0.5, 0.5);
    }

    fn apply_direction(&mut self, dir: Vec2, speed: f32, delta_time: f32) {
        self.x += dir.x * speed * delta_time;
        self.y += dir.y * speed * delta_time;
    }
}

#[derive(Default, Clone)]
pub struct GridCellPosition {
    pub position: GridPosition,
    pub fractional: FractionalGridPosition,
}
impl GridCellPosition {
    pub fn new(position: GridPosition) -> Self {
        Self {
            position,
            fractional: default(),
        }
    }

    pub fn new_with_fractional(position: GridPosition, fractional: FractionalGridPosition) -> Self {
        Self {
            position,
            fractional,
        }
    }

    pub fn set_absolute(&mut self, x: i32, y: i32) {
        self.fractional = default();
        self.position.x = x;
        self.position.y = y;
    }

    pub fn set(&mut self, new_pos: GridCellPosition) {
        self.position = new_pos.position;
        self.fractional = new_pos.fractional;
    }

    pub fn set_by_components(
        &mut self,
        new_pos: GridPosition,
        new_fractional: FractionalGridPosition,
    ) {
        self.position = new_pos;
        self.fractional = new_fractional;
    }

    pub fn get_decimal_position(&self) -> Vec2 {
        Vec2 {
            x: self.position.x as f32 + self.fractional.x,
            y: self.position.y as f32 + self.fractional.y,
        }
    }

    pub fn to_transform_translation(&self, height_offset: &GridHeigthOffset) -> Vec3 {
        let base_x = self.position.x as f32 * GRID_SIZE;
        let base_z = self.position.y as f32 * GRID_SIZE;
        Vec3 {
            x: base_x + self.fractional.x * GRID_SIZE,
            y: GRID_FLOOR_HEIGHT + height_offset.get_offset(),
            z: base_z + self.fractional.y * GRID_SIZE,
        }
    }
    pub fn move_in_grid(&mut self, dir: Vec2, speed: f32, delta_time: f32) {
        self.fractional.apply_direction(dir, speed, delta_time);

        // Verificamos si hay que actualizar la posición en la cuadrícula
        if self.fractional.x > 0.5 {
            self.fractional.x -= 1.0;
            self.position.x += 1;
        } else if self.fractional.x < -0.5 {
            self.fractional.x += 1.0;
            self.position.x -= 1;
        }

        if self.fractional.y > 0.5 {
            self.fractional.y -= 1.0;
            self.position.y += 1;
        } else if self.fractional.y < -0.5 {
            self.fractional.y += 1.0;
            self.position.y -= 1;
        }
    }
}

#[derive(Component, Default)]
#[require(Transform, GridHeigthOffset)]
pub struct GridPositionComponent {
    pub cell_pos: GridCellPosition,
}
impl GridPositionComponent {
    pub fn new(position: GridPosition) -> Self {
        Self {
            cell_pos: GridCellPosition::new(position),
        }
    }
}
