
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
pub struct Cell {
    items: Vec<Item>
}

impl Cell {
    fn new(items: Vec<Item>) -> Self {
        Cell {
            items
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(vec![])
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        let mut cell =  Cell {
            items: vec![]
        };
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