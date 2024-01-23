use crate::game::cell::Cell;
use crate::game::cell_state::CellState;
use crate::game::game_state::GameState;
use crate::view::game_view_trait::GameViewTrait;
use crate::view::web::util::*;
use crate::view::web::js::*;

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use std::time::{Duration};
use web_sys::HtmlCanvasElement;
use wasm_bindgen::closure::Closure;
use web_sys::{window, CanvasRenderingContext2d};

#[derive(Clone)]
pub struct GameView {
    game_state: GameState,
    cell_size: f64,
    update_interval: Duration,
    context: Option<CanvasRenderingContext2d>,
    last_update_time: f64,
}

impl GameView {
    const ALIVE_COLOR: &'static str = "black";
    const DEAD_COLOR: &'static str = "white";

    fn render(&mut self) {
        self.fill_background();

        let mut cells_to_draw = Vec::new();
        for (x, row) in self.game_state.board.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if cell == &CellState::Alive {
                    cells_to_draw.push(Cell::new(x as f64, y as f64, Self::ALIVE_COLOR));
                }
            }
        }
        for cell in cells_to_draw {
            self.draw_cell(cell);
        }
    }

    fn update(&mut self) {
        let start_time = performance_now();
        self.game_state.update();
        self.render();
        let end_time = performance_now();
        console_log(&format!("Rendering, took {} ms", end_time - start_time));
    }

    fn fill_background(&mut self) {
        let context = self.context.as_mut().unwrap();
        context.begin_path();
        context.rect(0.0, 0.0, self.game_state.board.width as f64, self.game_state.board.width as f64);
        context.set_fill_style(&Self::DEAD_COLOR.into());
        context.fill();
        context.stroke();
    }

    fn draw_cell(&mut self, cell: Cell) {
        let context = self.context.as_mut().unwrap();
        context.begin_path();
        context.rect(cell.get_x() * self.cell_size, cell.get_y() * self.cell_size, self.cell_size, self.cell_size);
        context.set_fill_style(&cell.get_color().into());
        context.fill();
        context.stroke();
    }

    fn render_loop_aux(self_rc: Rc<RefCell<Self>>) {
        let performance = window().unwrap().performance().unwrap();
        let current_time = performance.now();
        let mut borrow = self_rc.borrow_mut();

        if current_time - borrow.last_update_time >= borrow.update_interval.as_millis() as f64 {
            borrow.update();
            borrow.last_update_time = current_time;
        }
        drop(borrow);

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            GameView::render_loop_aux(self_rc.clone());
            let _ = f.borrow_mut().take();
        }) as Box<dyn FnMut()>));

        window().unwrap().request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref()).unwrap();
    }

    fn render_loop(&mut self) {
        let self_rc = Rc::new(RefCell::new(self.clone()));
        GameView::render_loop_aux(self_rc);
    }
}

impl GameViewTrait for GameView {
    fn new(game_state: GameState, cell_size: f64, update_interval_ms: u64) -> Self {
        console_log("New GameView");
        Self {
            game_state,
            cell_size,
            update_interval: Duration::from_millis(update_interval_ms),
            context: None,
            last_update_time: 0.0,
        }
    }

    fn init(&mut self) -> Result<(), String> {
        console_log("Initializing");
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

        canvas.set_width(self.game_state.board.width as u32);
        canvas.set_height(self.game_state.board.height as u32);

        self.context = Some(
            canvas.get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap()
        );
        self.render_loop();
        Ok(())
    }
}