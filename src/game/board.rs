use crate::game::cell::Cell;
use rand::{thread_rng, Rng};

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![vec![Cell::Dead; width]; height];
        Board { width, height, cells }
    }

    pub fn set_initial_state(&mut self) {
        let mut rng = thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                let alive = rng.gen_bool(0.5);
                self.cells[y][x] = if alive { Cell::Alive } else { Cell::Dead };
            }
        }
    }

    pub fn set_cell_state(&mut self, x: usize, y: usize, state: Cell) {
        if x < self.width && y < self.height {
            self.cells[x][y] = state;
        }
    }

    pub fn update(&mut self) {
        let mut new_state = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.count_alive_neighbors(x, y);

                let new_cell = match (self.cells[y][x], alive_neighbors) {
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Alive, 2) => Cell::Alive,
                    (Cell::Alive, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                new_state[y][x] = new_cell;
            }
        }

        self.cells = new_state;
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in 0..3 {
            for j in 0..3 {
                if !(i == 1 && j == 1) {
                    let dx = x as isize + i as isize - 1;
                    let dy = y as isize + j as isize - 1;
                    if dx >= 0 && dx < self.width as isize && dy >= 0 && dy < self.height as isize {
                        if self.cells[dy as usize][dx as usize] == Cell::Alive {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
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
        assert_eq!(board.cells.len(), 20);
        assert_eq!(board.cells[0].len(), 10);
    }

    #[test]
    fn test_initial_state() {
        let mut board = Board::new(10, 10);
        board.set_initial_state();
        let mut has_alive = false;
        let mut has_dead = false;

        for row in board.cells.iter() {
            for cell in row {
                match cell {
                    Cell::Alive => has_alive = true,
                    Cell::Dead => has_dead = true,
                }
            }
        }

        assert!(has_alive, "There should be at least one alive cell.");
        assert!(has_dead, "There should be at least one dead cell.");
    }

    #[test]
    fn test_set_cell_state() {
        let mut board = Board::new(10, 10);
        assert_eq!(board.cells[2][3], Cell::Dead);
        board.set_cell_state(2, 3, Cell::Alive);
        assert_eq!(board.cells[2][3], Cell::Alive);
    }

    #[test]
    fn test_all_neighbors_alive() {
        let mut board = Board::new(3, 3);
        for y in 0..3 {
            for x in 0..3 {
                if x != 1 || y != 1 {
                    board.set_cell_state(x, y, Cell::Alive);
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
        board.set_cell_state(0, 0, Cell::Alive);
        board.set_cell_state(0, 1, Cell::Alive);
        board.set_cell_state(1, 0, Cell::Alive);
        board.set_cell_state(1, 2, Cell::Alive);

        let alive_neighbors = board.count_alive_neighbors(1, 1);
        assert_eq!(alive_neighbors, 4, "Cell (1, 1) should have 4 alive neighbors.");
    }


    #[test]
    fn test_board_update() {
        let mut board = Board::new(5, 5);

        // Initial state:
        // 0 0 0 0 0
        // 0 0 0 0 0
        // 0 x x x 0
        // 0 0 0 0 0
        // 0 0 0 0 0
        board.set_cell_state(2, 1, Cell::Alive);
        board.set_cell_state(2, 2, Cell::Alive);
        board.set_cell_state(2, 3, Cell::Alive);

        board.update();

        // Expected state:
        // 0 0 0 0 0
        // 0 0 x 0 0
        // 0 0 x 0 0
        // 0 0 x 0 0
        // 0 0 0 0 0
        assert_eq!(board.cells[1][2], Cell::Alive, "Cell (1, 2) should be alive.");
        assert_eq!(board.cells[2][2], Cell::Alive, "Cell (2, 2) should be alive.");
        assert_eq!(board.cells[3][2], Cell::Alive, "Cell (3, 2) should be alive.");

        assert_eq!(board.cells[1][1], Cell::Dead, "Cell (1, 1) should be dead.");
        assert_eq!(board.cells[1][3], Cell::Dead, "Cell (1, 3) should be dead.");
    }
}
