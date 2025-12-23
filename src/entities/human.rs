use crate::world::map::Map;




#[derive(Clone, PartialEq)]
pub struct Human {
    pub id: u32,
    coord: (usize, usize), // Intentionally not public!
}


impl Human {

    pub fn new(id: u32, coord: (usize, usize)) -> Self {
        Human {
            id,
            coord
        }
    }

    pub fn update(&mut self, map: &Map) {
        

        // TODO: Add condition for idling/wandering
        // Idle/wandering behaviour
        loop {
            let new_coord = (
                self.coord.0.saturating_add_signed(rand::random_range(-1..1) as isize), 
                self.coord.1.saturating_add_signed(rand::random_range(-1..1) as isize)
            );

            if map.is_walkable(new_coord) {
            }
        }

    }
}


