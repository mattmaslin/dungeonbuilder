use point::Point;
use chunk::Chunk;
use hallway::Hallway;
use std::collections::HashMap;


pub struct HallwayBuilder {
    points: Vec<Point>,
    total_area: f32
}

impl HallwayBuilder  {
    pub fn new() -> HallwayBuilder  {
        HallwayBuilder { 
            points: Vec::new(), 
            total_area: 0f32
        }
    }

    pub fn total_area(&self) -> f32 {
        self.total_area
    }

    pub fn add_chunk(&mut self, chunk: Chunk) -> &mut HallwayBuilder {
        self.total_area = self.total_area + chunk.area();
        self.points.push(chunk.lower_left().clone());
        self.points.push(Point::new(chunk.lower_left().x(), chunk.upper_right().y()));
        self.points.push(chunk.upper_right().clone());
        self.points.push(Point::new(chunk.upper_right().x(), chunk.lower_left().y()));
        self
    }

    pub fn merge_hallways(&mut self) -> Vec<Hallway> {
        let mut hallways : Vec<Hallway> = Vec::new();
        self.points.sort_by(|a, b| a.compare_x_y(&b));
        self.points.dedup();
        let mut horizontal_edges : HashMap<u64, Point> = HashMap::with_capacity(self.points.len());
        let mut vertical_edges : HashMap<u64, Point> = HashMap::with_capacity(self.points.len());
        let mut idx = 0usize;
        while idx < self.points.len() - 1 {
            vertical_edges.insert(self.points[idx].hash(), self.points[idx+1].clone());
            vertical_edges.insert(self.points[idx+1].hash(), self.points[idx].clone());
            idx = idx + 2;
        }
        self.points.sort_by(|a, b| a.compare_y_x(&b));
        idx = 0usize;
        while idx < self.points.len() - 1 {
            horizontal_edges.insert(self.points[idx].hash(), self.points[idx+1].clone());
            horizontal_edges.insert(self.points[idx+1].hash(), self.points[idx].clone());
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
            hallways.push(Hallway::new(points));
            length = vertical_edges.len();
        }
        hallways
    }
}
