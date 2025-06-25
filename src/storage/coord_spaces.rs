/// Phantom type indicating that some coordinates are in the mathmatical realm
pub struct MathSpace;

/// Phantom type indicating that some coordinates are in the stages
#[allow(dead_code)]
pub struct StageSpace;

/// Phantom type indicating that some coordinates are referring to screen pixels
#[allow(dead_code)]
pub struct PixelSpace;
