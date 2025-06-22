// Two-dimensional data stage for the visualization of a graphics

use crate::storage::computation::comp_stage::CompStage;
use crate::storage::data_point::DataPoint;
use crate::storage::event::data_point_change_event::DataPointChange;

#[derive(Debug, Clone)]
pub struct VizStage {
    width: usize,
    height: usize,
    data: Vec<Option<DataPoint>>,
    set_count: usize,
}

impl VizStage {
    pub fn new(comp_stage: &CompStage) -> Self {
        let data = comp_stage.get_full_data();
        let set_count = data.iter().filter(|p| p.is_some()).count();
        VizStage {
            width: comp_stage.width(),
            height: comp_stage.height(),
            data,
            set_count,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    #[allow(dead_code)]
    pub fn set_count(&self) -> usize {
        self.set_count
    }
    /// Return the ratio of computed data in this stage, 1 is "all is computed"
    pub fn computed_ratio(&self) -> f32 {
        (self.set_count as f32 / self.data.len() as f32)
            .min(1.0)
            .max(0.0)
    }
    /// Returns whether this stage is fully computed
    pub fn is_fully_computed(&self) -> bool {
        self.set_count >= self.data.len()
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
        if self.data[index].is_none() {
            self.set_count += 1
        }
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
