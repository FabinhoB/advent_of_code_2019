#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point{
    pub fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }

    pub fn manhattan_distance(&self, p: &Point) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }

    pub fn equals(&self, p: &Point) -> bool {
        (self.x == p.x) && (self.y == p.y)
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}