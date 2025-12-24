
use std::borrow::Borrow;

use termion::color::{self, Fg};

use crate::entities::human::{Action, Human};

use super::generation::generate_map;

#[derive(Clone, PartialEq)]
pub enum CellType {
    Water,
    Sand,
    Grass,
    TreeTrunk {id: u32},
    Leaves {id: u32},

    Human {id: u32}
}


/*------------------------------ Map Cell Code -------------------------------*/

#[derive(Clone)]
pub struct MapCell {
    pub cell_type: CellType,
    pub val: char,

    pub covered_type: Option<CellType>,
    pub covered_val: Option<char>,
}

impl MapCell {
    pub fn render(&self) -> String {
        match self.cell_type {
            CellType::Water => format!("{}{}",  Fg(color::Cyan), self.val),
            CellType::Sand  => format!("{}{}",  Fg(color::Yellow), self.val),
            CellType::Grass => format!("{}{}", Fg(color::Green), self.val),
            CellType::TreeTrunk{id: _} => format!("{}{}", Fg(color::Rgb(139, 69, 19)), self.val),
            CellType::Leaves{id: _} => format!("{}{}", Fg(color::LightGreen), self.val),
            CellType::Human{id: _} => format!("{}{}", Fg(color::White), '☺')
        }
    }

    pub fn is_moveable(&self) -> bool {
        match self.cell_type {
            CellType::Water => false,
            CellType::Sand => false,
            CellType::Grass => false,
            CellType::TreeTrunk { id: _ } => false,
            CellType::Leaves { id: _ } => false,
            CellType::Human { id: _ } => true,
        }
    }
}



/*--------------------------------- Map Code ---------------------------------*/

/// Data structure containing information about each cell.
/// Has a 1-1 correspondance with the framebuffer
pub struct Map {
    pub cells: Vec<Vec<MapCell>>,
    pub size: (usize, usize),

    pub trees: Vec<u32>,
    pub humans: Vec<Human>
}


impl Map {
    pub fn empty(size: (usize, usize)) -> Self {

        // Empty cell
        let c = MapCell {
            cell_type: CellType::Grass,
            val: '\"',

            covered_type: None,
            covered_val: None
        };

        Map {
            cells: vec![vec![c; size.1.into()]; size.0.into()],
            size,
            trees: Vec::new(),
            humans: Vec::new()
        }
    }

    pub fn new(size: (usize, usize)) -> Self {
        let mut map = generate_map(size);

        map.populate_humans();

        return map;
    }

    fn populate_humans(&mut self) {
        
        let num_humans = 10;
        self.humans.reserve(num_humans);

        for i in 0..num_humans {

            // Place human on the map (on a grass tile)
            let mut coord: (usize, usize) = (0, 0);
            let mut chosen_cell = &mut self.cells[coord.0][coord.1];

            loop {

                if chosen_cell.cell_type == CellType::Grass {
                    break;
                }

                coord.0 = rand::random_range(0..self.size.0);
                coord.1 = rand::random_range(0..self.size.1);

                chosen_cell = &mut self.cells[coord.0][coord.1];
            }

            let new_human = Human::new(i as u32, coord);

            // Record original contents of cell as 'covered'
            chosen_cell.covered_val = Some(chosen_cell.val);
            chosen_cell.covered_type = Some(chosen_cell.cell_type.clone());



            chosen_cell.cell_type = CellType::Human {id: i as u32};
            chosen_cell.val = '☺';

            self.humans.push(new_human);
        }
    }

    pub fn update_entities(&mut self) {

        // Get decisions from humans
        let human_decisions: Vec<(u32, Action)> = self.humans
            .iter()
            .map(|h| h.decide(self))
            .collect();

        //println!("Human decisions: {:?}", human_decisions);
        //panic!();

        // Apply decisions to humans
        human_decisions
            .iter()
            .for_each(|d| {

                let r = match d.1 {
                    Action::None => Ok(()),
                    Action::Move(new_coord) => self.move_entity(self.humans[d.0 as usize].get_coord(), new_coord),
                };

                // Now update internal log for human
                if r.is_ok() {
                    self.humans[d.0 as usize].act(&d.1);
                }
            });

    }

    pub fn is_walkable(&self, coord: (usize, usize)) -> bool {
        match self.cells[coord.0][coord.1].cell_type {
            CellType::Water => false,
            CellType::Sand => true,
            CellType::Grass => true,
            CellType::TreeTrunk { id: _ } => false,
            CellType::Leaves { id: _ } => true,
            CellType::Human { id: _ } => false,
        }
    }

    pub fn is_in_bounds(&self, coord: (usize, usize)) -> bool {
        // We already know the coords are positive due to the usize type bound
        return coord.0 < self.size.0 && coord.1 < self.size.1;
    }

    fn move_entity(&mut self, coord_1: (usize, usize), coord_2: (usize, usize)) -> Result<(), ()> {
        
        if self.cells[coord_1.0][coord_1.1].is_moveable() && self.is_walkable(coord_2) {

            // Move coord_2 stuff to the background
            self.cells[coord_2.0][coord_2.1].covered_val = Some(self.cells[coord_2.0][coord_2.1].val);
            self.cells[coord_2.0][coord_2.1].covered_type = Some(self.cells[coord_2.0][coord_2.1].cell_type.clone());


            // Now write to the 'top layer' of coord_2
            self.cells[coord_2.0][coord_2.1].cell_type = self.cells[coord_1.0][coord_1.1].cell_type.clone();
            self.cells[coord_2.0][coord_2.1].val = self.cells[coord_1.0][coord_1.1].val;

            // Reset cell at coord_1
            if let Some(t) = &self.cells[coord_1.0][coord_1.1].covered_type {
                self.cells[coord_1.0][coord_1.1].cell_type = t.clone();
                self.cells[coord_1.0][coord_1.1].covered_type = None;
            }
            if let Some(v) = &self.cells[coord_1.0][coord_1.1].covered_val {
                self.cells[coord_1.0][coord_1.1].val = v.clone();
                self.cells[coord_1.0][coord_1.1].covered_val = None;
            }
            return Ok(());
        }

        Err(())
    }
}


