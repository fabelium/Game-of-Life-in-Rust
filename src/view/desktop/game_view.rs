use crate::game::cell_state::CellState;
use crate::game::game_state::GameState;
use crate::view::game_view_trait::GameViewTrait;

use opengl_graphics::GlGraphics;
use graphics::{clear, rectangle};
use std::time::{Duration, Instant};
use piston_window::{Events, EventSettings, OpenGL, PistonWindow, RenderArgs, RenderEvent, WindowSettings};

pub struct GameView {
    game_state: GameState,
    cell_size: f64,
    update_interval: Duration,
    gl: GlGraphics,
    window: PistonWindow,
}

impl GameView {
    fn render(&mut self, args: &RenderArgs) {
        const ALIVE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const DEAD_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |context, graphics| {
            clear(DEAD_COLOR, graphics);

            for (x, row) in self.game_state.board.cells.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    let square = rectangle::square(x as f64 * self.cell_size, y as f64 * self.cell_size, self.cell_size);
                    let color = match cell {
                        CellState::Alive => ALIVE_COLOR,
                        CellState::Dead => DEAD_COLOR,
                    };
                    rectangle(color, square, context.transform, graphics);
                }
            }
        });
    }

    fn update(&mut self) {
        self.game_state.update();
    }
}

impl GameViewTrait for GameView {
    fn new(game_state: GameState, cell_size: f64, update_interval_ms: u64) -> Self {
        let opengl = OpenGL::V3_2;

        let size = [
            game_state.board.width as u32,
            game_state.board.height as u32
        ];

        let window: PistonWindow = WindowSettings::new("Game of Life", size)
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Self {
            game_state,
            cell_size,
            update_interval: Duration::from_millis(update_interval_ms),
            gl: GlGraphics::new(opengl),
            window,
        }
    }

    fn init(&mut self) -> Result<(), String> {
        let mut events = Events::new(EventSettings::new());
        let mut last_update = Instant::now();

        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if last_update.elapsed() >= self.update_interval {
                self.update();
                last_update = Instant::now();
            }
        }
        Ok(())
    }
}
