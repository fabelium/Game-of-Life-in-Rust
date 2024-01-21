use std::time::{Duration};
use piston_window::RenderArgs;
use crate::game::game_state::GameState;
use crate::views::game_view_trait::GameViewTrait;

pub struct WasmGameView {
    game_state: GameState,
    cell_size: f64,
    update_interval: Duration,
}

impl GameViewTrait for WasmGameView {
    fn new(game_state: GameState, cell_size: f64, update_interval_ms: u64) -> Self {
        Self {
            game_state,
            cell_size,
            update_interval: Duration::from_millis(update_interval_ms),
        }
    }

    fn init(&mut self) -> Result<(), String> {
        unimplemented!();
    }

    fn render(&mut self, args: &RenderArgs) {
        unimplemented!()
    }

    fn update(&mut self) {
        self.game_state.update();
    }
}