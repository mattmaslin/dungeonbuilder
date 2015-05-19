pub struct DimensionOptions {
    pub min_width: f32,
    pub min_height: f32,
    pub min_area: f32,
    pub max_area: Option<f32>
}

impl DimensionOptions {
    pub fn new(min_width: f32, min_height: f32, min_area: f32) -> DimensionOptions {
        DimensionOptions { min_width: min_width, min_height: min_height, min_area: min_area, max_area: None }
    }

    pub fn new_with_max(min_width: f32, min_height: f32, min_area: f32, max_area: f32) -> DimensionOptions {
        DimensionOptions { min_width: min_width, min_height: min_height, min_area: min_area, max_area: Some(max_area) }
    }
}
