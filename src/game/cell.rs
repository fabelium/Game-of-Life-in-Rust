#[allow(dead_code)]
pub struct Cell {
    x: usize,
    y: usize,
    color: String,
}

impl Cell {

    #[allow(dead_code)]
    pub fn new(x: usize, y: usize, color: &str) -> Cell {
        let color = color.to_string();
        Self { x, y, color }
    }

    #[allow(dead_code)]
    pub fn get_x(&self) -> usize {
        self.x
    }

    #[allow(dead_code)]
    pub fn get_y(&self) -> usize {
        self.y
    }

    #[allow(dead_code)]
    pub fn get_color(&self) -> &str {
        &self.color
    }
}