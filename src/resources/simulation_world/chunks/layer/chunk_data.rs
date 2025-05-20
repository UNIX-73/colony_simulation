use crate::resources::simulation_world::chunks::{CHUNK_AREA, CHUNK_SIZE};

use super::CellData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkCellPos {
    pub idx: usize,
}

impl ChunkCellPos {
    pub fn new(idx: usize) -> ChunkCellPos {
        ChunkCellPos { idx }
    }

    pub fn from_xy(x: u32, y: u32) -> ChunkCellPos {
        ChunkCellPos {
            idx: (y as usize) * CHUNK_SIZE + (x as usize),
        }
    }

    #[inline]
    pub fn x(&self) -> usize {
        self.idx % CHUNK_SIZE
    }

    #[inline]
    pub fn y(&self) -> usize {
        self.idx / CHUNK_SIZE
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

    /// Obtiene una referencia a la celda por posición
    #[inline]
    pub fn get_pos(&self, pos: ChunkCellPos) -> &T {
        debug_assert!(pos.idx < CHUNK_AREA);
        &self.0[pos.idx]
    }

    /// Obtiene una referencia mutable a la celda por posición
    #[inline]
    pub fn get_pos_mut(&mut self, pos: ChunkCellPos) -> &mut T {
        debug_assert!(pos.idx < CHUNK_AREA);
        &mut self.0[pos.idx]
    }

    /// Obtiene una referencia a la celda por índice lineal
    #[inline]
    pub fn get_idx(&self, idx: usize) -> &T {
        debug_assert!(idx < CHUNK_AREA);
        &self.0[idx]
    }

    /// Obtiene una referencia mutable a la celda por índice lineal
    #[inline]
    pub fn get_idx_mut(&mut self, idx: usize) -> &mut T {
        debug_assert!(idx < CHUNK_AREA);
        &mut self.0[idx]
    }

    /// Devuelve una referencia al array completo
    #[inline]
    pub fn data(&self) -> &[T; CHUNK_AREA] {
        &self.0
    }

    /// Devuelve una referencia mutable al array completo
    #[inline]
    pub fn data_mut(&mut self) -> &mut [T; CHUNK_AREA] {
        &mut self.0
    }
}
