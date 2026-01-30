use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// A node in the HashLife quadtree
#[derive(Clone, Debug)]
pub struct Node {
    /// Level of this node (0 = single cell, 1 = 2x2, 2 = 4x4, etc.)
    level: u8,
    /// Population count (number of live cells)
    population: u64,
    /// Node content
    content: NodeContent,
}

#[derive(Clone, Debug)]
enum NodeContent {
    /// Leaf node containing a single cell state
    Leaf(bool),
    /// Inner node with 4 quadrants (NW, NE, SW, SE)
    Inner {
        nw: Rc<Node>,
        ne: Rc<Node>,
        sw: Rc<Node>,
        se: Rc<Node>,
    },
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.level != other.level {
            return false;
        }
        match (&self.content, &other.content) {
            (NodeContent::Leaf(a), NodeContent::Leaf(b)) => a == b,
            (
                NodeContent::Inner { nw: nw1, ne: ne1, sw: sw1, se: se1, .. },
                NodeContent::Inner { nw: nw2, ne: ne2, sw: sw2, se: se2, .. },
            ) => {
                Rc::ptr_eq(nw1, nw2) && Rc::ptr_eq(ne1, ne2) 
                    && Rc::ptr_eq(sw1, sw2) && Rc::ptr_eq(se1, se2)
            }
            _ => false,
        }
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.level.hash(state);
        match &self.content {
            NodeContent::Leaf(alive) => {
                0u8.hash(state);
                alive.hash(state);
            }
            NodeContent::Inner { nw, ne, sw, se, .. } => {
                1u8.hash(state);
                (Rc::as_ptr(nw) as usize).hash(state);
                (Rc::as_ptr(ne) as usize).hash(state);
                (Rc::as_ptr(sw) as usize).hash(state);
                (Rc::as_ptr(se) as usize).hash(state);
            }
        }
    }
}

impl Node {
    fn leaf(alive: bool) -> Self {
        Node {
            level: 0,
            population: if alive { 1 } else { 0 },
            content: NodeContent::Leaf(alive),
        }
    }

    fn inner(nw: Rc<Node>, ne: Rc<Node>, sw: Rc<Node>, se: Rc<Node>) -> Self {
        assert_eq!(nw.level, ne.level);
        assert_eq!(nw.level, sw.level);
        assert_eq!(nw.level, se.level);
        
        let population = nw.population + ne.population + sw.population + se.population;
        
        Node {
            level: nw.level + 1,
            population,
            content: NodeContent::Inner { nw, ne, sw, se },
        }
    }

    fn is_alive(&self) -> bool {
        matches!(self.content, NodeContent::Leaf(true))
    }
}

/// Cache for canonical nodes
pub struct NodeCache {
    leaves: [Rc<Node>; 2],
    inner_cache: HashMap<(usize, usize, usize, usize), Rc<Node>>,
}

impl NodeCache {
    fn new() -> Self {
        NodeCache {
            leaves: [
                Rc::new(Node::leaf(false)),
                Rc::new(Node::leaf(true)),
            ],
            inner_cache: HashMap::new(),
        }
    }

    fn get_leaf(&self, alive: bool) -> Rc<Node> {
        self.leaves[alive as usize].clone()
    }

    fn get_inner(&mut self, nw: Rc<Node>, ne: Rc<Node>, sw: Rc<Node>, se: Rc<Node>) -> Rc<Node> {
        let key = (
            Rc::as_ptr(&nw) as usize,
            Rc::as_ptr(&ne) as usize,
            Rc::as_ptr(&sw) as usize,
            Rc::as_ptr(&se) as usize,
        );

        if let Some(node) = self.inner_cache.get(&key) {
            return node.clone();
        }

        let node = Rc::new(Node::inner(nw, ne, sw, se));
        self.inner_cache.insert(key, node.clone());
        node
    }

    fn get_empty(&mut self, level: u8) -> Rc<Node> {
        if level == 0 {
            return self.get_leaf(false);
        }
        let sub = self.get_empty(level - 1);
        self.get_inner(sub.clone(), sub.clone(), sub.clone(), sub.clone())
    }
}

