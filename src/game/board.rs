use crate::game::rules::evolve_cell;
use crate::game::cell_state::CellState;

use rand::{Rng, thread_rng};


#[derive(Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![vec![CellState::Dead; height]; width];
        Board { width, height, cells }
    }

    pub fn set_initial_state(&mut self, initial_alive_probability: f64) {
        let mut rng = thread_rng();
        for x in 0..self.width {
            for y in 0..self.height {
                if rng.gen_bool(initial_alive_probability) {
                    self.set_cell(x, y, CellState::Alive);
                }
            }
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> CellState {
        if x < self.width && y < self.height {
            return self.cells[x][y];
        }
        CellState::Dead
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell_state: CellState) {
        if x < self.width && y < self.height {
            self.cells[x][y] = cell_state;
        }
    }

    pub fn update(&mut self) {
        let mut new_state = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                let new_cell = evolve_cell(self.get_cell(x, y), alive_neighbors);
                new_state[x][y] = new_cell;
            }
        }

        self.cells = new_state;
    }

    fn is_coord_in_board(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        if x >= self.width as isize {
            return false;
        }
        if x >= self.height as isize {
            return false;
        }
        true
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut neighbors = 0;
        for i in 0..3 {
            for j in 0..3 {
                if !(i == 1 && j == 1) {
                    let coord_x_neighbor = x as isize + i as isize - 1;
                    let coord_y_neighbor = y as isize + j as isize - 1;
                    if self.is_coord_in_board(coord_x_neighbor, coord_y_neighbor) {
                        if self.get_cell(coord_x_neighbor as usize, coord_y_neighbor as usize) == CellState::Alive {
                            neighbors += 1;
                        }
                    }
                }
            }
        }
        neighbors
    }

    #[allow(dead_code)]
    fn place_r_pentomino(&mut self, x: usize, y: usize) {
        // 0 X X
        // X X 0
        // 0 X 0
        self.set_cell(x, y, CellState::Dead);
        self.set_cell(x, y + 1, CellState::Alive);
        self.set_cell(x, y + 2, CellState::Dead);

        self.set_cell(x + 1, y, CellState::Alive);
        self.set_cell(x + 1, y + 1, CellState::Alive);
        self.set_cell(x + 1, y + 2, CellState::Alive);

        self.set_cell(x + 2, y, CellState::Alive);
        self.set_cell(x + 2, y + 1, CellState::Dead);
        self.set_cell(x + 2, y + 2, CellState::Dead);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new(10, 20);
        assert_eq!(board.width, 10);
        assert_eq!(board.height, 20);
        assert_eq!(board.cells.len(), 10);
        assert_eq!(board.cells[0].len(), 20);
    }

    #[test]
    fn test_initial_state() {
        let mut has_alive = false;
        let mut has_dead = false;

        let mut board = Board::new(10, 10);
        board.set_initial_state(0.5);

        for row in board.cells.iter() {
            for cell in row {
                match cell {
                    CellState::Alive => has_alive = true,
                    CellState::Dead => has_dead = true,
                }
            }
        }

        assert!(has_alive, "There should be at least one alive cell.");
        assert!(has_dead, "There should be at least one dead cell.");
    }

    #[test]
    fn test_initial_state_all_dead() {
        let mut has_alive = false;
        let mut has_dead = false;

        let mut board = Board::new(10, 10);
        board.set_initial_state(0.0);

        for row in board.cells.iter() {
            for cell in row {
                match cell {
                    CellState::Alive => has_alive = true,
                    CellState::Dead => has_dead = true,
                }
            }
        }
        assert!(!has_alive, "There should be no alive cells.");
        assert!(has_dead, "There should be at least one dead cell.");
    }

    #[test]
    fn test_set_cell() {
        let mut board = Board::new(10, 10);
        assert_eq!(board.get_cell(2, 3), CellState::Dead);
        board.set_cell(2, 3, CellState::Alive);
        assert_eq!(board.get_cell(2, 3), CellState::Alive);
    }

    #[test]
    fn test_all_neighbors_alive() {
        let mut board = Board::new(3, 3);
        for y in 0..3 {
            for x in 0..3 {
                if x != 1 || y != 1 {
                    board.set_cell(x, y, CellState::Alive);
                }
            }
        }

        let alive_neighbors = board.count_alive_neighbors(1, 1);
        assert_eq!(alive_neighbors, 8, "Cell (1, 1) should have 8 alive neighbors.");
    }

    #[test]
    fn test_all_neighbors_dead() {
        let board = Board::new(3, 3);

        let alive_neighbors = board.count_alive_neighbors(1, 1);
        assert_eq!(alive_neighbors, 0, "Cell (1, 1) should have 0 alive neighbors.");
    }

    #[test]
    fn test_four_alive_neighbors() {
        let mut board = Board::new(3, 3);
        board.set_cell(0, 0, CellState::Alive);
        board.set_cell(0, 1, CellState::Alive);
        board.set_cell(1, 0, CellState::Alive);
        board.set_cell(1, 2, CellState::Alive);

        let alive_neighbors = board.count_alive_neighbors(1, 1);
        assert_eq!(alive_neighbors, 4, "Cell (1, 1) should have 4 alive neighbors.");
    }


    #[test]
    fn test_board_update() {
        let mut board = Board::new(5, 5);

        // Initial state:
        // 0 0 0 0 0
        // 0 0 0 0 0
        // 0 X X X 0
        // 0 0 0 0 0
        // 0 0 0 0 0
        board.set_cell(1, 2, CellState::Alive);
        board.set_cell(2, 2, CellState::Alive);
        board.set_cell(3, 2, CellState::Alive);

        board.update();
        // Expected state:
        // 0 0 0 0 0
        // 0 0 X 0 0
        // 0 0 X 0 0
        // 0 0 X 0 0
        // 0 0 0 0 0
        assert_eq!(board.get_cell(2, 1), CellState::Alive);
        assert_eq!(board.get_cell(2, 2), CellState::Alive);
        assert_eq!(board.get_cell(2, 3), CellState::Alive);

        assert_eq!(board.get_cell(1, 2), CellState::Dead);
        assert_eq!(board.get_cell(3, 2), CellState::Dead);
    }

    #[test]
    fn test_r_pentomino_evolution() {
        let mut board = Board::new(6, 6);

        // Initial state:
        // 0 0 0 0 0
        // 0 0 X X 0
        // 0 X X 0 0
        // 0 0 X 0 0
        // 0 0 0 0 0
        board.place_r_pentomino(1, 1);

        board.update();
        // 0 0 0 0 0
        // 0 X X X 0
        // 0 X 0 0 0
        // 0 X X 0 0
        // 0 0 0 0 0
        assert_eq!(board.get_cell(0, 0), CellState::Dead);
        assert_eq!(board.get_cell(0, 1), CellState::Dead);
        assert_eq!(board.get_cell(0, 2), CellState::Dead);
        assert_eq!(board.get_cell(0, 3), CellState::Dead);
        assert_eq!(board.get_cell(0, 4), CellState::Dead);

        assert_eq!(board.get_cell(1, 0), CellState::Dead);
        assert_eq!(board.get_cell(1, 1), CellState::Alive);
        assert_eq!(board.get_cell(1, 2), CellState::Alive);
        assert_eq!(board.get_cell(1, 3), CellState::Alive);
        assert_eq!(board.get_cell(1, 4), CellState::Dead);

        assert_eq!(board.get_cell(2, 0), CellState::Dead);
        assert_eq!(board.get_cell(2, 1), CellState::Alive);
        assert_eq!(board.get_cell(2, 2), CellState::Dead);
        assert_eq!(board.get_cell(2, 3), CellState::Alive);
        assert_eq!(board.get_cell(2, 4), CellState::Dead);

        assert_eq!(board.get_cell(3, 0), CellState::Dead);
        assert_eq!(board.get_cell(3, 1), CellState::Alive);
        assert_eq!(board.get_cell(3, 2), CellState::Dead);
        assert_eq!(board.get_cell(3, 3), CellState::Dead);
        assert_eq!(board.get_cell(3, 4), CellState::Dead);

        assert_eq!(board.get_cell(4, 0), CellState::Dead);
        assert_eq!(board.get_cell(4, 1), CellState::Dead);
        assert_eq!(board.get_cell(4, 2), CellState::Dead);
        assert_eq!(board.get_cell(4, 3), CellState::Dead);
        assert_eq!(board.get_cell(4, 4), CellState::Dead);

        board.update();
        // 0 0 X 0 0
        // 0 X X 0 0
        // X 0 0 X 0
        // 0 X X 0 0
        // 0 0 0 0 0
        assert_eq!(board.get_cell(0, 2), CellState::Alive);
        assert_eq!(board.get_cell(1, 1), CellState::Alive);
        assert_eq!(board.get_cell(1, 3), CellState::Alive);
        assert_eq!(board.get_cell(2, 0), CellState::Alive);
        assert_eq!(board.get_cell(2, 1), CellState::Alive);
        assert_eq!(board.get_cell(2, 3), CellState::Alive);
        assert_eq!(board.get_cell(3, 2), CellState::Alive);
    }
}
