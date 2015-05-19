pub struct HallwayOptions {
    pub hallway_percent: f32,
    pub min_hallway_length: f32,
    pub min_hallway_width: f32,
    pub max_hallway_width: f32,
}

impl HallwayOptions {
    pub fn new(hallway_percent: f32, min_hallway_length: f32, min_hallway_width: f32, max_hallway_width: f32) -> HallwayOptions {
        HallwayOptions { 
            hallway_percent: hallway_percent, 
            min_hallway_length: min_hallway_length, 
            min_hallway_width: min_hallway_width, 
            max_hallway_width: max_hallway_width
        }
    }
}
