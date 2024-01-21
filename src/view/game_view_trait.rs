use piston_window::RenderArgs;
use crate::game::game_state::GameState;

pub trait GameViewTrait {
    fn new(game_state: GameState, cell_size: f64, update_interval_ms: u64) -> Self where Self: Sized;
    fn init(&mut self) -> Result<(), String>;
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self);
}