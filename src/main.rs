mod game;
mod game_view;

use crate::game::game_state::GameState;
use crate::game_view::GameView;

fn main() {
    let mut state = GameState::new(500, 500);
    state.init();

    let mut view = GameView::new(state);
    view.init().expect("Error initializing the main game loop.");

}
