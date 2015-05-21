use point::Point;
use chunk::Chunk;

pub struct Hallway {
    points: Vec<Point>,
}

impl Hallway {
    pub fn new(chunk: Chunk) -> Hallway {
        Hallway { 
            points: vec![chunk.lower_left().clone(), 
            Point::new(chunk.lower_left().x(), chunk.upper_right().y()),
            chunk.upper_right().clone(), 
            Point::new(chunk.upper_right().x(), chunk.lower_left().y())] 
        }
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
}
