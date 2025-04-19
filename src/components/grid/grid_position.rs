use bevy::prelude::*;

use super::{
    GRID_FLOOR_HEIGHT, GRID_SIZE, grid_fractional_position::GridFractionalPosition,
    grid_height_offset::GridHeigthOffset,
};

#[derive(Component, Default)]
#[require(Transform, GridHeigthOffset)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn to_transform_position_3d(&self, height_offset: &GridHeigthOffset) -> Vec3 {
        return Vec3 {
            x: self.x as f32 * GRID_SIZE,
            y: GRID_FLOOR_HEIGHT + height_offset.get_offset(),
            z: self.y as f32 * -GRID_SIZE, // Para que visualmente el axis coincida con la posicion visual ya que bevy no mirrorea la camara
        };
    }

    pub fn to_fractional_transform_position_3d(
        &self,
        fractional_position: &GridFractionalPosition,
        height_offset: &GridHeigthOffset,
    ) -> Vec3 {
        let grid_pos = self.to_transform_position_3d(height_offset);
        let offset = fractional_position.get_transform_offset();

        Vec3 {
            x: grid_pos.x + offset.x,
            y: grid_pos.y,
            z: grid_pos.z + offset.y,
        }
    }

    pub fn to_grid_fractional_position(
        &self,
        fractional_position: &GridFractionalPosition,
    ) -> Vec2 {
        Vec2 {
            x: self.x as f32 + fractional_position.x,
            y: self.y as f32 + fractional_position.y,
        }
    }
}
