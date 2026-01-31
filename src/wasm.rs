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

    /// Get renderable regions using the quadtree structure for efficient rendering.
    /// 
    /// This method is much more efficient than getCells for zoomed-out views because
    /// it uses the hierarchical structure of the hashlife quadtree to:
    /// - Skip empty regions entirely
    /// - Aggregate small regions into density values
    /// 
    /// Returns a flat array of [x, y, size, density] tuples packed as f32:
    /// - x, y: world coordinates (as f32)
    /// - size: the side length of the region (as f32)
    /// - density: population/area ratio (0.0 to 1.0)
    /// 
    /// min_render_size: minimum world-unit size for regions. Smaller regions are
    /// aggregated and rendered based on density.
    #[wasm_bindgen(js_name = getRenderRegions)]
    pub fn get_render_regions(
        &self,
        view_x_min: i32,
        view_y_min: i32,
        view_x_max: i32,
        view_y_max: i32,
        min_render_size: u32,
    ) -> Vec<f32> {
        let regions = self.universe.collect_render_regions(
            view_x_min as i64,
            view_y_min as i64,
            view_x_max as i64,
            view_y_max as i64,
            min_render_size,
        );

        let mut result = Vec::with_capacity(regions.len() * 4);
        for (x, y, size, density) in regions {
            result.push(x as f32);
            result.push(y as f32);
            result.push(size as f32);
            result.push(density);
        }
        result
    }
}
