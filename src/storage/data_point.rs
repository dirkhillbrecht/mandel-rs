// Data point for the computed data of a fractal image

#[derive(Clone,Debug)]
pub struct DataPoint {

    iteration_count: u32,
    final_x: f64,
    final_y: f64,

}

impl DataPoint {
    
    // Constructor
    pub fn new(iteration_count: u32, final_x: f64, final_y: f64) -> DataPoint {
        DataPoint { iteration_count, final_x, final_y }
    }

    pub fn iteration_count(&self) -> u32 { self.iteration_count }
    pub fn final_x(&self) -> f64 { self.final_x }
    pub fn final_y(&self) -> f64 { self.final_y }

}

// end of file
