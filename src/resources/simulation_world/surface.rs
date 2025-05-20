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

    pub fn get_block_color(&self) -> Option<(u8, u8, u8)> {
        match self {
            SurfaceBlock::Air => None,
            SurfaceBlock::Water => Some((51, 204, 255)),
            SurfaceBlock::Dirt => Some((153, 102, 51)),
            SurfaceBlock::Granite => Some((153, 153, 102)),
        }
    }
}
impl Default for SurfaceBlock {
    fn default() -> Self {
        Self::Dirt
    }
}
