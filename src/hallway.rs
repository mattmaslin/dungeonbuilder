use point::Point;

pub struct Hallway {
    points: Vec<Point>,
}

impl Hallway {
    pub fn new(points: Vec<Point>) -> Hallway {
        Hallway { 
            points: points, 
        }
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
}