/// Main HashLife universe
pub struct Universe {
    root: Rc<Node>,
    cache: NodeCache,
    generation: u64,
    result_cache: HashMap<usize, Rc<Node>>,
}

impl Universe {
    /// Create a new empty universe
    pub fn new(size_level: usize) -> Self {
        let mut cache = NodeCache::new();
        let level = size_level.max(3) as u8;
        let root = cache.get_empty(level);
        
        Universe {
            root,
            cache,
            generation: 0,
            result_cache: HashMap::new(),
        }
    }

    /// Set a cell at the given coordinates
    pub fn set_cell(&mut self, x: i64, y: i64, alive: bool) {
        let size = 1i64 << self.root.level;
        let half_size = size / 2;
        
        if x < -half_size || x >= half_size || y < -half_size || y >= half_size {
            self.expand();
            return self.set_cell(x, y, alive);
        }
        
        let root = self.root.clone();
        self.root = self.set_cell_recursive(&root, x, y, alive, -half_size, -half_size);
    }

    fn set_cell_recursive(&mut self, node: &Rc<Node>, x: i64, y: i64, alive: bool,
                          node_x: i64, node_y: i64) -> Rc<Node> {
        if node.level == 0 {
            return self.cache.get_leaf(alive);
        }

        let NodeContent::Inner { nw, ne, sw, se, .. } = &node.content else {
            unreachable!();
        };

        let half_size = 1i64 << (node.level - 1);
        let mid_x = node_x + half_size;
        let mid_y = node_y + half_size;

        if x < mid_x && y < mid_y {
            let new_nw = self.set_cell_recursive(nw, x, y, alive, node_x, node_y);
            self.cache.get_inner(new_nw, ne.clone(), sw.clone(), se.clone())
        } else if x >= mid_x && y < mid_y {
            let new_ne = self.set_cell_recursive(ne, x, y, alive, mid_x, node_y);
            self.cache.get_inner(nw.clone(), new_ne, sw.clone(), se.clone())
        } else if x < mid_x && y >= mid_y {
            let new_sw = self.set_cell_recursive(sw, x, y, alive, node_x, mid_y);
            self.cache.get_inner(nw.clone(), ne.clone(), new_sw, se.clone())
        } else {
            let new_se = self.set_cell_recursive(se, x, y, alive, mid_x, mid_y);
            self.cache.get_inner(nw.clone(), ne.clone(), sw.clone(), new_se)
        }
    }

    /// Get cell value at coordinates
    pub fn get_cell(&self, x: i64, y: i64) -> bool {
        let size = 1i64 << self.root.level;
        let half_size = size / 2;
        
        if x < -half_size || x >= half_size || y < -half_size || y >= half_size {
            return false;
        }
        
        self.get_cell_recursive(&self.root, x, y, -half_size, -half_size)
    }

    fn get_cell_recursive(&self, node: &Rc<Node>, x: i64, y: i64, 
                          node_x: i64, node_y: i64) -> bool {
        if node.level == 0 {
            return node.is_alive();
        }

        let NodeContent::Inner { nw, ne, sw, se, .. } = &node.content else {
            unreachable!();
        };

        let half_size = 1i64 << (node.level - 1);
        let mid_x = node_x + half_size;
        let mid_y = node_y + half_size;

        if x < mid_x && y < mid_y {
            self.get_cell_recursive(nw, x, y, node_x, node_y)
        } else if x >= mid_x && y < mid_y {
            self.get_cell_recursive(ne, x, y, mid_x, node_y)
        } else if x < mid_x && y >= mid_y {
            self.get_cell_recursive(sw, x, y, node_x, mid_y)
        } else {
            self.get_cell_recursive(se, x, y, mid_x, mid_y)
        }
    }

