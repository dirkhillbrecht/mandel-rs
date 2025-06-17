use std::time::{Duration, Instant};

use crate::storage::data_point::DataPoint;


pub struct DataPointChange {
    pub x: u32,
    pub y: u32,
    pub data: DataPoint,
}

impl DataPointChange {
    pub fn new(x: u32, y: u32, data: &DataPoint) -> Self {
        DataPointChange { x, y, data: *data }
    }
}

pub struct DataPointChangeBuffer {
    created: Instant,
    changes: Vec<DataPointChange>,
}

impl DataPointChangeBuffer {

    /// Create a new buffer
    pub fn new(max_capacity: usize) -> Self {
        DataPointChangeBuffer {
            created: Instant::now(),
            changes: Vec::with_capacity(max_capacity),
        }
    }

    /// Check if the event should be sent due to count of changes
    pub fn is_capacity_exceeded(&self) -> bool {
        self.changes.len()>=self.changes.capacity()
    }

    /// Check if the event should be sent due to timeout
    pub fn is_timeout_reached(&self, max_duration: Duration) -> bool {
        self.created.elapsed()>=max_duration
    }

    /// Add a data point change to the event, does _not_ perform any other actions!
    pub fn add_data_point_change(&mut self, change: DataPointChange) {
        self.changes.push(change);
    }

    pub fn into_event(self, finished: bool) -> DataPointChangeEvent {
        DataPointChangeEvent::new(self.changes,finished)
    }

}

pub struct DataPointChangeEvent {
    changes: Vec<DataPointChange>,
    finished: bool,
}

impl DataPointChangeEvent {
    pub fn new(changes: Vec<DataPointChange>, finished: bool) -> Self {
        DataPointChangeEvent { changes, finished }
    }
    pub fn changes(&self) -> &[DataPointChange] { &self.changes }
    pub fn is_finished(&self) -> bool { self.finished }
    pub fn len(&self) -> usize { self.changes.len() }
}

// end of file
