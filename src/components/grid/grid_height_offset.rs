use bevy::prelude::*;


#[derive(Component)]
pub struct GridHeigthOffset {
    height_offset: Option<f32>,
}

impl GridHeigthOffset {
    pub fn new(offset: Option<f32>) -> Self {
        GridHeigthOffset {
            height_offset: offset,
        }
    }

    pub fn get_offset(&self) -> f32 {
        match self.height_offset {
            Some(offset) => offset,
            None => 0.0_f32,
        }
    }
}
impl Default for GridHeigthOffset {
    fn default() -> Self {
        GridHeigthOffset {
            height_offset: None,
        }
    }
}