    fn expand(&mut self) {
        let empty = self.cache.get_empty(self.root.level - 1);
        let NodeContent::Inner { nw, ne, sw, se, .. } = &self.root.content else {
            unreachable!();
        };

        let new_nw = self.cache.get_inner(
            empty.clone(), empty.clone(),
            empty.clone(), nw.clone()
        );
        let new_ne = self.cache.get_inner(
            empty.clone(), empty.clone(),
            ne.clone(), empty.clone()
        );
        let new_sw = self.cache.get_inner(
            empty.clone(), sw.clone(),
            empty.clone(), empty.clone()
        );
        let new_se = self.cache.get_inner(
            se.clone(), empty.clone(),
            empty.clone(), empty.clone()
        );

        self.root = self.cache.get_inner(new_nw, new_ne, new_sw, new_se);
    }

    /// Step forward in time using HashLife algorithm
    /// Note: This advances by 2^(level-2) generations per call
    /// For a typical level-3 universe, this is 2 generations per step
    pub fn step(&mut self) {
        while self.root.level < 3 {
            if self.root.population == 0 {
                self.generation += 1;
                return;
            }
            self.expand();
        }

        let root = self.root.clone();
        let steps = 1u64 << (self.root.level - 2);
        self.root = self.next_generation(&root);
        self.generation += steps;
    }

    fn next_generation(&mut self, node: &Rc<Node>) -> Rc<Node> {
        let node_ptr = Rc::as_ptr(node) as usize;
        
        // Check result cache
        if let Some(result) = self.result_cache.get(&node_ptr) {
            return result.clone();
        }

        if node.level == 2 {
            let result = self.compute_level2(node);
            self.result_cache.insert(node_ptr, result.clone());
            return result;
        }

        let NodeContent::Inner { nw, ne, sw, se, .. } = &node.content else {
            unreachable!();
        };

        // Pre-compute center nodes
        let center_nw_ne = self.center_subnode_horizontal(nw, ne);
        let center_nw_sw = self.center_subnode_vertical(nw, sw);
        let center_ne_se = self.center_subnode_vertical(ne, se);
        let center_sw_se = self.center_subnode_horizontal(sw, se);
        let center = self.center_node(node);

        // Compute the 9 overlapping subnodes
        let n00 = self.next_generation(nw);
        let n01 = self.next_generation(&center_nw_ne);
        let n02 = self.next_generation(ne);
        let n10 = self.next_generation(&center_nw_sw);
        let n11 = self.next_generation(&center);
        let n12 = self.next_generation(&center_ne_se);
        let n20 = self.next_generation(sw);
        let n21 = self.next_generation(&center_sw_se);
        let n22 = self.next_generation(se);

        // Combine results into 4 intermediate quadrants
        let intermediate_nw = self.cache.get_inner(n00, n01.clone(), n10.clone(), n11.clone());
        let intermediate_ne = self.cache.get_inner(n01, n02, n11.clone(), n12.clone());
        let intermediate_sw = self.cache.get_inner(n10, n11.clone(), n20, n21.clone());
        let intermediate_se = self.cache.get_inner(n11, n12, n21, n22);

        // Advance the 4 intermediate quadrants one more time
        let result_nw = self.next_generation(&intermediate_nw);
        let result_ne = self.next_generation(&intermediate_ne);
        let result_sw = self.next_generation(&intermediate_sw);
        let result_se = self.next_generation(&intermediate_se);

        let result = self.cache.get_inner(result_nw, result_ne, result_sw, result_se);

        // Cache the result
        self.result_cache.insert(node_ptr, result.clone());
        result
    }

    fn center_node(&mut self, node: &Rc<Node>) -> Rc<Node> {
        let NodeContent::Inner { nw, ne, sw, se, .. } = &node.content else {
            unreachable!();
        };

        let NodeContent::Inner { se: nw_se, .. } = &nw.content else { unreachable!(); };
        let NodeContent::Inner { sw: ne_sw, .. } = &ne.content else { unreachable!(); };
        let NodeContent::Inner { ne: sw_ne, .. } = &sw.content else { unreachable!(); };
        let NodeContent::Inner { nw: se_nw, .. } = &se.content else { unreachable!(); };

        self.cache.get_inner(
            nw_se.clone(),
            ne_sw.clone(),
            sw_ne.clone(),
            se_nw.clone(),
        )
    }

