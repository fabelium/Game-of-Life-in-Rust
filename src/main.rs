mod game;
mod view;


use crate::view::game_view::GameView;
use crate::game::game_state::GameState;
use crate::view::game_view_trait::GameViewTrait;

fn main() {
    let initial_alive_probability = 0.1;
    let cell_size = 2.0;
    let update_interval_ms = 100;
    let board_width = 600;
    let board_height = 500;

    let mut game_state = GameState::new(board_width, board_height);
    game_state.init(initial_alive_probability);

    let mut view = Box::new(GameView::new(game_state, cell_size, update_interval_ms));
    view.init().expect("Error initializing the main game loop.");
}