use std::sync::RwLock;

use euclid::{Point2D, Size2D, Vector2D};
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

    /// Generate a new, independent comp stage which contains a shifted part of the current stage
    /// If the offset is (0,0), an unshifted independent clone of this stage is returned,
    /// try to prevent this from happening
    /// If the offset is so large that the new stage is completely outside this one,
    /// an empty new stage of the same size is returned.
    pub fn shifted_clone(&self, offset: Vector2D<i32, StageSpace>) -> Self {
        if offset.x.abs() as usize >= self.size.width || offset.y.abs() as usize >= self.size.height
        {
            Self::new(Size2D::new(self.size.width as u32, self.size.height as u32))
        } else {
            let ox = offset.x;
            let oy = offset.y;
            let empty_line_start = (ox.max(0) as usize).min(self.size.width);
            let empty_line_end = ((-ox).max(0) as usize).min(self.size.width);
            let empty_start_lines = (oy.max(0) as usize).min(self.size.height);
            let empty_end_lines = ((-oy).max(0) as usize).min(self.size.height);
            let line_width = self.size.width - (empty_line_start.max(empty_line_end));
            let first_line = empty_end_lines;
            let last_line = self.size.height - empty_start_lines;
            let mut data = Vec::with_capacity(self.size.area());
            for _ in 0..empty_start_lines {
                for _ in 0..self.size.width {
                    data.push(RwLock::new(None));
                }
            }
            for line in first_line..last_line {
                for _ in 0..empty_line_start {
                    data.push(RwLock::new(None));
                }
                let first_idx = line * self.size.width + empty_line_end;
                let last_idx = first_idx + line_width;
                for idx in first_idx..last_idx {
                    data.push(RwLock::new(self.internal_get(idx)));
                }
                for _ in 0..empty_line_end {
                    data.push(RwLock::new(None));
                }
            }
            for _ in 0..empty_end_lines {
                for _ in 0..self.size.width {
                    data.push(RwLock::new(None));
                }
            }
            CompStage {
                size: self.size,
                data,
                state: RwLock::new(StageState::Stalled),
                change_sender: std::sync::Mutex::new(None),
            }
        }
    }

    /// Return a stage containing some zoomed information from this stage
    /// The new stage might contain partial data or even nothing at all.
    /// It should approximate the content as good as possible
    pub fn zoomed_clone(&self, _origin: Point2D<i32, StageSpace>, _factor: f32) -> Self {
        // This is a dummy implementation always returning an empty new stage
        Self::new(Size2D::new(self.size.width as u32, self.size.height as u32))
    }
}

// end of file
