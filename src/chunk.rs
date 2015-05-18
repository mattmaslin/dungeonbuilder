use point::Point;
use hallway::Hallway;
use rand::Rng;

enum ChunkSplit {
    Vertical,
    Horizontal,
}

pub struct Chunk {
    lower_left: Point,
    upper_right: Point
}

impl Chunk {
    pub fn new(lower_left: Point, upper_right: Point) -> Chunk {
        Chunk { lower_left: lower_left, upper_right: upper_right }
    }

    pub fn width(&self) -> f32 {
        self.upper_right.x() - self.lower_left.x()
    }

    pub fn height(&self) -> f32 {
        self.upper_right.y() - self.lower_left.y()
    }

    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }

    fn cant_split(&self, max_size: f32) -> bool {
        self.width() < (max_size * 2.0f32) && self.height() < (max_size * 2.0f32)
    }

    pub fn split<T: Rng + ?Sized>(&mut self, max_size: f32, rng: &mut T) -> (Option<Chunk>, ChunkSplit) {
        let mut split_horizontal = true;
        if self.cant_split(max_size) {
            return (None, ChunkSplit::Horizontal)
        }
        if self.width() < (max_size * 2.0f32) {
            split_horizontal = false;
        } else if self.height() >= (max_size * 2.0f32) {
            split_horizontal = rng.gen_weighted_bool(2);
        }
        if split_horizontal {
            let mut min = self.lower_left.x() + max_size;
            if self.width() > (max_size * 2.0f32) {
                min = min + 1.0f32;
            }
            let max = self.upper_right.x() - max_size + 1.0f32;
            let mut split_x = min;
            if min < max {
                split_x = rng.gen_range(min, max);
            } else if min > max {
                panic!("Min is greater than max");
            }
            let upper_right = self.upper_right.clone();
            let lower_left = Point::new(split_x, self.lower_left.y());
            self.upper_right.set_x(split_x);
            (Some(Chunk::new(lower_left, upper_right)), ChunkSplit::Horizontal)
        } else {
            let mut min = self.lower_left.y() + max_size;
            if self.height() > (max_size * 2.0f32) {
                min = min + 1.0f32;
            }
            let max = self.upper_right.y() - max_size + 1.0f32;            
            let mut split_y = min;
            if min < max {
                split_y = rng.gen_range(min, max);
            } else if min > max {
                panic!("Min is greater than max");
            }

            let split_y = rng.gen_range(min, max);
            let upper_right = self.upper_right.clone();
            let lower_left = Point::new(self.lower_left.x(), split_y);
            self.upper_right.set_y(split_y);
            (Some(Chunk::new(lower_left, upper_right)), ChunkSplit::Vertical)
        }
    }

    pub fn lower_left(&self) -> &Point {
        &self.lower_left
    }

    pub fn upper_right(&self) -> &Point {
        &self.upper_right
    }

    pub fn strip_hallway(&mut self, side: ChunkSplit) -> Hallway {
        match side {
            ChunkSplit::Vertical => {
                let lower_left = Point::new(self.lower_left.x(), self.upper_right.y() - 1f32);
                let upper_right = Point::new(self.upper_right.x(), self.upper_right.y());
                self.upper_right.add(Point::new(0f32, -1f32));
                Hallway::new(lower_left, upper_right)
            },
            ChunkSplit::Horizontal => {
                let lower_left = Point::new(self.upper_right.x() - 1f32, self.lower_left.y());
                let upper_right = Point::new(self.upper_right.x(), self.upper_right.y());
                self.upper_right.add(Point::new(-1f32, 0f32));
                Hallway::new(lower_left, upper_right)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chunk::ChunkSplit;
    use point::Point;
    use rand::distributions::range::SampleRange;
    use rand::Rng;

    struct MockRng;

    impl Rng for MockRng {
        fn next_u32(&mut self) -> u32 {
            return 2u32;
        }
    }

    #[test]
    fn test_new() {
        let lower_left = Point::new(2f32, 3f32);
        let upper_right = Point::new(20f32, 21f32);
        let chunk = Chunk::new(lower_left, upper_right); 
        assert_eq!(2f32, chunk.lower_left().x());
        assert_eq!(3f32, chunk.lower_left().y());
        assert_eq!(20f32, chunk.upper_right().x());
        assert_eq!(21f32, chunk.upper_right().y());
    }

    #[test]
    fn test_width() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 10f32);
        let chunk = Chunk::new(lower_left, upper_right); 
        assert_eq!(20f32, chunk.width());
    }

    #[test]
    fn test_height() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 10f32);
        let chunk = Chunk::new(lower_left, upper_right); 
        assert_eq!(10f32, chunk.height());
    }

    #[test]
    fn test_area() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 10f32);
        let chunk = Chunk::new(lower_left, upper_right); 
        assert_eq!(200f32, chunk.area());
    }

    #[test]
    fn test_split() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 20f32);
        let mut chunk = Chunk::new(lower_left, upper_right); 
        let mut mockrng = MockRng;
        let (new_chunk_option,split) = chunk.split(5f32, &mut mockrng); 
        let new_chunk = new_chunk_option.unwrap();
        assert!(match split {
            ChunkSplit::Horizontal => true,
            _ => false
        });
        assert_eq!(6f32, chunk.upper_right().x());
        assert_eq!(20f32, chunk.upper_right().y());
        assert_eq!(6f32, new_chunk.lower_left().x());
        assert_eq!(0f32, new_chunk.lower_left().y());
        assert_eq!(20f32, new_chunk.upper_right().x());
        assert_eq!(20f32, new_chunk.upper_right().y());
    }

    #[test]
    fn test_horizontal_split() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 2f32);
        let mut chunk = Chunk::new(lower_left, upper_right); 
        let mut mockrng = MockRng;
        let (new_chunk_option,split) = chunk.split(5f32, &mut mockrng); 
        let new_chunk = new_chunk_option.unwrap();
        assert!(match split {
            ChunkSplit::Horizontal => true,
            _ => false
        });
        assert_eq!(6f32, chunk.upper_right().x());
        assert_eq!(2f32, chunk.upper_right().y());
        assert_eq!(6f32, new_chunk.lower_left().x());
        assert_eq!(0f32, new_chunk.lower_left().y());
        assert_eq!(20f32, new_chunk.upper_right().x());
        assert_eq!(2f32, new_chunk.upper_right().y());
    }

    #[test]
    fn test_vertical_split() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(2f32, 20f32);
        let mut chunk = Chunk::new(lower_left, upper_right); 
        let mut mockrng = MockRng;
        let (new_chunk_option,split) = chunk.split(5f32, &mut mockrng); 
        let new_chunk = new_chunk_option.unwrap();
        assert!(match split {
            ChunkSplit::Vertical => true,
            _ => false
        });
        assert_eq!(2f32, chunk.upper_right().x());
        assert_eq!(6f32, chunk.upper_right().y());
        assert_eq!(0f32, new_chunk.lower_left().x());
        assert_eq!(6f32, new_chunk.lower_left().y());
        assert_eq!(2f32, new_chunk.upper_right().x());
        assert_eq!(20f32, new_chunk.upper_right().y());
    }

    #[test]
    fn test_cant_split() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(2f32, 2f32);
        let mut chunk = Chunk::new(lower_left, upper_right); 
        let mut mockrng = MockRng;
        let (new_chunk_option,split) = chunk.split(5f32, &mut mockrng); 
        assert!(new_chunk_option.is_none());
    }


    #[test]
    fn test_strip_vertical_hallway() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 20f32);
        let mut chunk = Chunk::new(lower_left, upper_right); 
        let hallway = chunk.strip_hallway(ChunkSplit::Vertical);
        assert_eq!(19f32, chunk.upper_right.y());
        assert_eq!(20f32, chunk.upper_right.x());
        assert_eq!(19f32, hallway.lower_left().y());
        assert_eq!(0f32, hallway.lower_left().x());
        assert_eq!(20f32, hallway.upper_right().y());
    }

    #[test]
    fn test_strip_horizontal_hallway() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 20f32);
        let mut chunk = Chunk::new(lower_left, upper_right); 
        let hallway = chunk.strip_hallway(ChunkSplit::Horizontal);
        assert_eq!(20f32, chunk.upper_right.y());
        assert_eq!(19f32, chunk.upper_right.x());
        assert_eq!(0f32, hallway.lower_left().y());
        assert_eq!(19f32, hallway.lower_left().x());
        assert_eq!(20f32, hallway.upper_right().y());
    }
}
