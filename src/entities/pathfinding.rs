use std::collections::BinaryHeap;

use crate::world::map::{Coord, Map};

struct HeapNode(f32, Coord);

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        return self.0.eq(&other.0);
    }
}
impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.0.partial_cmp(&other.0);
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.0.total_cmp(&other.0);
    }
}


impl Map {
    
    pub fn get_path(&self, start: Coord, goal: Coord) -> Option<Vec<Coord>> {

        // Basic greedy search algorithm (with backtracking)
        // NOTE: This is a MAX HEAP (not a min heap), so our heuristic will be inverted
        let mut priority_queue: BinaryHeap<HeapNode> = BinaryHeap::new();

        // Add initial walkable cells
        self.get_adjacent_walkable_cells(start)
            .iter()
            .map(|c| HeapNode(Self::compute_heuristic(*c, goal), *c))
            .for_each(|n| priority_queue.push(n));


        while !priority_queue.is_empty() {
            
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

        return (((target.0 - curr.0) as f32).powi(2) + ((target.1 - curr.1) as f32).powi(2)).sqrt();
    }

}


