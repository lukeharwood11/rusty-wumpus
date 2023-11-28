use crate::game::items::Item;
use crate::util::Point;
#[derive(Debug)]
pub struct Player {
    backpack: Vec<Item>,
    pub location: Point,
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
