use wasm_bindgen::prelude::*;
use crate::hashlife::Universe;

#[wasm_bindgen]
pub struct WasmUniverse {
    universe: Universe,
    size_level: usize,
}

#[wasm_bindgen]
impl WasmUniverse {
    #[wasm_bindgen(constructor)]
    pub fn new(size_level: usize) -> WasmUniverse {
        WasmUniverse {
            universe: Universe::new(size_level),
            size_level,
        }
    }

    #[wasm_bindgen(js_name = setCell)]
    pub fn set_cell(&mut self, x: i32, y: i32, alive: bool) {
        self.universe.set_cell(x as i64, y as i64, alive);
    }

    #[wasm_bindgen(js_name = getCell)]
    pub fn get_cell(&self, x: i32, y: i32) -> bool {
        self.universe.get_cell(x as i64, y as i64)
    }

    pub fn step(&mut self) {
        self.universe.step();
    }

    pub fn generation(&self) -> u64 {
        self.universe.generation()
    }

    pub fn population(&self) -> u64 {
        self.universe.population()
    }

    pub fn clear(&mut self) {
        self.universe = Universe::new(self.size_level);
    }

    #[wasm_bindgen(js_name = setCells)]
    pub fn set_cells(&mut self, cells: &[i32]) {
        for i in (0..cells.len()).step_by(2) {
            if i + 1 < cells.len() {
                self.set_cell(cells[i], cells[i + 1], true);
            }
        }
    }

    #[wasm_bindgen(js_name = getCells)]
    pub fn get_cells(&self, x_min: i32, y_min: i32, x_max: i32, y_max: i32) -> Vec<i32> {
        let mut cells = Vec::new();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if self.get_cell(x, y) {
                    cells.push(x);
                    cells.push(y);
                }
            }
        }
        cells
    }
}
