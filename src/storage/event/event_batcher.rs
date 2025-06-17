use std::time::Duration;

pub struct EventBatcher {

    max_capacity: usize,
    max_interval: Duration,

}

impl EventBatcher {

    pub fn new(max_capacity: usize, max_interval: Duration) -> Self {
        EventBatcher { max_capacity, max_interval }
    }

}

// end of file
