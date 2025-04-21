use super::grid_position::{GridCellPosition, GridPosition};
use bevy::prelude::*;

/// Componente que define el movimiento autónomo en la cuadrícula
#[derive(Component)]
pub struct GridMovementComponent {
    pub speed: f32, // Cells/sec
    pub target: GridCellPosition,
}
impl GridMovementComponent {
    pub fn new(speed: f32, target: GridPosition) -> Self {
        Self {
            speed,
            target: GridCellPosition::new(target),
        }
    }

    pub fn new_with_fractional(speed: f32, target: GridCellPosition) -> Self {
        Self { speed, target }
    }
}
