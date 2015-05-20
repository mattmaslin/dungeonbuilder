use point::Point;
use hallway::Hallway;
use rand::Rng;
use dimensionoptions::DimensionOptions;
use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub enum ChunkSplit {
    Vertical,
    Horizontal,
}

pub struct Chunk {
    lower_left: Point,
    upper_right: Point,
    chunk_split: ChunkSplit
}

impl Chunk {
    pub fn new(lower_left: Point, upper_right: Point, chunk_split: ChunkSplit) -> Chunk {
        Chunk { lower_left: lower_left, upper_right: upper_right, chunk_split: chunk_split }
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

    fn can_split(&self, dimension_options: &DimensionOptions) -> bool {
        self.area() > (dimension_options.min_area * 2.0f32) && 
            (self.can_split_vertically(dimension_options) || self.can_split_horizontally(dimension_options))
    }

    fn can_split_vertically(&self, dimension_options: &DimensionOptions) -> bool {
        self.width() > (dimension_options.min_width * 2.0f32)
    }

    fn can_split_horizontally(&self, dimension_options: &DimensionOptions) -> bool {
        self.height() > (dimension_options.min_height * 2.0f32)
    }

    pub fn split<T: Rng + ?Sized>(&mut self, dimension_options: &DimensionOptions, rng: &mut T) -> Option<Chunk> {
        if !self.can_split(dimension_options) {
            return None;
        }
        match dimension_options.max_area {
            Some(max_area) => {
                if self.area() < max_area && rng.gen_weighted_bool(4) {
                    return None;
                }
            }
            _ => {}
        };
        let split_horizontal = match self.chunk_split {
            ChunkSplit::Horizontal => !self.can_split_vertically(dimension_options),
            ChunkSplit::Vertical => self.can_split_horizontally(dimension_options)
        };
        if !split_horizontal {
            let mut min = self.lower_left.x() + dimension_options.min_width;
            if self.width() > (dimension_options.min_width * 2.0f32) {
                min = min + 1.0f32;
            }
            let max = self.upper_right.x() - dimension_options.min_width + 1.0f32;
            let mut split_x = min;
            if min < max {
                split_x = rng.gen_range(min, max);
            } else if min > max {
                panic!("Min is greater than max");
            }
            let upper_right = self.upper_right.clone();
            let lower_left = Point::new(split_x, self.lower_left.y());
            self.upper_right.set_x(split_x);
            self.chunk_split = ChunkSplit::Vertical;
            Some(Chunk::new(lower_left, upper_right, ChunkSplit::Vertical))
        } else {
            let mut min = self.lower_left.y() + dimension_options.min_height;
            if self.height() > (dimension_options.min_height * 2.0f32) {
                min = min + 1.0f32;
            }
            let max = self.upper_right.y() - dimension_options.min_height + 1.0f32;            
            let mut split_y = min;
            if min < max {
                split_y = rng.gen_range(min, max);
            } else if min > max {
                panic!("Min is greater than max");
            }

            let upper_right = self.upper_right.clone();
            let lower_left = Point::new(self.lower_left.x(), split_y);
            self.upper_right.set_y(split_y);
            self.chunk_split = ChunkSplit::Horizontal;
            Some(Chunk::new(lower_left, upper_right, ChunkSplit::Horizontal))
        }
    }

    pub fn lower_left(&self) -> &Point {
        &self.lower_left
    }

    pub fn upper_right(&self) -> &Point {
        &self.upper_right
    }

    pub fn chunk_split(&self) -> ChunkSplit {
        self.chunk_split
    }

    pub fn strip_hallway(&mut self, side: ChunkSplit, hallway_width: f32) -> Hallway {
        match side {
            ChunkSplit::Horizontal => {
                let lower_left = Point::new(self.lower_left.x(), self.upper_right.y() - hallway_width);
                let upper_right = Point::new(self.upper_right.x(), self.upper_right.y());
                self.upper_right.add(Point::new(0f32, -hallway_width));
                Hallway::new(lower_left, upper_right)
            },
            ChunkSplit::Vertical => {
                let lower_left = Point::new(self.upper_right.x() - hallway_width, self.lower_left.y());
                let upper_right = Point::new(self.upper_right.x(), self.upper_right.y());
                self.upper_right.add(Point::new(-hallway_width, 0f32));
                Hallway::new(lower_left, upper_right)
            },
        }
    }
}

impl Ord for Chunk {
    fn cmp(&self, other: &Chunk) -> Ordering {
        if self.area() < other.area() {
            return Ordering::Less;
        } else if self.area() > other.area() {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Chunk {
    fn partial_cmp(&self, other: &Chunk) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Chunk {
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Chunk) -> bool {
        self.cmp(other) == Ordering::Equal
    }

    fn ne(&self, other: &Chunk) -> bool {
        !self.eq(other)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;
    use rand::Rng;
    use dimensionoptions::DimensionOptions;
    use hallwayoptions::HallwayOptions;

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
        let (new_chunk_option,split) = chunk.split(&DimensionOptions::new(5f32,5f32,5f32), &mut mockrng); 
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
        let (new_chunk_option,split) = chunk.split(&DimensionOptions::new(5f32,5f32,5f32), &mut mockrng); 
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
        let (new_chunk_option,split) = chunk.split(&DimensionOptions::new(5f32,5f32,5f32), &mut mockrng); 
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
        let (new_chunk_option,_) = chunk.split(&DimensionOptions::new(5f32,5f32,5f32), &mut mockrng); 
        assert!(new_chunk_option.is_none());
    }


    #[test]
    fn test_strip_vertical_hallway() {
        let lower_left = Point::new(0f32, 0f32);
        let upper_right = Point::new(20f32, 20f32);
        let mut chunk = Chunk::new(lower_left, upper_right); 
        let hallway = chunk.strip_hallway(ChunkSplit::Vertical, 1f32);
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
        let hallway = chunk.strip_hallway(ChunkSplit::Horizontal, 1f32);
        assert_eq!(20f32, chunk.upper_right.y());
        assert_eq!(19f32, chunk.upper_right.x());
        assert_eq!(0f32, hallway.lower_left().y());
        assert_eq!(19f32, hallway.lower_left().x());
        assert_eq!(20f32, hallway.upper_right().y());
    }
}
