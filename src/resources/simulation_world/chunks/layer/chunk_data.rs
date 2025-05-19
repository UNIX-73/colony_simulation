use crate::{
    components::grid::grid_position::GridPosition,
    resources::simulation_world::chunks::{CHUNK_AREA, CHUNK_SIZE},
};

use super::CellData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkCellPos(pub usize, pub usize);
impl ChunkCellPos {
    /// Convierte un índice lineal a posición de celda (x, y) dentro del chunk
    pub fn from_idx(idx: usize) -> ChunkCellPos {
        let x = idx % CHUNK_SIZE;
        let y = idx / CHUNK_SIZE;
        ChunkCellPos(x, y)
    }

    pub fn as_grid_position(&self) -> GridPosition {
        GridPosition {
            x: self.0 as i32,
            y: self.1 as i32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData<T: CellData>([T; CHUNK_AREA]);

impl<T: CellData> ChunkData<T> {
    #[inline]
    pub fn get(&self) -> &[T; CHUNK_AREA] {
        &self.0
    }
    #[inline]
    pub fn get_mut(&mut self) -> &mut [T; CHUNK_AREA] {
        &mut self.0
    }

    pub fn new(data: [T; CHUNK_AREA]) -> Self {
        Self(data)
    }

    /// Convierte posición (x, y) a índice lineal
    pub fn to_idx(pos: ChunkCellPos) -> usize {
        pos.1 * CHUNK_SIZE + pos.0
    }

    /// Obtiene una referencia a la celda en el índice lineal
    pub fn get_idx(&self, idx: usize) -> &T {
        debug_assert!(idx < CHUNK_AREA);
        &self.0[idx]
    }

    pub fn get_idx_mut(&mut self, idx: usize) -> &mut T {
        debug_assert!(idx < CHUNK_AREA);
        &mut self.0[idx]
    }

    /// Obtiene una referencia a la celda en la posición (x, y)
    pub fn get_pos(&self, pos: ChunkCellPos) -> &T {
        let idx = Self::to_idx(pos);
        self.get_idx(idx)
    }

    /// Obtiene una referencia mutable a la celda en la posición (x, y)
    pub fn get_pos_mut(&mut self, pos: ChunkCellPos) -> &mut T {
        let idx = Self::to_idx(pos);
        debug_assert!(idx < CHUNK_AREA);
        &mut self.0[idx]
    }

    /// Devuelve una referencia al array completo
    pub fn data(&self) -> &[T; CHUNK_AREA] {
        &self.0
    }

    /// Devuelve una referencia mutable al array completo
    pub fn data_mut(&mut self) -> &mut [T; CHUNK_AREA] {
        &mut self.0
    }
}
