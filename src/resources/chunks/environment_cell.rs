use std::ops::{Deref, DerefMut};

use super::ChunkData;

#[derive(Clone, Debug)]
pub struct EnvironmentCellData {
    pub temperature: f32,
    pub pressure: f32,
    pub oxygen_percentage: f32,
}

#[derive(Debug, Clone)]
pub struct ChunkEnvironments(pub ChunkData<EnvironmentCellData>);
impl Deref for ChunkEnvironments {
    type Target = ChunkData<EnvironmentCellData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ChunkEnvironments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
