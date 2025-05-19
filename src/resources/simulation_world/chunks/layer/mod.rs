pub mod chunk_data;
pub mod rle_layer;

use crate::utils::memory_size::MemorySize;

use bevy::platform::collections::HashMap;
use chunk_data::ChunkData;
use std::marker::PhantomData;

pub trait CellData: Clone + PartialEq + Default {}
impl<T: Clone + PartialEq + Default> CellData for T {}

pub trait ChunkLayerStorage<T: CellData> {
    /// Devuelve el valor en una posición específica, si existe.
    fn get(&self, x: usize, y: usize) -> Option<T>;

    /// Devuelve el valor en índice lineal (0..CHUNK_AREA).
    fn get_index(&self, idx: usize) -> Option<T>;

    /// Descomprime a un array plano de T.
    fn unzip(&self) -> ChunkData<T>;

    /// Comprime a el método de almacenamiento de T
    fn from_unzip(unzip: ChunkData<T>) -> Self;

    fn memory_usage(&self) -> MemorySize;

    /// Devuelve true si la capa es válida internamente (útil para compresión).
    fn is_valid(&self) -> bool;

    fn iter(&self) -> impl Iterator<Item = (usize, T)> + '_;
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct ChunkPos(i32, i32);
impl ChunkPos {
    pub fn new(x: i32, y: i32) -> ChunkPos {
        ChunkPos(x, y)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}

/// Struct general de chunks por capa
pub struct WorldLayerChunks<T: CellData, Chunk: ChunkLayerStorage<T>>(
    HashMap<ChunkPos, Chunk>,
    PhantomData<T>,
);

impl<T: CellData, Chunk: ChunkLayerStorage<T>> WorldLayerChunks<T, Chunk> {
    pub fn new() -> Self {
        Self(HashMap::new(), PhantomData)
    }

    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<&Chunk> {
        self.0.get(&chunk_pos)
    }

    pub fn get_chunk_mut(&mut self, chunk_pos: ChunkPos) -> Option<&mut Chunk> {
        self.0.get_mut(&chunk_pos)
    }

    pub fn unload_chunk(&mut self, chunk_pos: ChunkPos) {
        self.0.remove(&chunk_pos);
    }

    pub fn set_chunk(&mut self, chunk_pos: ChunkPos, chunk: ChunkData<T>) {
        self.0.insert(chunk_pos, Chunk::from_unzip(chunk));
    }

    /// Devuelve la cantidad de memoria usada por los chunks de capa (GB, MB, Bytes).
    pub fn memory_usage(&self) -> MemorySize {
        let mut memory = MemorySize::new(0);

        for (_, chunk) in self.0.iter() {
            let chunk_mem = chunk.memory_usage();

            memory = memory + chunk_mem;
        }

        memory
    }
}
