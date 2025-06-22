use crate::storage::data_point::DataPoint;

/// Description of the change of a single data point
#[derive(Debug, Clone, Copy)]
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

// Description of the change of multiple (independent) data points
#[derive(Debug, Clone)]
pub struct DataPointMultiChange {
    changes: Vec<DataPointChange>,
}

impl DataPointMultiChange {
    pub fn new(changes: Vec<DataPointChange>) -> Self {
        DataPointMultiChange { changes }
    }
    pub fn changes(&self) -> &[DataPointChange] { &self.changes }
    #[allow(dead_code)]  // kind of public API, may be useful in the future
    pub fn len(&self) -> usize { self.changes.len() }
}

// end of file
