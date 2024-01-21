use crate::game::cell::Cell;

pub fn evolve_cell(current_state: Cell, alive_neighbors: u8) -> Cell {
    match (current_state, alive_neighbors) {
        (Cell::Dead, 3) => Cell::Alive,
        (Cell::Alive, 2) => Cell::Alive,
        (Cell::Alive, 3) => Cell::Alive,
        _ => Cell::Dead,
    }
}

#[test]
fn cell_comes_to_life() {
    assert_eq!(evolve_cell(Cell::Dead, 3), Cell::Alive, "A dead cell with exactly 3 live neighbors should come to life.");
}

#[test]
fn cell_stays_alive() {
    assert_eq!(evolve_cell(Cell::Alive, 2), Cell::Alive, "A live cell with 2 live neighbors should stay alive.");
    assert_eq!(evolve_cell(Cell::Alive, 3), Cell::Alive, "A live cell with 3 live neighbors should stay alive.");
}


#[test]
fn cell_dies() {
    assert_eq!(evolve_cell(Cell::Alive, 1), Cell::Dead, "A live cell with less than 2 live neighbors should die.");
    assert_eq!(evolve_cell(Cell::Alive, 4), Cell::Dead, "A live cell with more than 3 live neighbors should die.");
}

#[test]
fn cell_stays_dead() {
    assert_eq!(evolve_cell(Cell::Dead, 2), Cell::Dead, "A dead cell with fewer than 3 live neighbors should stay dead.");
    assert_eq!(evolve_cell(Cell::Dead, 4), Cell::Dead, "A dead cell with more than 3 live neighbors should stay dead.");
}