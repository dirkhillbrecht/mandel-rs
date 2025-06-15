use std::sync::Arc;
use crate::storage::image_comp_properties::ImageCompProperties;
use crate::storage::computation::comp_storage::CompStorage;
use super::viz_stage::VizStage;

/// Storage for image data computation
/// Contains parameters of computation and the actual computation stage
pub struct VizStorage {

    pub properties: ImageCompProperties,
    pub stage: VizStage,

}

impl VizStorage {

    /// Create a new comp storage instance, initialize the stage internally.
    pub fn new(arc_of_comp_storage: Arc<CompStorage>) -> VizStorage {
        let stage=VizStage::new(&arc_of_comp_storage.as_ref().stage);
        VizStorage {
            properties: arc_of_comp_storage.as_ref().properties,
            stage,
        }
    }

}

// end of file
