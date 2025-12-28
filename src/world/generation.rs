use super::map::{CellType, Map};

use rand::{self, seq::IndexedRandom};


pub const LAKE_RADIUS: f32 = 5.0;



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

    // Generate lakes
    generate_lakes(&mut map);


    // Generate Trees
    let num_trees = rand::random_range(5..20);
    map.trees.reserve(num_trees);

    for i in 0..num_trees {
        //let mut coord: (usize, usize) = (0,0);

        //// Loop until we get a position that's on the grass
        //loop {
        //    if map.cells[coord.0][coord.1].cell_type == CellType::Grass {
        //        break;
        //    } 

        //    coord.0 = rand::random_range(1..map.size.0);
        //    coord.1 = rand::random_range(1..map.size.1);
        //}

        let coord = match map.get_random_constrained_coord(Some(&[CellType::Grass])) {
            Some(c) => c,
            None => continue
        };


        // Draw surrounding leaves
        for x in -5..=5 {
            for y in -2..=2 {

                let new_row = coord.0.saturating_add_signed(y);
                let new_col = coord.1.saturating_add_signed(x);
                
                if new_row >= map.size.0 || new_col >= map.size.1 {continue;}


                if (x as f32).hypot(y as f32) < 3.5 {

                    let cell = &mut map.cells[new_row][new_col];

                    // Record covered cell contents
                    cell.covered.push((cell.cell_type.clone(), cell.val));

                    cell.cell_type = CellType::Leaves{id: i as u32};
                    cell.val = *['░', '▒'].choose(&mut rng).expect("RNG failed.");
                }
            }
        }

        // Draw trunk (overwriting leaves)
        let trunk_cell = &mut map.cells[coord.0][coord.1];

        trunk_cell.covered.push((trunk_cell.cell_type.clone(), trunk_cell.val));

        trunk_cell.cell_type = CellType::TreeTrunk{id: i as u32};
        trunk_cell.val = '0';

        // Record tree in data structure
        map.trees.push(i as u32);
    }


    return map;
}

fn generate_lakes(map: &mut Map) {
    
    let num_lakes = rand::random_range(2..=5);
    let mut rng = rand::rng();

    for _ in 0..num_lakes {
        
        let coord = match map.get_random_constrained_coord(Some(&[CellType::Grass])) {
            Some(c) => c,
            None => continue
        };

        map.lakes.push(coord);


        for x in -7..=7 {
            for y in -3..=3 {

                let new_row = coord.0.saturating_add_signed(y);
                let new_col = coord.1.saturating_add_signed(x);
                
                if new_row >= map.size.0 || new_col >= map.size.1 {continue;}


                if (x as f32).hypot(y as f32) < LAKE_RADIUS {

                    let cell = &mut map.cells[new_row][new_col];

                    cell.cell_type = CellType::Water;
                    cell.val = *['~', '='].choose(&mut rng).expect("RNG failed.");
                }
            }
        }

    }
    

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



