mod game;
mod views;

use crate::game::game_state::GameState;
use crate::views::GameView;

fn main() {
    let initial_alive_probability = 0.1;

    let mut game_state = GameState::new(500, 500);
    game_state.init(initial_alive_probability);

    let cell_size = 3.0;
    let update_interval_ms = 5;

    let mut view = GameView::new(game_state, cell_size, update_interval_ms);
    view.init().expect("Error initializing the main game loop.");
}
