use crate::world::map::{Coord, Map};
use std::collections::VecDeque;

/* ------------------------------- CONSTANTS ------------------------------- */

const MAX_WALK_ATTEMPTS: i32 = 10;
const STATUS_MAX: f32 = 1.0;


const THIRST_GROWTH_RATE: f32 = 0.001;
const CRITICAL_THIRST: f32 = 0.7;



#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    None,
    Move(Vec<Coord>),
    Drink
}



#[derive(Clone, PartialEq)]
pub struct Human {
    pub id: u32,
    coord: (usize, usize), // Intentionally not public!
    pub action_queue: VecDeque<Action>,

    thirst: f32,
    
}


impl Human {

    pub fn new(id: u32, coord: (usize, usize)) -> Self {
        Human {
            id,
            coord,
            action_queue: VecDeque::new(),
            thirst: 0.0,
        }
    }

    pub fn get_coord(&self) -> (usize, usize) {
        return self.coord;
    }

    // This is simply where humans choose an action based on the conditions
    pub fn decide(&self, map: &Map) -> (u32, Action) {

        // Address critical conditions first
        // TODO: Check if 'Drink' action is already in the queue before continuing
        if self.thirst >= CRITICAL_THIRST {

            return (self.id, Action::Move(Vec::new()));
        }


        // If action queue has things to do, then continue executing those.
        if !self.action_queue.is_empty() {
            return (self.id, Action::None)
        }


        // If nothing else, idle/wander
        let mut attempts = 0;
        while attempts < MAX_WALK_ATTEMPTS {
            let new_coord = (
                self.coord.0.saturating_add_signed(rand::random_range(-1..=1) as isize), 
                self.coord.1.saturating_add_signed(rand::random_range(-1..=1) as isize)
            );

            if map.is_in_bounds(new_coord) && map.is_walkable(new_coord) {
                return (self.id, Action::Move(vec![new_coord]));
            }

            attempts += 1;
        }

        return (self.id, Action::None);
    }

    pub fn queue_action(&mut self, action: &Action) {
        self.action_queue.push_back(action.clone());
    }


    fn move_to_water(&mut self) {
        // Locate the nearest fresh water and navigate towards it
    }


    // This is the primary 'update' step for humans (regarding internal state changes)
    pub fn update(&mut self, curr_action: Action) {

        // GENERIC UPDATES

        // Only increase up to the maximum
        if self.thirst < STATUS_MAX {
            self.thirst += THIRST_GROWTH_RATE;
        }


        // ACTIONS


        match curr_action {
            Action::None => (),
            Action::Move(ref coords) => self.coord = *coords.last().unwrap_or(&self.coord),
            Action::Drink => todo!(),
        };

    }
}


