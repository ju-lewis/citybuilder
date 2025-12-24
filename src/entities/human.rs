use crate::world::map::Map;

const MAX_WALK_ATTEMPTS: i32 = 10;


#[derive(Debug)]
pub enum Action {
    None,
    Move((usize, usize))
}



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

    pub fn get_coord(&self) -> (usize, usize) {
        return self.coord;
    }

    // This is simply where humans choose an action based on the conditions
    pub fn decide(&self, map: &Map) -> (u32, Action) {

        // TODO: Add condition for idling/wandering
        // Idle/wandering behaviour
        let mut attempts = 0;
        while attempts < MAX_WALK_ATTEMPTS {
            let new_coord = (
                self.coord.0.saturating_add_signed(rand::random_range(-1..=1) as isize), 
                self.coord.1.saturating_add_signed(rand::random_range(-1..=1) as isize)
            );

            if map.is_in_bounds(new_coord) && map.is_walkable(new_coord) {
                return (self.id, Action::Move(new_coord));
            }

            attempts += 1;
        }

        return (self.id, Action::None);
    }


    // This is the primary 'update' step for humans (regarding internal state changes)
    pub fn act(&mut self, action: &Action) {

        match action {
            Action::None => (),
            Action::Move(new_coord) => self.coord = *new_coord,
        }
    }
}


