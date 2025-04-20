use bevy::prelude::*;
use std::fmt;

use super::{GRID_FLOOR_HEIGHT, GRID_SIZE, grid_height_offset::GridHeigthOffset};

#[derive(Default)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}
impl fmt::Display for GridPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Default)]
pub struct FractionalGridPosition {
    pub x: f32, // de -0.5 a 0.5
    pub y: f32, // de -0.5 a 0.5
}
impl FractionalGridPosition {
    pub fn clamp(&mut self) {
        self.x = self.x.clamp(-0.5, 0.5);
        self.y = self.y.clamp(-0.5, 0.5);
    }

    fn apply_direction(&mut self, dir: Vec2, speed: f32, delta_time: f32) {
        self.x += dir.x * speed * delta_time;
        self.y += dir.y * speed * delta_time;
    }
}

#[derive(Component, Default)]
#[require(Transform, GridHeigthOffset)]
pub struct GridPositionComponent {
    pub position: GridPosition,
    pub fractional: FractionalGridPosition,
}
impl GridPositionComponent {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: GridPosition { x, y },
            fractional: FractionalGridPosition::default(),
        }
    }

    pub fn new_with_fractional(x: i32, y: i32, f_x: f32, f_y: f32) -> Self {
        Self {
            position: GridPosition { x, y },
            fractional: FractionalGridPosition { x: f_x, y: f_y },
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

    // Método para mover la posición en la cuadrícula
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
