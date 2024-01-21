use crate::game::board::Board;
use crate::game::cell::Cell;

pub struct GameState {
    pub board: Board,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        GameState {
            board: Board::new(width, height),
        }
    }

    pub fn init(&mut self) {
        self.board.set_initial_state();
        self.board.set_cell_state(53, 53, Cell::Alive);
        self.board.set_cell_state(54, 53, Cell::Alive);
    }

    pub fn update(&mut self) {
        self.board.update();
    }
}