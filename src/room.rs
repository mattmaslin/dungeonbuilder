use point::Point;
use chunk::Chunk;

pub struct Room {
    upper_right: Point,
    lower_left: Point
}

impl Room {
    pub fn new(chunk: Chunk) -> Room {
        Room { upper_right: chunk.upper_right().clone(), lower_left: chunk.lower_left().clone() }
    }

    pub fn upper_right(&self) -> &Point {
        &self.upper_right
    }

    pub fn lower_left(&self) -> &Point {
        &self.lower_left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;
    use chunk::{ChunkSplit, Chunk};

    #[test]
    fn test_new() {
        let lower_left = Point::new(1f32,2f32);
        let upper_right = Point::new(22f32,21f32);
        let chunk = Chunk::new(lower_left, upper_right, ChunkSplit::Horizontal);
        let room = Room::new(chunk);
        assert_eq!(1f32, room.lower_left().x());
        assert_eq!(2f32, room.lower_left().y());
        assert_eq!(22f32, room.upper_right().x());
        assert_eq!(21f32, room.upper_right().y());        
    }
}
