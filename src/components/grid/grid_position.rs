use bevy::prelude::*;

pub const GRID_SIZE: f32 = 100.0;
pub const FLOOR_HEIGHT: f32 = 0.0;

#[derive(Component, Default)]
pub struct GridPosition {
    x: i32,
    y: i32,
}
impl GridPosition {
    pub fn to_transform_position(&self) -> Vec3 {
        return Vec3 {
            x: self.x as f32 * GRID_SIZE,
            y: FLOOR_HEIGHT,
            z: self.y as f32 * GRID_SIZE,
        };
    }
}
