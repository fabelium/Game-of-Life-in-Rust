mod game;

use crate::game::game_state::GameState;

fn main() {
    let mut state = GameState::new(500, 500);
    state.init();
}
