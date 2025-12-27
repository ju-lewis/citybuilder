use std::{collections::{BinaryHeap, HashSet}, rc::Rc};

use crate::world::map::{Coord, Map};

struct HeapNode {
    val: f32,
    cell: Coord,
    prev: Option<Rc<HeapNode>>
}

impl HeapNode {

    pub fn backtrack(&self) -> Vec<(usize, usize)> {

        let mut maybe_prev = &self.prev;
        let mut coords: Vec<Coord> = vec![self.cell];

        while let Some(prev) = maybe_prev {
            coords.push(prev.cell);

            maybe_prev = &prev.prev;
        }
        
        return coords;
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        return self.val.eq(&other.val);
    }
}
impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.val.partial_cmp(&other.val);
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.val.total_cmp(&other.val);
    }
}


impl Map {
    
    pub fn get_path(&self, start: Coord, goal: Coord) -> Option<Vec<Coord>> {

        // Early return for impossible searches
        if !self.is_walkable(goal) {
            return None;
        }

        // Create 'visited' dictionary
        let mut visited: HashSet<Coord> = HashSet::new();
        visited.insert(start);


        // Basic greedy search algorithm (with backtracking)
        // NOTE: This is a MAX HEAP (not a min heap), so our heuristic is the reciprocal 
        // of a normal heuristic.
        let mut priority_queue: BinaryHeap<Rc<HeapNode>> = BinaryHeap::new();

        let initial_node = HeapNode {
            val: Self::compute_heuristic(start, goal),
            cell: start,
            prev: None
        };

        priority_queue.push(Rc::new(initial_node));


        while !priority_queue.is_empty() {
            let maybe_best_node = priority_queue.pop();

            if let Some(best_node) = maybe_best_node {

                // Goal test
                if best_node.cell == goal {
                    return Some(best_node.backtrack());
                }

                // Add all new reachable cells
                self.get_adjacent_walkable_cells(best_node.cell)
                    .iter()
                    .map(|c| HeapNode {
                        val: Self::compute_heuristic(*c, goal), 
                        cell: *c,
                        prev: Some(Rc::clone(&best_node))
                    })
                    .for_each(|n| {

                        let cell = n.cell;
                        
                        if !visited.contains(&cell) {
                            priority_queue.push(Rc::new(n));
                        }

                        // Mark the corresponding cell as visited
                        visited.insert(cell);
                    });
            }
        }

        None
    }

    pub fn get_adjacent_walkable_cells(&self, c: Coord) -> Vec<Coord> {

        let mut coords: Vec<Coord> = Vec::new();

        for x in -1..=1 {
            for y in -1..=1 {
                
                let new_row = c.0.saturating_add_signed(y);
                let new_col = c.1.saturating_add_signed(x);

                if self.is_walkable((new_row, new_col)) {
                    coords.push((new_row, new_col));
                }

            }
        }

        return coords;
    }

    fn compute_heuristic(curr: Coord, target: Coord) -> f32 {

        // We're using the negative to make the heuristic appropriate for a max-heap
        return -((target.0 as f32 - curr.0 as f32).powi(2) + (target.1 as f32 - curr.1 as f32).powi(2)).sqrt();
    }

}


