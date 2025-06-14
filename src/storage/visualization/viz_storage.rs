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
    pub fn new(comp_storage: CompStorage) -> VizStorage {
        let stage=VizStage::new(comp_storage.stage);
        VizStorage {
            properties: comp_storage.properties,
            stage,
        }
    }

}

// end of file
