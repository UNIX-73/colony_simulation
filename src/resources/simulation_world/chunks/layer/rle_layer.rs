use super::{CellData, ChunkData, ChunkLayerStorage};
use crate::{
    resources::simulation_world::chunks::{CHUNK_AREA, CHUNK_SIZE},
    utils::memory_size::MemorySize,
};
use std::mem::MaybeUninit;

pub struct RleRun<T: CellData> {
    pub id: T,
    pub count: u16,
}

pub struct RleChunkLayer<T: CellData>(Vec<RleRun<T>>);

impl<T: CellData> ChunkLayerStorage<T> for RleChunkLayer<T> {
    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<T> {
        self.get_index(y * CHUNK_SIZE + x)
    }

    /// Es ineficiente ya que recorre todos los vec hasta encontrar el correcto
    fn get_index(&self, idx: usize) -> Option<T> {
        if idx >= CHUNK_AREA {
            return None;
        }

        let mut i = 0usize;
        for run in &self.0 {
            if idx < i + run.count as usize {
                return Some(run.id.clone());
            }
            i += run.count as usize;
        }

        None
    }

    fn unzip(&self) -> ChunkData<T> {
        debug_assert!(self.is_valid());

        let mut raw: [MaybeUninit<T>; CHUNK_AREA] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut idx = 0_usize;
        for run in &self.0 {
            for _ in 0..run.count {
                raw[idx].write(run.id.clone());
                idx += 1;
            }
        }

        if idx != CHUNK_AREA {
            panic!(
                "RLE chunk size is {:0} when it should be {:1}",
                idx, CHUNK_AREA
            )
        }

        ChunkData::new(std::array::from_fn(|i| unsafe {
            raw[i].assume_init_read()
        }))
    }

    fn from_unzip(unzip: ChunkData<T>) -> Self {
        let mut data: Vec<RleRun<T>> = Vec::new();

        let mut iter = unzip.get().into_iter();
        let mut last_cell = iter.next().unwrap(); // asumimos que CHUNK_AREA > 0
        let mut count = 1;

        for cell in iter {
            if cell == last_cell {
                count += 1;
            } else {
                data.push(RleRun {
                    id: last_cell.clone(),
                    count,
                });
                last_cell = cell;
                count = 1;
            }
        }

        data.push(RleRun {
            id: last_cell.clone(),
            count,
        });

        Self(data)
    }

    fn memory_usage(&self) -> MemorySize {
        use std::mem::size_of;

        let run_size = size_of::<RleRun<T>>();
        let vec_len = self.0.len();
        let vec_base = size_of::<Vec<RleRun<T>>>(); // típicamente 24 bytes

        let total = vec_base + vec_len * run_size;

        let bytes = total % (1024 * 1024);

        MemorySize::new(bytes as u64)
    }

    fn is_valid(&self) -> bool {
        let total: usize = self.0.iter().map(|r| r.count as usize).sum();
        total == CHUNK_AREA
    }

    fn iter(&self) -> impl Iterator<Item = (usize, T)> + '_ {
        let mut index = 0;
        self.0.iter().flat_map(move |run| {
            let start = index;
            index += run.count as usize;
            (start..index).map(move |i| (i, run.id.clone()))
        })
    }
}
