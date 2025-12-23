use super::map::{CellType, Map};

use rand;



pub fn generate_map(size: (usize, usize)) -> Map {
    
    let mut map = Map::empty(size);

    // Generate ocean
    for row in 0..map.size.0 {
        for col in 0..map.size.1 {

            if is_in_ocean((row, col)) {
                map.cells[row][col].cell_type = CellType::Water;
            }
        }
    }
    

    return map;
}


fn is_in_ocean(coord: (usize, usize)) -> bool {
    
    coord.1 < 20
}


