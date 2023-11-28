
use rand::Rng;

#[derive(Debug)]
struct Point(usize, usize);

#[derive(Debug)]
struct Player {
    backpack: Vec<Item>,
    location: Point,
    money: i32,
}

impl Player {
    fn new(starting_location: Point, starting_money: i32, starting_items: Vec<Item>) -> Player {
        Player {
            location: starting_location,
            money: starting_money,
            backpack: starting_items,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new(Point(0, 0), 0, vec![])
    }
}
#[derive(Copy, Clone, Debug)]
enum Item {
    Arrow,
    Coin
}

struct Cell {
    items: Vec<Item>
}


/// For fun we are implementing the cells of the board to be a flat array
/// In the future this can be converted to a 2d array
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
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

    fn get_random_start(&self) -> Point {
        let random = rand::thread_rng().gen_range(0..self.width*self.height);
        self.num_to_point(random)
    }

    fn get_cell(&self, point: Point) -> &Cell {
        let xy = point.0*point.1 % self.width*self.height; // wrap around
        &self.cells[xy]
    }
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


fn main() {
    let width = 10;
    let height = 4;
    let board = Board::new(width, height);
    let mut player = Player::default();
    player.location = board.get_random_start();
    println!("Width: {width}, Height: {height}");
    println!("Player Location: {:?}", player.location);
    println!("Number of cells: {}", board.cells.len());
}