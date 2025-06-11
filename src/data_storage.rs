// Definition of a complete data set for creating an image

use crate::data_plane::DataPlane;

#[derive(Debug, Clone)]
pub struct ComputeProgress {
    pub completed_pixels: usize,
    pub total_pixels: usize,
    pub completion_ratio: f64,
}

impl ComputeProgress {
    pub fn new(completed_pixels: usize, total_pixels: usize) -> ComputeProgress {
        ComputeProgress { completed_pixels, total_pixels,
            completion_ratio: (completed_pixels as f64 / total_pixels as f64).max(0.0).min(1.0) }
    }
}

#[derive (Debug, Clone)]
pub struct DataStorage {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iteration: u32,
    plane: DataPlane,
}

impl DataStorage {
    pub fn new(x_min:f64,x_max:f64,y_min:f64,y_max:f64,width:u32,height:u32,max_iteration:u32) -> DataStorage {
        DataStorage{x_min,x_max,y_min,y_max,max_iteration,plane:DataPlane::new(width as usize,height as usize,x_min,x_max,y_min,y_max)}
    }
    pub fn x_min(&self) -> f64 { self.x_min }
    pub fn x_max(&self) -> f64 { self.x_max }
    pub fn y_min(&self) -> f64 { self.y_min }
    pub fn y_max(&self) -> f64 { self.y_max }
    pub fn max_iteration(&self) -> u32 { self.max_iteration }

    pub fn plane(&self) -> &DataPlane { &self.plane }
    pub fn plane_mut(&mut self) -> &mut DataPlane { &mut self.plane }
}

// end of file
