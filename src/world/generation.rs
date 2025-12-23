use super::map::{CellType, Map};

use rand::{self, random_range, seq::IndexedRandom};



pub fn generate_map(size: (usize, usize)) -> Map {
    
    let mut map = Map::empty(size);

    let mut rng = rand::rng();

    // Generate ocean
    for row in 0..map.size.0 {
        for col in 0..map.size.1 {

            if is_in_ocean((row, col)) {
                map.cells[row][col].cell_type = CellType::Water;
                map.cells[row][col].val = *['~', '-', '='].choose(&mut rng).expect("RNG failed.");
            }
        }
    }
    

    return map;
}


fn is_in_ocean(coord: (usize, usize)) -> bool {
    
    let shoreline = ((coord.0 as f32 * 0.17).sin() + 2.0) * 10.0;

    return (coord.1 as f32) < shoreline;
}


fn is_on_beach() -> bool {
    todo!();
}
