use std::time::Duration;

use tokio::sync::mpsc;

use crate::storage::{event::stage_event_batcher::{StageEvent, StageEventBatcher}, image_comp_properties::ImageCompProperties};
use super::comp_stage::CompStage;

struct EventSystem {
    task_handle: Option<tokio::task::JoinHandle<()>>,
    sender: Option<mpsc::UnboundedSender<StageEvent>>,
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem { task_handle: None, sender: None }
    }
}

/// Errors with regard to the event system
pub enum EventSystemError {
    /// Event system is already active, cannot be started twice
    AlreadyActive,
    /// Event system is not active, cannot be dropped
    NotActive,
}

/// Storage for image data computation
/// Contains parameters of computation and the actual computation stage
pub struct CompStorage {

    pub properties: ImageCompProperties,
    pub stage: CompStage,

    event_system: std::sync::Mutex<EventSystem>,
}

impl CompStorage {

    /// Create a new comp storage instance, initialize the stage internally.
    pub fn new(properties: ImageCompProperties) -> CompStorage {
        let stage=CompStage::new(properties.stage_properties.width,properties.stage_properties.height);
        CompStorage {
            properties,
            stage,
            event_system: std::sync::Mutex::new(EventSystem::new()),
        }
    }

    pub fn get_event_receiver(&self, max_capacity: usize, max_interval: Duration) -> Result<mpsc::UnboundedReceiver<StageEvent>,EventSystemError> {
        let mut event_system=self.event_system.lock().unwrap();

        // event system cannot be active twice
        if event_system.sender.is_some() {
            return Err(EventSystemError::AlreadyActive);
        }
        // Create channel for CompStage sending events to batcher
        let (comp_sender, comp_receiver) = mpsc::unbounded_channel();
        // Create channel for VizStorage receiving events from batcher
        let (viz_sender, viz_receiver) = mpsc::unbounded_channel();
        // Create the batcher
        let batcher = StageEventBatcher::new(max_capacity,max_interval);
        // Spawn the async task, this also connects both channels to the batcher
        let task_handle = tokio::task::spawn(batcher.run(comp_receiver,viz_sender));
        // Connect the comp channel to the stage
        self.stage.set_change_sender(Some(comp_sender.clone()));
        // Put everything in event system
        event_system.sender=Some(comp_sender);
        event_system.task_handle=Some(task_handle);

        // And finally return the receiver to the caller
        Ok(viz_receiver)
    }

    pub fn drop_event_receiver(&self) -> Result<(),EventSystemError> {
        let mut event_system=self.event_system.lock().unwrap();

        if event_system.sender.is_none() {
            return Err(EventSystemError::NotActive);
        }

        // Disconnect CompStage from the event system
        self.stage.set_change_sender(None);
        event_system.task_handle.take().unwrap().abort();
        event_system.sender.take().unwrap();  // Dropping the sender automatically closes the channel - according to Claude…

        Ok(())
    }

}

// end of file
