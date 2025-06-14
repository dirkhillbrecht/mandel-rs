use crate::storage::image_comp_properties::ImageCompProperties;
use super::comp_stage::CompStage;

/// Storage for image data computation
/// Contains parameters of computation and the actual computation stage
pub struct CompStorage {

    pub properties: ImageCompProperties,
    pub stage: CompStage,

}

impl CompStorage {

    /// Create a new comp storage instance, initialize the stage internally.
    pub fn new(properties: ImageCompProperties) -> CompStorage {
        let stage=CompStage::new(properties.stage_properties.width,properties.stage_properties.height);
        CompStorage {
            properties,
            stage,
        }
    }

}

// end of file
