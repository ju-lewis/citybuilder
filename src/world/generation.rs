use super::map::{CellType, Map};

use rand::{self, random_range, seq::IndexedRandom};



pub fn generate_map(size: (usize, usize)) -> Map {
    
    let mut map = Map::empty(size);

    let mut rng = rand::rng();

    // Generate ocean
    for row in 0..map.size.0 {
        for col in 0..map.size.1 {

            let coord = (row, col);

            if is_in_ocean(coord) {
                map.cells[row][col].cell_type = CellType::Water;
                map.cells[row][col].val = *['~', '-', '='].choose(&mut rng).expect("RNG failed.");
            } else if is_on_beach(coord) {
                map.cells[row][col].cell_type = CellType::Sand;
                map.cells[row][col].val = *['#', '@'].choose(&mut rng).expect("RNG failed.");
            }



            // Add some detailing to remaining grass
            if map.cells[row][col].cell_type == CellType::Grass {
                map.cells[row][col].val = *['\'', '`', ',', '♠'].choose(&mut rng).expect("RNG failed.");
            }
        }
    }
    

    return map;
}


fn compute_shoreline(coord: (usize, usize)) -> f32 {
    return ((coord.0 as f32 * 0.17).sin() + 2.0) * 10.0;
}



fn is_in_ocean(coord: (usize, usize)) -> bool {
    
    return (coord.1 as f32) < compute_shoreline(coord);
}


fn is_on_beach(coord: (usize, usize)) -> bool {
    let dist = coord.1 as f32 - compute_shoreline(coord);

    dist > 0.0 && dist < 5.0
}
