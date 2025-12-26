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
    
    pub fn get_path(&self, c1: Coord, c2: Coord) -> Option<Vec<Coord>> {

        // Basic greedy search algorithm (with backtracking)
        // NOTE: This is a MAX HEAP (not a min heap), so our heuristic will be inverted
        let mut priority_queue: BinaryHeap<HeapNode> = BinaryHeap::new();


        None
    }


    pub fn get_adjacent_walkable_cells(&self, c: Coord) -> Vec<Coord> {


        
        Vec::new()
    }

}


