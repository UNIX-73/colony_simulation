use crate::resources::simulation_world::chunks::CHUNK_AREA;

use super::layer::ChunkLayer;
use std::mem::MaybeUninit;

pub struct CellsRun<T> {
    pub id: T,
    pub count: u16,
}

pub struct CompressedChunkLayer<T: Default + Clone + PartialEq>(pub Vec<CellsRun<T>>);
impl<T: Default + Copy + PartialEq> CompressedChunkLayer<T> {
    pub fn assert_size(&self) -> bool {
        let mut size: u16 = 0;
        for run in &self.0 {
            size += run.count;
        }

        if size as usize == CHUNK_AREA {
            true
        } else {
            false
        }
    }

    pub unsafe fn unsafe_unzip(&self) -> ChunkLayer<T> {
        let mut raw: [MaybeUninit<T>; CHUNK_AREA] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut i = 0usize;

        for run in &self.0 {
            for _ in 0..run.count {
                raw[i].write(run.id.clone());
                i += 1;
            }
        }
        debug_assert!(i == CHUNK_AREA, "¡La descompresión no llenó todo el array!");

        let init_data: [T; CHUNK_AREA] =
            std::array::from_fn(|idx| unsafe { raw[idx].assume_init_read() });

        ChunkLayer(init_data)
    }

    pub fn unzip(&self) -> ChunkLayer<T> {
        assert!(self.assert_size());

        unsafe { self.unsafe_unzip() }
    }
}
