
use termion::color::{self, Fg};

use super::generation::generate_map;

#[derive(Clone, PartialEq)]
pub enum CellType {
    Water,
    Sand,
    Grass,
    TreeTrunk,
    Leaves,
    // TODO: Add more cell types
}


/*------------------------------ Map Cell Code -------------------------------*/

#[derive(Clone)]
pub struct MapCell {
    pub cell_type: CellType,
    pub val: char
}

impl MapCell {
    pub fn render(&self) -> String {
        match self.cell_type {
            CellType::Water => format!("{}{}",  Fg(color::Cyan), self.val),
            CellType::Sand  => format!("{}{}",  Fg(color::Yellow), self.val),
            CellType::Grass => format!("{}{}", Fg(color::Green), self.val),
            CellType::TreeTrunk => format!("{}{}", Fg(color::Rgb(139, 69, 19)), self.val),
            CellType::Leaves => format!("{}{}", Fg(color::LightGreen), self.val),
        }
    }
}



/*--------------------------------- Map Code ---------------------------------*/

/// Data structure containing information about each cell.
/// Has a 1-1 correspondance with the framebuffer
pub struct Map {
    pub cells: Vec<Vec<MapCell>>,
    pub size: (usize, usize)
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
            size
        }
    }

    pub fn new(size: (usize, usize)) -> Self {
        generate_map(size)
    }
}


