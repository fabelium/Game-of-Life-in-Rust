#[allow(dead_code)]
pub struct Cell {
    x: f64,
    y: f64,
    color: String,
}

impl Cell {

    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, color: &str) -> Cell {
        let color = color.to_string();
        Self { x, y, color }
    }

    #[allow(dead_code)]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    #[allow(dead_code)]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[allow(dead_code)]
    pub fn get_color(&self) -> &str {
        &self.color
    }
}