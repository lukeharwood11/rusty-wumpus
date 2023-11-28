
use rand::Rng;
use wumpus::game::board::Board;
use wumpus::game::player::Player;


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