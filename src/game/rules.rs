use crate::game::cell_state::CellState;

pub fn evolve_cell(current_state: CellState, alive_neighbors: u8) -> CellState {
    match (current_state, alive_neighbors) {
        (CellState::Dead, 3) => CellState::Alive,
        (CellState::Alive, 2) => CellState::Alive,
        (CellState::Alive, 3) => CellState::Alive,
        _ => CellState::Dead,
    }
}

#[test]
fn cell_comes_to_life() {
    assert_eq!(evolve_cell(CellState::Dead, 3), CellState::Alive, "A dead cell with exactly 3 live neighbors should come to life.");
}

#[test]
fn cell_stays_alive() {
    assert_eq!(evolve_cell(CellState::Alive, 2), CellState::Alive, "A live cell with 2 live neighbors should stay alive.");
    assert_eq!(evolve_cell(CellState::Alive, 3), CellState::Alive, "A live cell with 3 live neighbors should stay alive.");
}

#[test]
fn cell_dies() {
    assert_eq!(evolve_cell(CellState::Alive, 1), CellState::Dead, "A live cell with less than 2 live neighbors should die.");
    assert_eq!(evolve_cell(CellState::Alive, 4), CellState::Dead, "A live cell with more than 3 live neighbors should die.");
}

#[test]
fn cell_stays_dead() {
    assert_eq!(evolve_cell(CellState::Dead, 2), CellState::Dead, "A dead cell with fewer than 3 live neighbors should stay dead.");
    assert_eq!(evolve_cell(CellState::Dead, 4), CellState::Dead, "A dead cell with more than 3 live neighbors should stay dead.");
}