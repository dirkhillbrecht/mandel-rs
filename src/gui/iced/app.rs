use std::sync::Arc;
use std::time::Instant;

use euclid::Rect;
use iced::widget::canvas::Cache;

/// Application frame for the Iced-based mandel-rs GUI
use crate::comp::mandelbrot_engine::MandelbrotEngine;
use crate::comp::math_data::{MathData, MathPreset};
use crate::storage::computation::comp_storage::CompStorage;
use crate::storage::coord_spaces::MathSpace;
use crate::storage::visualization::coloring::presets::{GradientColorPreset, IterationAssignment};
use crate::storage::visualization::viz_storage::VizStorage;

/// Mathematical state of the app
pub struct MathState {
    pub width: String,
    pub height: String,
    pub area: Rect<f64, MathSpace>,
    pub max_iteration: String,
}

impl MathState {
    pub fn new(
        width: String,
        height: String,
        area: Rect<f64, MathSpace>,
        max_iteration: String,
    ) -> Self {
        MathState {
            width,
            height,
            area,
            max_iteration,
        }
    }
    pub fn from_math_data(width: String, height: String, data: MathData) -> Self {
        Self::new(
            width,
            height,
            data.coordinates(),
            data.max_iteration().to_string(),
        )
    }
}

impl Default for MathState {
    fn default() -> Self {
        Self::from_math_data(
            "800".to_string(),
            "600".to_string(),
            MathPreset::preset(&MathPreset::MandelbrotFull),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageRenderScheme {
    Cropped,
    FilledWithBackground,
    Filled,
    ShrunkWithBackground,
    Shrunk,
    CenteredWithBackground,
    Centered,
}

impl ImageRenderScheme {
    #[allow(dead_code)]
    pub fn all() -> &'static [Self] {
        &[
            Self::Cropped,
            Self::FilledWithBackground,
            Self::Filled,
            Self::ShrunkWithBackground,
            Self::Shrunk,
            Self::CenteredWithBackground,
            Self::Centered,
        ]
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cropped => "Scaled to crop",
            Self::FilledWithBackground => "Scaled to fill",
            Self::Filled => "Scaled to fill (blank)",
            Self::ShrunkWithBackground => "No upscale",
            Self::Shrunk => "No upscale (blank)",
            Self::CenteredWithBackground => "Unscaled centered",
            Self::Centered => "Unscaled centered (blank)",
        }
    }
    pub fn needs_background_cropped(&self) -> bool {
        matches!(
            *self,
            ImageRenderScheme::FilledWithBackground
                | ImageRenderScheme::ShrunkWithBackground
                | ImageRenderScheme::CenteredWithBackground
        )
    }
}

impl std::fmt::Display for ImageRenderScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Visual state of the app
pub struct VizState {
    pub math_preset: MathPreset,
    pub auto_start_computation: bool,
    pub sidebar_visible: bool,
    pub gradient_color_preset: GradientColorPreset,
    pub iteration_assignment: IterationAssignment,
    pub render_scheme: ImageRenderScheme,
}

impl VizState {
    pub fn new(
        math_preset: MathPreset,
        auto_start_computation: bool,
        sidebar_visible: bool,
        gradient_color_preset: GradientColorPreset,
        iteration_assignment: IterationAssignment,
        render_scheme: ImageRenderScheme,
    ) -> Self {
        VizState {
            math_preset,
            auto_start_computation,
            sidebar_visible,
            gradient_color_preset,
            iteration_assignment,
            render_scheme,
        }
    }
}

impl Default for VizState {
    fn default() -> Self {
        Self::new(
            MathPreset::MandelbrotFull,
            true,
            true,
            GradientColorPreset::Sunrise,
            IterationAssignment::Linear,
            ImageRenderScheme::FilledWithBackground,
        )
    }
}

/// Runtime state of the app
pub struct RuntimeState {
    pub computing: bool,
    pub canvas_cache: Cache,
    pub zoom_timer: Option<Instant>,
}

impl RuntimeState {
    pub fn new(computing: bool) -> Self {
        RuntimeState {
            computing,
            canvas_cache: Cache::new(),
            zoom_timer: None,
        }
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new(false)
    }
}

pub struct AppState {
    pub storage: Option<VizStorage>,
    pub comp_storage: Option<Arc<CompStorage>>,
    pub engine: Option<MandelbrotEngine>,
    pub math: MathState,
    pub viz: VizState,
    pub runtime: RuntimeState,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            storage: None,
            comp_storage: None,
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