    fn center_subnode_horizontal(&mut self, left: &Rc<Node>, right: &Rc<Node>) -> Rc<Node> {
        let NodeContent::Inner { ne: left_ne, se: left_se, .. } = &left.content else { unreachable!(); };
        let NodeContent::Inner { nw: right_nw, sw: right_sw, .. } = &right.content else { unreachable!(); };

        self.cache.get_inner(
            left_ne.clone(),
            right_nw.clone(),
            left_se.clone(),
            right_sw.clone(),
        )
    }

    fn center_subnode_vertical(&mut self, top: &Rc<Node>, bottom: &Rc<Node>) -> Rc<Node> {
        let NodeContent::Inner { sw: top_sw, se: top_se, .. } = &top.content else { unreachable!(); };
        let NodeContent::Inner { nw: bottom_nw, ne: bottom_ne, .. } = &bottom.content else { unreachable!(); };

        self.cache.get_inner(
            top_sw.clone(),
            top_se.clone(),
            bottom_nw.clone(),
            bottom_ne.clone(),
        )
    }

    fn compute_level2(&mut self, node: &Rc<Node>) -> Rc<Node> {
        let NodeContent::Inner { nw, ne, sw, se, .. } = &node.content else {
            unreachable!();
        };

        // Extract 16 cells from 4x4 area
        let mut cells = [[false; 4]; 4];
        self.extract_2x2(nw, &mut cells, 0, 0);
        self.extract_2x2(ne, &mut cells, 2, 0);
        self.extract_2x2(sw, &mut cells, 0, 2);
        self.extract_2x2(se, &mut cells, 2, 2);

        // Apply Conway's rules to center 2x2 area
        let mut result = [[false; 2]; 2];
        for y in 0..2 {
            for x in 0..2 {
                let cx = x + 1;
                let cy = y + 1;
                let neighbors = self.count_neighbors_array(&cells, cx, cy);
                result[y][x] = match (cells[cy][cx], neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        // Build result node (level 1 = 2x2)
        let r_nw = self.cache.get_leaf(result[0][0]);
        let r_ne = self.cache.get_leaf(result[0][1]);
        let r_sw = self.cache.get_leaf(result[1][0]);
        let r_se = self.cache.get_leaf(result[1][1]);

        self.cache.get_inner(r_nw, r_ne, r_sw, r_se)
    }

    fn extract_2x2(&self, node: &Rc<Node>, cells: &mut [[bool; 4]; 4], 
                   offset_x: usize, offset_y: usize) {
        if node.level == 0 {
            cells[offset_y][offset_x] = node.is_alive();
        } else {
            let NodeContent::Inner { nw, ne, sw, se, .. } = &node.content else {
                unreachable!();
            };
            self.extract_2x2(nw, cells, offset_x, offset_y);
            self.extract_2x2(ne, cells, offset_x + 1, offset_y);
            self.extract_2x2(sw, cells, offset_x, offset_y + 1);
            self.extract_2x2(se, cells, offset_x + 1, offset_y + 1);
        }
    }

    fn count_neighbors_array(&self, cells: &[[bool; 4]; 4], x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx < 4 && ny < 4 && cells[ny][nx] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn population(&self) -> u64 {
        self.root.population
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
        
        // With HashLife at level 3, step advances by 2 generations
        // So horizontal blinker -> vertical (1 step) -> horizontal (2 steps)
        universe.step();
        
        // After 2 generations, should be back to horizontal
        assert!(universe.get_cell(0, 0));
        assert!(universe.get_cell(1, 0));
        assert!(universe.get_cell(2, 0));
        assert!(!universe.get_cell(1, -1));
        assert!(!universe.get_cell(1, 1));
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
        
        // Step - should remain the same (still life)
        universe.step();
        
        assert!(universe.get_cell(0, 0));
        assert!(universe.get_cell(1, 0));
        assert!(universe.get_cell(0, 1));
        assert!(universe.get_cell(1, 1));
        assert_eq!(universe.population(), 4);
    }
}
