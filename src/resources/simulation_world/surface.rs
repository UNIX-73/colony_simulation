use rand::Rng;
use strum::EnumCount;
use strum_macros::{Display, EnumCount};

#[repr(u16)]
#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy, Display, EnumCount)]
pub enum SurfaceBlock {
    Air,
    Water,
    Dirt,
    Granite,
    // Más en un futuro
}
impl SurfaceBlock {
    pub fn new_random() -> SurfaceBlock {
        let index = rand::rng().random_range(0..SurfaceBlock::COUNT);

        unsafe { std::mem::transmute(index as u16) }
    }
}
impl Default for SurfaceBlock {
    fn default() -> Self {
        Self::Dirt
    }
}
