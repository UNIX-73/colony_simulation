use bevy::prelude::*;

use super::{GRID_FLOOR_HEIGHT, GRID_SIZE, grid_position::GridPosition};

#[derive(Component, Default)]
#[require(GridPosition)]
pub struct GridFractionalPosition {
    pub x: f32, // Offset horizontal
    pub y: f32, // Offset en profundidad (z visual)
}

impl GridFractionalPosition {
    pub fn get_transform_offset(&self) -> Vec2 {
        Vec2 {
            x: self.x * GRID_SIZE,
            y: self.y * -GRID_SIZE,
        }
    }

    pub fn get_transform_offset_3d(&self, height_offset: f32) -> Vec3 {
        Vec3 {
            x: self.x * GRID_SIZE,
            y: height_offset + GRID_FLOOR_HEIGHT,
            z: self.y * -GRID_SIZE,
        }
    }
}
