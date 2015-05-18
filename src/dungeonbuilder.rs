use room::Room;
use hallway::Hallway;
use chunk::Chunk;
use point::Point;
use dungeon::Dungeon;
use rand::{SeedableRng, XorShiftRng};

pub struct DungeonBuilder {
    chunks: Vec<Chunk>,
    rng: XorShiftRng
}

impl DungeonBuilder  {
    pub fn new(seed: f32) -> DungeonBuilder  {
        DungeonBuilder  { chunks: Vec::new(), rng: XorShiftRng::from_seed([1u32,3u32,5u32,6u32]) }
    }

    pub fn in_area(&mut self, lower_left: Point, upper_right: Point) -> &mut DungeonBuilder {
        let chunk = Chunk::new(lower_left, upper_right);
        self.chunks.push(chunk);
        self        
    }

    pub fn build(&mut self) -> Dungeon {
        let mut dungeon = Dungeon::new();
        while self.chunks.len() > 0 {
            let mut chunk = self.chunks.pop().expect("chunk not found in queue");
            let (new_chunk_option, chunksplit) = chunk.split(15f32, &mut self.rng);
            match new_chunk_option {
                Some(new_chunk) => {
                    self.chunks.push(chunk);
                    self.chunks.push(new_chunk);
                },
                None => {
                    dungeon.add_room(Room::new(chunk));
                }
            }
        }
        dungeon
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

    #[test]
    fn test_build() {
        let dungeon = DungeonBuilder::new(10f32).in_area(Point::new(0f32,0f32), Point::new(1000f32,1000f32)).build();

    }
}
