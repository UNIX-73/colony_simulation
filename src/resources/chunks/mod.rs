pub mod collision;
pub mod environment_cell;

use bevy::prelude::*;
use collision::ChunkCollisions;
use environment_cell::{ChunkEnvironments, EnvironmentCellData};
use std::collections::HashMap;

pub const CHUNK_SIZE: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPosition(usize, usize);
impl ChunkPosition {
    #[inline]
    pub fn x(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn y(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn set_x(&mut self, new_x: usize) {
        self.0 = new_x;
    }

    #[inline]
    pub fn set_y(&mut self, new_y: usize) {
        self.1 = new_y;
    }
}
pub struct Chunk {
    // La posición la tenemos en el propio key del map
    pub collisions: ChunkCollisions, // Se guarda separado a enviroment porque no todos los chunks van a tener celdas con colisión
    pub environments: ChunkEnvironments, // Se guardan juntos varios valores porque todos los cells contienen un valor
}
impl Chunk {
    /// Crea un chunk vacío con valores por defecto.
    pub fn empty(default_env: EnvironmentCellData) -> Self {
        Self {
            collisions: ChunkCollisions(ChunkData::new(false)),
            environments: ChunkEnvironments(ChunkData::new(default_env)),
        }
    }
}

#[derive(Resource)]
pub struct ChunkMap {
    pub chunks: HashMap<ChunkPosition, Chunk>,
}

#[derive(Debug, Clone)]
pub struct ChunkData<T: Clone> {
    data: Vec<T>,
}
impl<T: Clone> ChunkData<T> {
    /// Construye un ChunkData rellenándolo con `default`. Necesita T: Clone.
    pub fn new(default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    /// Construye a partir de tu propio Vec, sin clones.
    pub fn from_vec(data: Vec<T>) -> Self {
        assert_eq!(data.len(), CHUNK_SIZE * CHUNK_SIZE);
        Self { data }
    }

    /// Construye a partir de una función `f(x,y) -> T`.
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE);
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                data.push(f(x, y));
            }
        }
        Self { data }
    }

    /// Calcula índice interno (row‑major).
    #[inline]
    fn index(cell: ChunkPosition) -> usize {
        debug_assert!(
            cell.x() < CHUNK_SIZE && cell.y() < CHUNK_SIZE,
            "Tried to access a chunk position outside of CHUNK_SIZE"
        );
        cell.y() * CHUNK_SIZE + cell.x()
    }

    /// Obtiene referencia inmutable. Devuelve None si está fuera de rango.
    #[inline]
    pub fn get(&self, cell: ChunkPosition) -> Option<&T> {
        self.data.get(Self::index(cell))
    }

    /// Obtiene referencia mutable.
    #[inline]
    pub fn get_mut(&mut self, cell: ChunkPosition) -> Option<&mut T> {
        self.data.get_mut(Self::index(cell))
    }
}
