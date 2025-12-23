
use termion::color::{self, Fg};

use crate::entities::human::Human;

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

    //pub covered_type: Option<CellType>,
    //pub covered_val: Option<char>,
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
            val: '\"'
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
            loop {
                if self.cells[coord.0][coord.1].cell_type == CellType::Grass {
                    break;
                }

                coord.0 = rand::random_range(0..self.size.0);
                coord.1 = rand::random_range(0..self.size.1);
            }

            let new_human = Human::new(i as u32, coord);


            self.cells[coord.0][coord.1].cell_type = CellType::Human {id: i as u32};


            self.humans.push(new_human);
        }

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
}


