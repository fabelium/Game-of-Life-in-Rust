use crate::game::game_state::GameState;

pub trait GameViewTrait {
    fn new(game_state: GameState, cell_size: usize, update_interval_ms: usize) -> Self where Self: Sized;
    fn init(&mut self) -> Result<(), String>;
}