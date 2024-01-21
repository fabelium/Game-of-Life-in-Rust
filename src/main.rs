mod game;
mod views;

use crate::game::game_state::GameState;
use crate::views::desktop::piston_game_view::PistonGameView;
use crate::views::game_view_trait::GameViewTrait;

fn main() {
    let initial_alive_probability = 0.1;

    let mut game_state = GameState::new(500, 500);
    game_state.init(initial_alive_probability);

    let cell_size = 3.0;
    let update_interval_ms = 5;

    let mut view = create_game_view(game_state, cell_size, update_interval_ms);
    view.init().expect("Error initializing the main game loop.");
}

fn create_game_view(game_state: GameState, cell_size: f64, update_interval_ms: u64) -> Box<dyn GameViewTrait> {
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(WebGameView::new(game_state))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(PistonGameView::new(game_state, cell_size, update_interval_ms))
    }
}