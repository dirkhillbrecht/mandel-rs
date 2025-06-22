use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc::UnboundedReceiver;

use crate::storage::event::stage_event_batcher::StageEvent;
use crate::storage::image_comp_properties::{ImageCompProperties, StageState};
use crate::storage::computation::comp_storage::CompStorage;
use super::viz_stage::VizStage;

/// Storage for image data computation
/// Contains parameters of computation and the actual computation stage
pub struct VizStorage {

    pub properties: ImageCompProperties,
    pub stage: VizStage,
    /// State of the comp storage which is seen by the viz storage, can change through events.
    #[allow(dead_code)]
    pub seen_state: StageState,

    comp_storage: Arc<CompStorage>,
    event_receiver: Option<UnboundedReceiver<StageEvent>>,

}

impl VizStorage {

    /// Create a new comp storage instance, initialize the stage internally.
    pub fn new(arc_of_comp_storage: Arc<CompStorage>) -> VizStorage {
        // First step: Couple to events so that no changes are missed
        let event_receiver_result=arc_of_comp_storage.as_ref().get_event_receiver(1000, Duration::from_millis(50)).ok();
        // Second step: Copy stage
        let seen_state=arc_of_comp_storage.as_ref().stage.get_state();
        // Third step: Initialize the visualization stage - which reads the contents of the computation stage
        let stage=VizStage::new(&arc_of_comp_storage.as_ref().stage);
        // And here we go!
        VizStorage {
            properties: arc_of_comp_storage.as_ref().properties,
            stage,
            seen_state,
            comp_storage: arc_of_comp_storage,
            event_receiver: event_receiver_result,
        }
    }

    /// Process events received from the comp storage
    pub fn process_events(&mut self) {
        if let Some(receiver) = &mut self.event_receiver {
            // Read and handle events as long as there are some
            while let Ok(event) = receiver.try_recv() {
                match event {
                    StageEvent::ContentChange(change) => {
                        self.stage.set_from_change(change);
                    }
                    StageEvent::ContentMultiChange(changes) => {
                        changes.changes().iter().for_each(|change| self.stage.set_from_change(*change));
                    }
                    StageEvent::StateChange(thestate) => {
                        if thestate==StageState::Stalled || thestate==StageState::Completed {
                            let _ = self.comp_storage.drop_event_receiver();
                        }
                    }
                }
            }
        }
    }

}

// end of file
