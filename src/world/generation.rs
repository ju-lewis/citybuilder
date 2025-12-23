use super::map::Map;

use rand;



pub fn generate_map(size: (usize, usize)) -> Map {
    

    let mut cells = vec![vec![size.1]; size.0];

    // Generate ocean
    for row in 0..size.0 {
        for col in 0..size.1 {
            // TODO
        }
    }
    

    Map {
        cells: vec![],
        size: (0,0)
    }
}


fn is_in_ocean(xy: (usize, usize)) -> bool {
    
    xy.0 < 50
}


