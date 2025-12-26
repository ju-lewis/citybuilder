use crate::world::map::Map;

/* ------------------------------- CONSTANTS ------------------------------- */

const MAX_WALK_ATTEMPTS: i32 = 10;
const STATUS_MAX: f32 = 1.0;


const THIRST_GROWTH_RATE: f32 = 0.01;
const CRITICAL_THIRST: f32 = 0.7;



#[derive(Debug)]
pub enum Action {
    None,
    Move((usize, usize))
}



#[derive(Clone, PartialEq)]
pub struct Human {
    pub id: u32,
    coord: (usize, usize), // Intentionally not public!
    target_coord: Option<(usize, usize)>,

    thirst: f32,
    
}


impl Human {

    pub fn new(id: u32, coord: (usize, usize)) -> Self {
        Human {
            id,
            coord,
            target_coord: None,
            thirst: 0.0,
        }
    }

    pub fn get_coord(&self) -> (usize, usize) {
        return self.coord;
    }

    // This is simply where humans choose an action based on the conditions
    pub fn decide(&self, map: &Map) -> (u32, Action) {

        // Address critical conditions first
        if self.thirst >= CRITICAL_THIRST {

            // TODO: Navigate to fresh water
            return (self.id, Action::None);
        }


        // Otherwise continue navigating to target coord
        if let Some(c) = self.target_coord {
            // TODO: PATHFINDING TIME!!!!!
        }


        // If nothing else, idle/wander
        


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

        // Only increase up to the maximum
        if self.thirst < STATUS_MAX {
            self.thirst += THIRST_GROWTH_RATE;
        }


        match action {
            Action::None => (),
            Action::Move(new_coord) => self.coord = *new_coord,
        }
    }
}


