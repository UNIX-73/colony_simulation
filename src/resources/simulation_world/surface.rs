#[repr(u16)]
#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub enum SurfaceBlock {
    Air,
    Water,
    Dirt,
    Granite,
    // Más en un futuro
}
impl Default for SurfaceBlock {
    fn default() -> Self {
        Self::Dirt
    }
}
