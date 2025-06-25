/// Application frame for the Iced-based mandel-rs GUI
use crate::comp::mandelbrot_engine::MandelbrotEngine;
use crate::comp::math_data::{MathData, MathPresets};
use crate::storage::visualization::viz_storage::VizStorage;

/// Mathematical state of the app
pub struct MathState {
    pub width: String,
    pub height: String,
    pub left: String,
    pub right: String,
    pub top: String,
    pub bottom: String,
    pub max_iteration: String,
}

impl MathState {
    pub fn new(
        width: String,
        height: String,
        left: String,
        right: String,
        top: String,
        bottom: String,
        max_iteration: String,
    ) -> Self {
        MathState {
            width,
            height,
            left,
            right,
            top,
            bottom,
            max_iteration,
        }
    }
    pub fn from_math_data(width: String, height: String, data: MathData) -> Self {
        Self::new(
            width,
            height,
            data.coordinates().min_x().to_string(),
            data.coordinates().max_x().to_string(),
            data.coordinates().max_y().to_string(),
            data.coordinates().min_y().to_string(),
            data.max_iteration().to_string(),
        )
    }
}

impl Default for MathState {
    fn default() -> Self {
        Self::from_math_data(
            "800".to_string(),
            "600".to_string(),
            MathPresets::preset(&MathPresets::MandelbrotFull),
        )
    }
}

/// Visual state of the app
pub struct VizState {
    pub auto_start_computation: bool,
    pub sidebar_visible: bool,
}

impl VizState {
    pub fn new(auto_start_computation: bool, sidebar_visible: bool) -> Self {
        VizState {
            auto_start_computation,
            sidebar_visible,
        }
    }
}

impl Default for VizState {
    fn default() -> Self {
        Self::new(true, true)
    }
}

/// Runtime state of the app
pub struct RuntimeState {
    pub computing: bool,
}

impl RuntimeState {
    pub fn new(computing: bool) -> Self {
        RuntimeState { computing }
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new(false)
    }
}

pub struct AppState {
    pub storage: Option<VizStorage>,
    pub engine: Option<MandelbrotEngine>,
    pub math: MathState,
    pub viz: VizState,
    pub runtime: RuntimeState,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            storage: None,
            engine: None,
            math: MathState::default(),
            viz: VizState::default(),
            runtime: RuntimeState::default(),
        }
    }
}

pub fn launch() -> iced::Result {
    iced::application(
        "Mandelbrot Fractal Visualizer",
        super::update::update,
        super::view::view,
    )
    .subscription(super::subscription::subscription)
    .run()
}

// end of file
