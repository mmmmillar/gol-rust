mod utils;

use ca_formats::rle::Rle;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

extern crate js_sys;
extern crate web_sys;

const NEIGHBOURS: &'static [(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    depth: i32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn new(width: i32, depth: i32, pattern: &str) -> Universe {
        utils::set_panic_hook();

        let size = (width * depth) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        let mut positions = Vec::new();

        Rle::new(pattern).unwrap().for_each(|cell| {
            let (x, y) = cell.unwrap().position;
            positions.push((y * width as i64 + x) as usize)
        });

        for i in 0..size {
            match positions.is_empty() {
                false => cells.set(i, positions.contains(&i)),
                _ => cells.set(i, js_sys::Math::random() < 0.5),
            }
        }

        Universe {
            width,
            depth,
            cells,
        }
    }

    pub fn live_neighbour_count(&self, x: i32, y: i32) -> usize {
        NEIGHBOURS
            .iter()
            .map(|&(nx, ny)| self.get_index((x + nx) % self.width, (y + ny) % self.depth))
            .filter(|&i| self.cells[i])
            .count()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.depth {
                let index = self.get_index(x, y);
                match (self.cells[index], self.live_neighbour_count(x, y)) {
                    (true, c) if c == 2 || c == 3 => next.set(index, true),
                    (false, c) if c == 3 => next.set(index, true),
                    _ => next.set(index, false),
                }
            }
        }

        self.cells = next;
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.depth
    }
}

impl Universe {
    pub fn new_with_cells(width: i32, depth: i32, initial_cells: &[(i32, i32)]) -> Universe {
        let size = (width * depth) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        cells.set_range(.., false);

        for (x, y) in initial_cells {
            cells.set((y * width + x) as usize, true);
        }

        Universe {
            width,
            depth,
            cells,
        }
    }

    pub fn is_live(&self, x: i32, y: i32) -> bool {
        return self.cells[self.get_index(x, y)];
    }
}
