use crate::game::board::Board;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
}

impl GameState {
    pub fn new(width: usize, height: usize, cell_size: usize) -> Self {
        let grid_width = width / cell_size;
        let grid_height = height / cell_size;

        GameState {
            board: Board::new(grid_width, grid_height),
        }
    }

    pub fn init(&mut self, initial_alive_probability: f64) {
        self.board.set_initial_state(initial_alive_probability);
    }

    pub fn add_pattern(&mut self, pattern: Vec<String>) {
        self.board.add_pattern(pattern);
    }

    pub fn update(&mut self) {
        self.board.update();
    }
}