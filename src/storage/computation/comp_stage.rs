use std::sync::RwLock;

use tokio::sync::mpsc::UnboundedSender;

use crate::storage::{data_point::DataPoint, event::{data_point_change_event::DataPointChange, stage_event_batcher::StageEvent}, image_comp_properties::StageState};

/// Actual stage with data for the image pixels to be computed.
/// Designed for massively parallel access
/// Note that a CompStage has no idea of the mathematical algorithms which produce pictures.
/// It only knows about the mathematical data stored for each pixel.
pub struct CompStage {

    /// Width of the stored data in pixels
    width: usize,
    /// Height of the stored data in pixels
    height: usize,
    /// Internal storage of all the data, internally the 2-dimensional storage is serialized into a 1-dimensional vector.
    data: Vec<RwLock<Option<DataPoint>>>,
    /// Current state of the stage (set by whoever performs changes on the stage)
    state: RwLock<StageState>,
    /// channel end point for data points to be buffered to create events.
    change_sender: std::sync::Mutex<Option<UnboundedSender<StageEvent>>>,

}

impl CompStage {

    /// Create a new computation stage, gets the dimensions, creates the actual storage internally.
    pub fn new(width:u32,height:u32) -> Self {
        let mut data=Vec::with_capacity((width*height) as usize);
        for _ in 0..(width*height) {
            data.push(RwLock::new(None));
        }
        CompStage {
            width: width as usize,
            height: height as usize,
            data,
            state: RwLock::new(StageState::Initialized),
            change_sender: std::sync::Mutex::new(None),
//            event_buffer_capacity,
//            event_buffer: RwLock::new(None),
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    /// Internal helper method: Compute an index
    fn index(&self,x:u32,y:u32) -> usize {
        if x as usize>=self.width || y as usize>=self.height {
            panic!("Coordinates ({},{}) out of bounds for computation stage of size {}*{}", x, y, self.width, self.height);
        }
        y as usize*self.width+x as usize
    }

    /// Return whether the point at the given location has already been computed
    pub fn is_computed(&self, x: u32, y: u32) -> bool {
        let idx=self.index(x,y);
        self.data[idx].read().unwrap().is_some()
    }

    fn internal_get(&self,idx:usize) -> Option<DataPoint> {
        let guard=self.data[idx].read().unwrap();
        *guard
    }

    // Get the data of the point at the given position, returns independent data, uses lock internally
    pub fn get(&self, x: u32, y: u32) -> Option<DataPoint> {
        self.internal_get(self.index(x,y))
    }

    pub fn set_change_sender(&self, sender: Option<UnboundedSender<StageEvent>>) {
        *self.change_sender.lock().unwrap()=sender;
    }

    /// Return the current stage of the stage as reported by the stage data writer
    pub fn get_state(&self) -> StageState {
        let guard=self.state.read().unwrap();
        *guard
    }

    // Set data of a point, handles locking internally.
    pub fn set(&self, x: u32, y: u32, data_point: DataPoint) {
        {
            let mut data_write_guard=self.data[self.index(x,y)].write().unwrap();
            *data_write_guard = Option::Some(data_point);
        }
        if let Some(sender)=&*self.change_sender.lock().unwrap() {
            let _ = sender.send(StageEvent::ContentChange(DataPointChange::new(x,y,&data_point)));
        }
    }

    /// Set the state of the stage
    pub fn set_state(&self, new_state: StageState) {
        let mut send_new_state=false;
        {
            let mut state_write_guard=self.state.write().unwrap();
            if *state_write_guard != new_state {
                *state_write_guard=new_state;
                send_new_state=true;
            }
        }
        if send_new_state {
            if let Some(sender)=&*self.change_sender.lock().unwrap() {
                let _ = sender.send(StageEvent::StateChange(new_state));
            }
        }
    }

    /// Return the full data as a vector without the read-write locks
    pub fn get_full_data(&self) -> Vec<Option<DataPoint>> {
        // This functional approach is slightly less performant as it might reallocate the target Vec memory
        //(0..self.data.len()).map(|i| self.internal_get(i)).collect()
        let mut retval=Vec::with_capacity(self.width*self.height);
        for i in 0..self.data.len() {
            retval.push(self.internal_get(i));
        }
        retval
    }

}

// end of file
