use std::fmt::Display;
use crate::util::Point;
use crate::game::items::Item;
use rand::Rng;

/// For fun we are implementing the cells of the board to be a flat array
/// In the future this can be converted to a 2d array
pub struct Board {
    width: usize,
    height: usize,
    pub cells: Vec<Cell>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            cells: vec![Cell::default(); width*height]
        }
    }

    fn num_to_point(&self, num: usize) -> Point {
        let y = num / self.width;        // get the row
        let x = num - (self.width*y);    // get the column
        Point(x, y)
    }

    pub fn get_random_start(&self) -> Point {
        let random = rand::thread_rng().gen_range(0..self.width*self.height);
        self.num_to_point(random)
    }

    fn get_cell(&self, point: Point) -> &Cell {
        let xy = point.0*point.1 % self.width*self.height; // wrap around
        &self.cells[xy]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for x in 0..self.width {
            let mut y_str = String::new();
            for y in 0..self.height {
                let cell = self.get_cell(Point(x, y));
                y_str.push_str(" | ");
                y_str.push_str(cell.to_string().as_str());
            }
            y_str.push_str(" | \n");
            ret.push_str(y_str.as_str());
        }
        write!(f, "{}", ret)
    }
}
pub struct Cell {
    items: Vec<Item>,
    visited: bool
}

const MAX_CELL_ITEMS: usize = 4;

impl Cell {
    fn new(items: Vec<Item>) -> Self {
        Cell {
            items,
            visited: false
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(vec![])
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for i in 0..MAX_CELL_ITEMS {
            let item = self.items.get(i);
            if self.visited {
                ret.push_str(
                    match item {
                        Some(Item::Arrow) => { "A" },
                        Some(Item::Coin) => { "C" },
                        None => { "_" }
                    }
                );
            } else {
                ret.push_str("?");
            }
        }
        write!(f, "{}", ret)
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        let mut cell =  Cell::default();
        self.items.iter().for_each(|i| {
            cell.items.push(*i);
        });
        cell
    }

    fn clone_from(&mut self, source: &Self) {
        self.items = vec![];
        source.items.iter().for_each(|i| {
            self.items.push(*i);
        });
    }
}