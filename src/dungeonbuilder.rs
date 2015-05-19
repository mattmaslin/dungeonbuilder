use room::Room;
use chunk::{Chunk, ChunkSplit};
use point::Point;
use dungeon::Dungeon;
use rand::{Rng, ThreadRng, thread_rng};
use dimensionoptions::DimensionOptions;
use hallwayoptions::HallwayOptions;

pub struct DungeonBuilder {
    chunks: Vec<Chunk>,
    rng: ThreadRng,
    dimension_options: Option<DimensionOptions>,
    hallway_options: Option<HallwayOptions>,
}

impl DungeonBuilder  {
    pub fn new() -> DungeonBuilder  {
        DungeonBuilder { 
            chunks: Vec::new(), 
            rng: thread_rng(), 
            dimension_options: None, 
            hallway_options: None 
        }
    }

    pub fn in_area(&mut self, lower_left: Point, upper_right: Point) -> &mut DungeonBuilder {
        let chunk = Chunk::new(lower_left, upper_right);
        self.chunks.push(chunk);
        self        
    }

    pub fn with_dimension_options(&mut self, dimension_options: DimensionOptions) -> &mut DungeonBuilder {
        self.dimension_options = Some(dimension_options);
        self
    }

    pub fn with_hallway_options(&mut self, hallway_options: HallwayOptions) -> &mut DungeonBuilder {
        self.hallway_options = Some(hallway_options);
        self
    }

    pub fn build(&mut self) -> Dungeon {
        let mut total_hallway_percent = 0f32;
        let total_area = self.chunks[0].area();
        let mut dungeon = Dungeon::new();
        match self.dimension_options {
            Some(ref dimension_options) => {
                while self.chunks.len() > 0 {
                    let mut chunk = self.chunks.pop().expect("chunk not found in queue");
                    let (new_chunk_option, chunksplit) = chunk.split(&dimension_options, &mut self.rng);
                    match new_chunk_option {
                        Some(new_chunk) => {
                            match self.hallway_options {
                                Some(ref hallway_options) => {
                                    let can_strip_hallway = match chunksplit {
                                        ChunkSplit::Horizontal => {
                                            chunk.height() > hallway_options.min_hallway_length && chunk.width() > hallway_options.min_hallway_width
                                        },
                                        ChunkSplit::Vertical => { 
                                            chunk.width() > hallway_options.min_hallway_length && chunk.height() > hallway_options.min_hallway_width
                                        }
                                    };
                                    if can_strip_hallway && total_hallway_percent < hallway_options.hallway_percent {
                                        let hallway_width = self.rng.gen_range(hallway_options.min_hallway_width, hallway_options.max_hallway_width);
                                        let hallway = chunk.strip_hallway(chunksplit, hallway_width);
                                        let hallway_percent = (hallway.area() / total_area) * 100f32;
                                        total_hallway_percent = total_hallway_percent + hallway_percent;
                                        dungeon.add_hallway(hallway);
                                    }
                                },
                                _ => {}
                            }

                            self.chunks.push(chunk);
                            self.chunks.push(new_chunk);
                        },
                        None => {
                            dungeon.add_room(Room::new(chunk));
                        }
                    }
                }
            },
            None => {
                panic!("dimension options must be set with 'with_dimension_options'")
            }
        }
        dungeon
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;
    use dimensionoptions::DimensionOptions;

    #[test]
    fn test_build() {
        DungeonBuilder::new(10f32)
            .in_area(Point::new(0f32,0f32), Point::new(1000f32,1000f32))
            .with_dimension_options(DimensionOptions::new(5f32,5f32,5f32))
            .build();
    }
}
