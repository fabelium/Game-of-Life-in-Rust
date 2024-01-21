use std::time::{Duration, Instant};
use opengl_graphics::GlGraphics;
use piston_window::{Events, EventSettings, OpenGL, PistonWindow, RenderArgs, RenderEvent, WindowSettings};
use crate::game::cell::Cell;
use crate::game::game_state::GameState;


pub struct GameView {
    game_state: GameState,
    gl: GlGraphics,
    window: PistonWindow,
}

impl GameView {
    pub fn new(game_state: GameState) -> Self {
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
            gl: GlGraphics::new(opengl),
            window,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        let mut events = Events::new(EventSettings::new());
        let mut last_update = Instant::now();
        let update_interval = Duration::from_millis(200);


        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if last_update.elapsed() >= update_interval {
                self.update();
                last_update = Instant::now();
            }
        }
        Ok(())
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::{clear, rectangle};

        const ALIVE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const DEAD_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const CELL_SIZE: f64 = 2.0;

        self.gl.draw(args.viewport(), |context, graphics| {
            clear(DEAD_COLOR, graphics);

            for (x, row) in self.game_state.board.cells.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    let square = rectangle::square(x as f64 * CELL_SIZE, y as f64 * CELL_SIZE, CELL_SIZE);
                    let color = match cell {
                        Cell::Alive => ALIVE_COLOR,
                        Cell::Dead => DEAD_COLOR,
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
