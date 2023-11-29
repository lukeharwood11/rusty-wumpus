pub mod player;
pub mod items;
pub mod board;

enum GameState {
    InProgress,
    Complete(bool) // boolean states whether or not the user won
}

use board::Board;
pub fn initialize_board(width: usize, height: usize) -> Board {
    let board = Board::new(width, height);
    board
}

pub fn start() {
    let mut board = initialize_board(10, 10);
    let mut game_state = GameState::InProgress;
    println!("Starting!");
    loop {
        /// first print the board
        break;
    }
}
