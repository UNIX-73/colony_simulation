use std::array;

use super::{
    CHUNK_AREA, CHUNK_SIZE,
    compressed_layer::{CellsRun, CompressedChunkLayer},
};

pub struct ChunkLayer<T: Default + Clone + PartialEq>(pub [T; CHUNK_AREA]);

impl<T: Default + Clone + PartialEq> ChunkLayer<T> {
    /// Crea un nuevo ChunkLayer con todos los valores por defecto.
    pub fn new() -> Self {
        Self(array::from_fn(|_| T::default()))
    }

    pub fn new_with_fn<F>(mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = array::from_fn(|_| T::default());
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                data[y * CHUNK_SIZE + x] = f(x, y);
            }
        }
        Self(data)
    }

    /// Obtiene una referencia a la celda en el índice plano (0..CHUNK_AREA).
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    /// Obtiene una referencia mutable a la celda en el índice plano.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    /// Asigna un valor a una celda si el índice es válido.
    #[inline]
    pub fn set(&mut self, index: usize, value: T) {
        if let Some(cell) = self.0.get_mut(index) {
            *cell = value;
        }
    }

    #[inline]
    pub fn get_idx(x: usize, y: usize) -> usize {
        y * CHUNK_SIZE + x
    }

    #[inline]
    pub fn get_pos(&self, x: usize, y: usize) -> Option<T> {
        self.0.get(Self::get_idx(x, y)).cloned()
    }

    /// Devuelve una referencia mutable al valor en la posición `(x, y)`.
    #[inline]
    pub fn get_pos_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.0.get_mut(Self::get_idx(x, y))
    }

    /// Itera sobre todas las celdas (por índice y valor).
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.0.iter().enumerate()
    }

    /// Itera de forma mutable.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.0.iter_mut().enumerate()
    }

    /// Llena todo el layer con un valor.
    pub fn fill(&mut self, value: T) {
        for cell in &mut self.0 {
            *cell = value.clone();
        }
    }

    /// Devuelve un puntero crudo al array (por ejemplo, para acceso C).
    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    /// Devuelve un puntero mutable crudo al array.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }

    /// Devuelve una referencia a todo el array.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    /// Devuelve una referencia mutable a todo el array.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }

    pub fn zip(&self) -> CompressedChunkLayer<T> {
        let mut data: Vec<CellsRun<T>> = Vec::new();

        let mut iter = self.0.iter();

        // Primer elemento, o defecto si está vacío (normalmente no debería)
        let mut last_value = match iter.next() {
            Some(v) => v.clone(),
            None => return CompressedChunkLayer(vec![]),
        };

        let mut count: u16 = 1;

        for val in iter {
            if *val == last_value {
                count = count.saturating_add(1);
            } else {
                // Guardar el run anterior
                data.push(CellsRun {
                    id: last_value.clone(),
                    count,
                });
                last_value = val.clone();
                count = 1;
            }
        }

        // Guardar el último run
        data.push(CellsRun {
            id: last_value,
            count,
        });

        CompressedChunkLayer(data)
    }
}
