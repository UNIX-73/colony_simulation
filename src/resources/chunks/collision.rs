use std::ops::{Deref, DerefMut};

use super::ChunkData;

#[derive(Debug, Clone)]
pub struct ChunkCollisions(pub ChunkData<bool>);

impl Deref for ChunkCollisions {
    type Target = ChunkData<bool>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ChunkCollisions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
