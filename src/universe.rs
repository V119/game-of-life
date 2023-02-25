extern crate js_sys;
use std::fmt::Display;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Universe {
    height: u32,
    width: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (x * self.width + y) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for delt_x in [self.width - 1, 0, 1].iter().copied() {
            for delt_y in [self.height - 1, 0, 1].iter().copied() {
                if delt_x == 0 && delt_y == 0 {
                    continue;
                }

                let pos_x = (delt_x + x) % self.width;
                let pos_y = (delt_y + y) % self.height;
                let idx = self.get_index(pos_x, pos_y);
                count = count + self.cells[idx] as u8;
            }
        }

        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        // let height = 64;
        // let width = 64;
        let cells = (0..height * width)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for x_pos in 0..self.width {
            for y_pos in 0..self.height {
                let idx = self.get_index(x_pos, y_pos);
                let cell = self.cells[idx];
                let live_count = self.live_neighbor_count(x_pos, y_pos);

                let next_cell = match (cell, live_count) {
                    (Cell::Alive, count) if count < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, count) if count > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next_cells[idx] = next_cell;
            }
        }

        self.cells = next_cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Alive { '◼' } else { '◻' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
