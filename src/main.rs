mod game;
mod view;

use std::{env, io};
use std::fs::File;
use std::io::BufRead;

use crate::view::game_view::GameView;
use crate::game::game_state::GameState;
use crate::view::game_view_trait::GameViewTrait;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = if args.len() > 1 {
        let pattern_filename = &args[1];
        Some(read_file(pattern_filename).unwrap())
    } else {
        None
    };
    let cell_size: u8 = 3;

    initialize_game(pattern, cell_size);
}

fn initialize_game(pattern: Option<Vec<String>>, cell_size: u8) {
    let initial_alive_probability = 0.1;

    let update_interval_ms: usize = 300;
    let board_width: usize = 600;
    let board_height: usize = 500;

    let mut game_state = GameState::new(board_width, board_height, cell_size as usize);

    initialize_game_state(&mut game_state, pattern, initial_alive_probability);

    let mut view = Box::new(GameView::new(game_state, cell_size as usize, update_interval_ms));
    view.init().expect("Error initializing the main game loop.");
}

fn initialize_game_state(game_state: &mut GameState, pattern: Option<Vec<String>>, initial_alive_probability: f64) {
    match pattern {
        Some(p) => {
            game_state.add_pattern(p);
        }
        None => {
            game_state.init(initial_alive_probability);
        }
    }
}

fn read_file(filename: &str) -> io::Result<Vec<String>> {
    match File::open(filename) {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            Ok(reader.lines().filter_map(Result::ok).collect())
        }
        Err(e) => {
            Err(e)
        }
    }
}