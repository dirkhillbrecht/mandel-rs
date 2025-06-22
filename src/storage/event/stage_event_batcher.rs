use std::{pin::Pin, time::{Duration, Instant}};

use tokio::sync::mpsc;

use crate::storage::{event::data_point_change_event::{DataPointChange, DataPointMultiChange}, image_comp_properties::StageState};

/// Events emit by a stage (currently the CompStage only) to signal changes in its content or state
pub enum StageEvent {
    /// Signals a change of the stage state in general
    StateChange(StageState),
    /// Signals the change of one single data point in the stage
    ContentChange(DataPointChange),
    /// Signals the change of multiple data points in the stage
    ContentMultiChange(DataPointMultiChange),
}

/// Internal buffer for the event batcher
struct DataPointChangeBuffer {
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
    pub fn push_data_point_change(&mut self, change: DataPointChange) {
        self.changes.push(change);
    }

    pub fn into_multi_change(self) -> DataPointMultiChange {
        DataPointMultiChange::new(self.changes)
    }

}



/// Organiser for stage events, collects content changes, emits content multi changes
///
/// The batcher gets single data point change events and collects their content.
/// On certain occasions, it creates a multi change event out of the collected changes and emits this as a new event
/// The occasions are
/// * A certain number of events has accumulated
/// * A certain time has passed since the last event has been emit
/// * Stage state changes into Stalled or Complete
pub struct StageEventBatcher {

    max_capacity: usize,
    max_interval: Duration,

}

impl StageEventBatcher {

    pub fn new(max_capacity: usize, max_interval: Duration) -> Self {
        StageEventBatcher { max_capacity, max_interval }
    }

    fn flush_buffer_and_clear_timer(
        buffer: &mut Option<DataPointChangeBuffer>,
        timer: &mut Option<Pin<Box<tokio::time::Sleep>>>,
        output: &mpsc::UnboundedSender<StageEvent>,
    ) {
        if let Some(buf) = buffer.take() {  // take "empties" the original Option
            let multi_change = buf.into_multi_change();
            let _ = output.send(StageEvent::ContentMultiChange(multi_change));
        }
        *timer = None;
    }

    fn push_data_point_change_to_buffer(
        change: DataPointChange,
        current_buffer: &mut Option<DataPointChangeBuffer>,
        timer: &mut Option<Pin<Box<tokio::time::Sleep>>>,
        max_capacity: usize,
        max_interval: Duration,
        output: &mpsc::UnboundedSender<StageEvent>,
    ) {
        if current_buffer.is_none() {
            *current_buffer = Some(DataPointChangeBuffer::new(max_capacity));
            *timer = Some(Box::pin(tokio::time::sleep(max_interval)));
        }
        current_buffer.as_mut().unwrap().push_data_point_change(change);
        if current_buffer.as_ref().unwrap().is_capacity_exceeded() {
            // send the buffer
            Self::flush_buffer_and_clear_timer(current_buffer,timer,output);
        }
    }

    pub async fn run(
        self,
        mut input: mpsc::UnboundedReceiver<StageEvent>,
        output: mpsc::UnboundedSender<StageEvent>,
    ) {
        let mut current_buffer: Option<DataPointChangeBuffer> = None;
        let mut timer: Option<Pin<Box<tokio::time::Sleep>>> = None;

        loop {
            tokio::select! {
                // Branch 1: Some stuff from the input channel receiver
                result = input.recv() => {
                    match result {

                        // Branch 1.1: Empty option from input channel: Event handling is over
                        None => {
                            Self::flush_buffer_and_clear_timer(&mut current_buffer, &mut timer, &output);
                            break; // This drops the output sender and therefore closes the output channel
                        }

                        // Branch 1.2: New event
                        Some(event) => {
                            match event {
                                StageEvent::ContentChange(change) => {
                                    Self::push_data_point_change_to_buffer(
                                        change,
                                        &mut current_buffer,
                                        &mut timer,
                                        self.max_capacity,
                                        self.max_interval,
                                        &output);
                                }
                                StageEvent::StateChange(new_state) => {
                                    let _ = output.send(StageEvent::StateChange(new_state));
                                    if new_state==StageState::Stalled || new_state==StageState::Completed {
                                        Self::flush_buffer_and_clear_timer(&mut current_buffer, &mut timer, &output);
                                        break;
                                    }
                                }
                                StageEvent::ContentMultiChange(multi_change) => {
                                    for change in multi_change.changes() {
                                        Self::push_data_point_change_to_buffer(
                                            *change,
                                            &mut current_buffer,
                                            &mut timer,
                                            self.max_capacity,
                                            self.max_interval,
                                            &output);
                                    }
                                }
                            }
                        }
                    }
                }

                // Branch 2: Timer fired (only if it actually exists)
                //() = timer.as_mut().unwrap(), if timer.is_some() => {
                () = async {
                    if let Some(t) = timer.as_mut() {
                        t.await
                    } else {
                        std::future::pending().await
                    }
                } => {
                    Self::flush_buffer_and_clear_timer(&mut current_buffer, &mut timer, &output);
                }

            }
        }
    }

}

// end of file
