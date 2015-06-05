use room::Room;
use chunk::{Chunk, ChunkSplit};
use point::Point;
use dungeon::Dungeon;
use rand::{Rng, ThreadRng, thread_rng};
use dimensionoptions::DimensionOptions;
use hallway::Hallway;
use hallwayoptions::HallwayOptions;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct DungeonBuilder {
    chunks: BinaryHeap<Chunk>,
    rng: ThreadRng,
    dimension_options: Option<DimensionOptions>,
    hallway_options: Option<HallwayOptions>,
    total_area: f32,
}

impl DungeonBuilder  {
    pub fn new() -> DungeonBuilder  {
        DungeonBuilder { 
            chunks: BinaryHeap::new(), 
            rng: thread_rng(), 
            dimension_options: None, 
            hallway_options: None,
            total_area: 0f32,
        }
    }

    pub fn in_area(&mut self, lower_left: Point, upper_right: Point) -> &mut DungeonBuilder {
        let mut chunk_split = ChunkSplit::Horizontal;
        if self.rng.gen_weighted_bool(2) {
            chunk_split = ChunkSplit::Vertical;
        }
        let chunk = Chunk::new(lower_left, upper_right, chunk_split);
        self.total_area = chunk.area();
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
        let mut dungeon = Dungeon::new();
        let mut hallway_points : Vec<Point> = Vec::new();
        match self.dimension_options {
            Some(ref dimension_options) => {
                while self.chunks.len() > 0 {
                    let mut chunk = self.chunks.pop().expect("chunk not found in queue");
                    let new_chunk_option = chunk.split(&dimension_options, &mut self.rng);
                    match new_chunk_option {
                        Some(new_chunk) => {
                            match self.hallway_options {
                                Some(ref hallway_options) => {
                                    let can_strip_hallway = match new_chunk.chunk_split() {
                                        ChunkSplit::Vertical => {
                                            chunk.height() > hallway_options.min_hallway_length && chunk.width() > hallway_options.min_hallway_width
                                        },
                                        ChunkSplit::Horizontal => { 
                                            chunk.width() > hallway_options.min_hallway_length && chunk.height() > hallway_options.min_hallway_width
                                        }
                                    };
                                    if can_strip_hallway && total_hallway_percent < hallway_options.hallway_percent {
                                        let hallway_width = self.rng.gen_range(hallway_options.min_hallway_width, hallway_options.max_hallway_width);
                                        let hallway_chunk = chunk.strip_hallway(new_chunk.chunk_split(), hallway_width);
                                        let hallway_percent = (hallway_chunk.area() / self.total_area) * 100f32;
                                        total_hallway_percent = total_hallway_percent + hallway_percent;
                                        hallway_points.push(hallway_chunk.lower_left().clone());
                                        hallway_points.push(Point::new(hallway_chunk.lower_left().x(), hallway_chunk.upper_right().y()));
                                        hallway_points.push(hallway_chunk.upper_right().clone());
                                        hallway_points.push(Point::new(hallway_chunk.upper_right().x(), hallway_chunk.lower_left().y()));
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
        self.merge_hallways(&mut hallway_points, &mut dungeon);
        dungeon
    }

    fn merge_hallways(&mut self, hallway_points: &mut Vec<Point>, dungeon: &mut Dungeon) {
        hallway_points.sort_by(|a, b| a.compare_x_y(&b));
        hallway_points.dedup();
        let mut horizontal_edges : HashMap<u64, Point> = HashMap::with_capacity(hallway_points.len());
        let mut vertical_edges : HashMap<u64, Point> = HashMap::with_capacity(hallway_points.len());
        let mut idx = 0usize;
        while idx < hallway_points.len() - 1 {
            vertical_edges.insert(hallway_points[idx].hash(), hallway_points[idx+1].clone());
            vertical_edges.insert(hallway_points[idx+1].hash(), hallway_points[idx].clone());
            idx = idx + 2;
        }
        hallway_points.sort_by(|a, b| a.compare_y_x(&b));
        idx = 0usize;
        while idx < hallway_points.len() - 1 {
            horizontal_edges.insert(hallway_points[idx].hash(), hallway_points[idx+1].clone());
            horizontal_edges.insert(hallway_points[idx+1].hash(), hallway_points[idx].clone());
            idx = idx + 2;
        }
        let mut length = vertical_edges.len();
        while length > 1 {
            let mut points : Vec<Point> = Vec::new();
            points.push(vertical_edges.values().nth(0).expect("No points in vertical_edges").clone()); 
            let mut current_hash = points[0].hash();
            let first_hash = current_hash;
            let point = horizontal_edges.remove(&current_hash).expect("Start Horizontal Point not found");
            let mut next_hash = point.hash();
            horizontal_edges.remove(&next_hash);
            points.push(point);
            current_hash = next_hash;
            next_hash = vertical_edges.get(&current_hash).expect("Next Vertical Point not found").hash();
            while first_hash != next_hash {
                let point = vertical_edges.remove(&current_hash).unwrap();
                vertical_edges.remove(&point.hash());
                points.push(point);
                current_hash = next_hash;
                let point = horizontal_edges.remove(&current_hash).expect("Horizontal Point not found");
                next_hash = point.hash();
                horizontal_edges.remove(&next_hash);
                points.push(point);
                current_hash = next_hash;
                next_hash = vertical_edges.get(&current_hash).expect("Vertical Point not found").hash();
            }
            let point = vertical_edges.remove(&current_hash).unwrap();
            vertical_edges.remove(&point.hash());
            dungeon.add_hallway(Hallway::new(points));
            length = vertical_edges.len();
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;
    use dimensionoptions::DimensionOptions;

    #[test]
    fn test_build() {
        DungeonBuilder::new()
            .in_area(Point::new(0f32,0f32), Point::new(1000f32,1000f32))
            .with_dimension_options(DimensionOptions::new(5f32,5f32,5f32))
            .build();
    }
}
