use std::sync::RwLock;

use euclid::Size2D;
use tokio::sync::mpsc::UnboundedSender;

use crate::storage::{
    coord_spaces::StageSpace,
    data_point::DataPoint,
    event::{data_point_change_event::DataPointChange, stage_event_batcher::StageEvent},
    image_comp_properties::StageState,
};

/// Actual stage with data for the image pixels to be computed.
/// Designed for massively parallel access
/// Note that a CompStage has no idea of the mathematical algorithms which produce pictures.
/// It only knows about the mathematical data stored for each pixel.
pub struct CompStage {
    /// Size of the stored data in pixels
    size: Size2D<usize, StageSpace>,
    /// Internal storage of all the data, internally the 2-dimensional storage is serialized into a 1-dimensional vector.
    data: Vec<RwLock<Option<DataPoint>>>,
    /// Current state of the stage (set by whoever performs changes on the stage)
    state: RwLock<StageState>,
    /// channel end point for data points to be buffered to create events.
    change_sender: std::sync::Mutex<Option<UnboundedSender<StageEvent>>>,
}

impl CompStage {
    /// Create a new computation stage, gets the dimensions, creates the actual storage internally.
    pub fn new(size: Size2D<u32, StageSpace>) -> Self {
        let mut data = Vec::with_capacity(size.area() as usize);
        for _ in 0..(size.area()) {
            data.push(RwLock::new(None));
        }
        CompStage {
            size: Size2D::new(size.width as usize, size.height as usize),
            data,
            state: RwLock::new(StageState::Initialized),
            change_sender: std::sync::Mutex::new(None),
            //            event_buffer_capacity,
            //            event_buffer: RwLock::new(None),
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Size2D<usize, StageSpace> {
        self.size
    }
    pub fn width(&self) -> usize {
        self.size.width
    }
    pub fn height(&self) -> usize {
        self.size.height
    }

    /// Internal helper method: Compute an index
    fn index(&self, x: u32, y: u32) -> usize {
        if x as usize >= self.size.width || y as usize >= self.size.height {
            panic!(
                "Coordinates ({},{}) out of bounds for computation stage of size {}*{}",
                x, y, self.size.width, self.size.height
            );
        }
        y as usize * self.size.width + x as usize
    }

    /// Internal base operation to get one point from the stage with handling the RwLock internally
    fn internal_get(&self, idx: usize) -> Option<DataPoint> {
        let guard = self.data[idx].read().unwrap();
        *guard
    }

    // Get the data of the point at the given position, returns independent data, uses lock internally
    pub fn get(&self, x: u32, y: u32) -> Option<DataPoint> {
        self.internal_get(self.index(x, y))
    }

    /// Return whether the point at the given location has already been computed
    pub fn is_computed(&self, x: u32, y: u32) -> bool {
        self.get(x, y).is_some()
    }

    pub fn set_change_sender(&self, sender: Option<UnboundedSender<StageEvent>>) {
        *self.change_sender.lock().unwrap() = sender;
    }

    /// Return the current stage of the stage as reported by the stage data writer
    pub fn get_state(&self) -> StageState {
        let guard = self.state.read().unwrap();
        *guard
    }

    // Set data of a point, handles locking internally.
    pub fn set(&self, x: u32, y: u32, data_point: DataPoint) {
        {
            let mut data_write_guard = self.data[self.index(x, y)].write().unwrap();
            *data_write_guard = Option::Some(data_point);
        }
        if let Some(sender) = &*self.change_sender.lock().unwrap() {
            let _ = sender.send(StageEvent::ContentChange(DataPointChange::new(
                x,
                y,
                &data_point,
            )));
        }
    }

    /// Set the state of the stage
    pub fn set_state(&self, new_state: StageState) {
        let mut send_new_state = false;
        {
            let mut state_write_guard = self.state.write().unwrap();
            if *state_write_guard != new_state {
                *state_write_guard = new_state;
                send_new_state = true;
            }
        }
        if send_new_state {
            if let Some(sender) = &*self.change_sender.lock().unwrap() {
                let _ = sender.send(StageEvent::StateChange(new_state));
            }
        }
    }

    /// Return the full data as a vector without the read-write locks
    pub fn get_full_data(&self) -> Vec<Option<DataPoint>> {
        // This functional approach is slightly less performant as it might reallocate the target Vec memory
        //(0..self.data.len()).map(|i| self.internal_get(i)).collect()
        let mut retval = Vec::with_capacity(self.size.area());
        for i in 0..self.data.len() {
            retval.push(self.internal_get(i));
        }
        retval
    }
}

// end of file
