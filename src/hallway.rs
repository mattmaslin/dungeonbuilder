use point::Point;

pub struct Hallway {
    lower_left: Point,
    upper_right: Point
}

impl Hallway {
    pub fn new(lower_left: Point, upper_right: Point) -> Hallway {
        Hallway { lower_left: lower_left, upper_right: upper_right }
    }

    pub fn lower_left(&self) -> &Point {
        &self.lower_left
    }

    pub fn upper_right(&self) -> &Point {
        &self.upper_right
    }
}
