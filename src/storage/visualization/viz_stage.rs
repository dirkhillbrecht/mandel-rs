// Two-dimensional data stage for the visualization of a graphics

use crate::storage::computation::comp_stage::CompStage;
use crate::storage::data_point::DataPoint;
use crate::storage::event::data_point_change_event::DataPointChange;

#[derive(Debug, Clone)]
pub struct VizStage {
    width: usize,
    height: usize,
    data: Vec<Option<DataPoint>>,
}

impl VizStage {
    pub fn new(comp_stage: &CompStage) -> Self {
        VizStage {
            width: comp_stage.width(),
            height: comp_stage.height(),
            data: comp_stage.get_full_data(),
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    fn index(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!(
                "Coordinates ({},{}) out of bounds for visualization stage of size {}*{}",
                x, y, self.width, self.height
            );
        }
        y * self.width + x
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&DataPoint> {
        self.data[self.index(x, y)].as_ref()
    }
    pub fn set(&mut self, x: usize, y: usize, data_point: DataPoint) {
        let index = self.index(x, y);
        self.data[index] = Some(data_point);
    }
    pub fn set_from_change(&mut self, data_point_change: DataPointChange) {
        self.set(
            data_point_change.x as usize,
            data_point_change.y as usize,
            data_point_change.data,
        );
    }
}

// end of file
