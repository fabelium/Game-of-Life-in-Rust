use crate::game::board::Board;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        GameState {
            board: Board::new(width, height),
        }
    }

    pub fn init(&mut self, initial_alive_probability: f64) {
        self.board.set_initial_state(initial_alive_probability);
    }

    pub fn update(&mut self) {
        self.board.update();
    }
}