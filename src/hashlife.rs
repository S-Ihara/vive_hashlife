use std::collections::HashMap;

/// Main Game of Life universe
pub struct Universe {
    cells: HashMap<(i64, i64), bool>,
    generation: u64,
}

impl Universe {
    /// Create a new empty universe
    pub fn new(_size_level: usize) -> Self {
        Universe {
            cells: HashMap::new(),
            generation: 0,
        }
    }

    /// Set a cell at the given coordinates
    pub fn set_cell(&mut self, x: i64, y: i64, alive: bool) {
        if alive {
            self.cells.insert((x, y), true);
        } else {
            self.cells.remove(&(x, y));
        }
    }

    /// Get cell value at coordinates
    pub fn get_cell(&self, x: i64, y: i64) -> bool {
        self.cells.get(&(x, y)).copied().unwrap_or(false)
    }

    /// Count neighbors for a cell
    fn count_neighbors(&self, x: i64, y: i64) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get_cell(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Step forward in time
    pub fn step(&mut self) {
        let mut new_cells = HashMap::new();
        
        // Get bounds of current cells
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        
        for &(x, y) in self.cells.keys() {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        
        // If no cells, nothing to do
        if min_x == i64::MAX {
            self.generation += 1;
            return;
        }
        
        // Check all cells in the bounding box plus a margin
        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let alive = self.get_cell(x, y);
                let neighbors = self.count_neighbors(x, y);
                
                let new_state = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
                
                if new_state {
                    new_cells.insert((x, y), true);
                }
            }
        }
        
        self.cells = new_cells;
        self.generation += 1;
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn population(&self) -> u64 {
        self.cells.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_universe() {
        let universe = Universe::new(4);
        assert_eq!(universe.population(), 0);
        assert_eq!(universe.generation(), 0);
    }

    #[test]
    fn test_set_get_cell() {
        let mut universe = Universe::new(4);
        universe.set_cell(0, 0, true);
        universe.set_cell(1, 0, true);
        universe.set_cell(0, 1, true);
        
        assert!(universe.get_cell(0, 0));
        assert!(universe.get_cell(1, 0));
        assert!(universe.get_cell(0, 1));
        assert!(!universe.get_cell(2, 2));
    }

    #[test]
    fn test_blinker() {
        let mut universe = Universe::new(4);
        
        // Create horizontal blinker
        universe.set_cell(0, 0, true);
        universe.set_cell(1, 0, true);
        universe.set_cell(2, 0, true);
        
        assert_eq!(universe.population(), 3);
        
        // Step once - should become vertical
        universe.step();
        
        assert!(!universe.get_cell(0, 0));
        assert!(universe.get_cell(1, 0));
        assert!(!universe.get_cell(2, 0));
        assert!(universe.get_cell(1, -1));
        assert!(universe.get_cell(1, 1));
        assert_eq!(universe.population(), 3);
    }

    #[test]
    fn test_block() {
        let mut universe = Universe::new(4);
        
        // Create block (still life)
        universe.set_cell(0, 0, true);
        universe.set_cell(1, 0, true);
        universe.set_cell(0, 1, true);
        universe.set_cell(1, 1, true);
        
        assert_eq!(universe.population(), 4);
        
        // Step - should remain the same
        universe.step();
        
        assert!(universe.get_cell(0, 0));
        assert!(universe.get_cell(1, 0));
        assert!(universe.get_cell(0, 1));
        assert!(universe.get_cell(1, 1));
        assert_eq!(universe.population(), 4);
    }
}
