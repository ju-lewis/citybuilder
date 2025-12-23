
use termion::color::{self, Fg};

#[derive(Clone)]
pub enum CellType {
    Water,
    Sand,
    Grass,
    // TODO: Add more cell types
}


/*------------------------------ Map Cell Code -------------------------------*/

#[derive(Clone)]
pub struct MapCell {
    cell_type: CellType
}

impl MapCell {
    pub fn render(&self) -> String {
        match self.cell_type {
            CellType::Water => format!("{}~",  Fg(color::Blue)),
            CellType::Sand  => format!("{}#",  Fg(color::Yellow)),
            CellType::Grass => format!("{}\"", Fg(color::LightGreen)),
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
            cell_type: CellType::Grass
        };

        Map {
            cells: vec![vec![c; size.1.into()]; size.0.into()],
            size
        }
    }
}


