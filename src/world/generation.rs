use super::map::{CellType, Map};

use rand::{self, seq::IndexedRandom};



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
                map.cells[row][col].val = *['\'', '`', ',', '♠', '.', '♣', '\"', 'ι', '\"', '\"', '\'', ','].choose(&mut rng).expect("RNG failed.");
            }
        }
    }


    // Generate Trees
    let num_trees = rand::random_range(5..20);
    map.trees.reserve(num_trees);

    for i in 0..num_trees {
        let mut coord: (usize, usize) = (0,0);

        // Loop until we get a position that's on the grass
        loop {
            if map.cells[coord.0][coord.1].cell_type == CellType::Grass {
                break;
            } 

            coord.0 = rand::random_range(1..map.size.0);
            coord.1 = rand::random_range(1..map.size.1);
        }



        // Draw surrounding leaves
        for x in -5..=5 {
            for y in -2..=2 {

                if coord.0 as i32 + y < 0 || coord.1 as i32 + x < 0 ||
                   coord.0 as i32 + y >= map.size.0 as i32 || coord.1 as i32 + x >= map.size.1 as i32 {
                    continue;
                }

                if (x as f32).hypot(y as f32) < 3.5 {
                    map.cells[(coord.0 as i32 + y) as usize][(coord.1 as i32 + x) as usize].cell_type = CellType::Leaves{id: i as u32};
                    map.cells[(coord.0 as i32 + y) as usize][(coord.1 as i32 + x) as usize].val = *['░', '▒'].choose(&mut rng).expect("RNG failed.");
                }
            }
        }

        // Draw trunk (overwriting leaves)
        map.cells[coord.0][coord.1].cell_type = CellType::TreeTrunk{id: i as u32};
        map.cells[coord.0][coord.1].val = '0';

        // Record tree in data structure
        map.trees.push(i as u32);
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



