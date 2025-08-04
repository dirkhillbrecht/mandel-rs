//! Application state management for the Mandelbrot fractal visualizer.
//!
//! This module defines the core state structures that represent the application's
//! mathematical configuration, visual settings, and runtime status. The state is
//! organized into three main categories:
//!
//! - **Mathematical State**: Coordinates, dimensions, and computation parameters
//! - **Visual State**: UI configuration, coloring schemes, and rendering options
//! - **Runtime State**: Dynamic state like computation status and interactive operations
//!
//! The application uses a dual-storage architecture where CompStorage handles
//! parallel computation access while VizStorage manages sequential visualization.

use std::sync::Arc;
use std::time::{Duration, Instant};

use euclid::Size2D;
use iced::Point;
use iced::widget::canvas::Cache;

use crate::comp::mandelbrot_engine::MandelbrotEngine;
use crate::comp::math_area::{MathArea, RasteredMathArea};
use crate::comp::math_data::{MathData, MathPreset};
use crate::storage::computation::comp_storage::CompStorage;
use crate::storage::coord_spaces::StageSpace;
use crate::storage::visualization::coloring::presets::{GradientColorPreset, IterationAssignment};
use crate::storage::visualization::viz_storage::VizStorage;

/// Mathematical configuration and computation parameters.
///
/// Stores the core mathematical settings for fractal computation including
/// image dimensions, coordinate area, and iteration limits. The width and height
/// are stored as strings to support direct binding to UI input fields.
///
/// # Examples
///
/// ```rust
/// let math_state = MathState::new(
///     Size2D::new(800,600),
///     some_coordinate_rect,
///     "1000".to_string(),
/// );
/// ```
pub struct MathState {
    /// The computational used math area with coordinates shifted into the pixel centers
    pub area: RasteredMathArea,
    /// Maximum iteration count for fractal computation (stored as string for UI binding)
    pub max_iteration: u32,
}

impl MathState {
    /// Creates a new mathematical state with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `stage_size` - Size of the computation stage
    /// * `area` - Mathematical coordinate rectangle
    /// * `max_iteration` - Maximum iteration count as string
    pub fn new(area: RasteredMathArea, max_iteration: u32) -> Self {
        MathState {
            area,
            max_iteration,
        }
    }
    /// Creates mathematical state from existing MathData and dimensions.
    ///
    /// Convenience constructor that extracts coordinate area and iteration
    /// count from a MathData instance while setting custom dimensions.
    ///
    /// # Arguments
    ///
    /// * `stage_size` - Size of the computation stage
    /// * `data` - Source MathData containing coordinates and iteration limit
    pub fn from_math_data(stage_size: Size2D<u32, StageSpace>, data: MathData) -> Self {
        Self::new(
            RasteredMathArea::new(
                MathArea::from_rect_f64(data.coordinates()).unwrap(),
                stage_size,
            ),
            data.max_iteration(),
        )
    }
}

impl Default for MathState {
    /// Creates default mathematical state with 800x600 dimensions
    /// and full Mandelbrot set view.
    fn default() -> Self {
        Self::from_math_data(
            Size2D::new(800, 600),
            MathPreset::preset(&MathPreset::MandelbrotFull),
        )
    }
}

/// Defines how the computed fractal image is rendered within the available canvas space.
///
/// Different schemes handle aspect ratio mismatches between the computed image
/// and the display canvas in various ways, offering trade-offs between
/// completeness, scaling, and visual appeal.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageRenderScheme {
    /// Scale image to fill canvas, cropping excess portions
    Cropped,
    /// Scale to fill canvas, show background for missing areas
    FilledWithBackground,
    /// Scale to fill canvas, blank areas instead of background
    Filled,
    /// No upscaling, show background for unfilled areas
    ShrunkWithBackground,
    /// No upscaling, blank areas instead of background
    Shrunk,
    /// Center image without scaling, show background around it
    CenteredWithBackground,
    /// Center image without scaling, blank areas around it
    Centered,
}

impl ImageRenderScheme {
    /// Returns all available render scheme variants.
    ///
    /// Useful for populating UI selection lists.
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
    /// Returns human-readable name for the render scheme.
    ///
    /// Used for displaying options in the user interface.
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
    /// Determines if this scheme requires background rendering.
    ///
    /// Returns true for schemes that show background instead of
    /// blank areas when the image doesn't fill the entire canvas.
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

/// Visual configuration and user interface settings.
///
/// Manages all aspects of how the fractal is displayed including
/// color schemes, rendering options, and UI visibility settings.
/// These settings affect visualization but not the underlying
/// mathematical computation.
pub struct VizState {
    /// Mathematical preset for quick coordinate area selection
    pub math_preset: MathPreset,
    /// True at startup so that first computation is performed automatically
    pub auto_start_computation: bool,
    /// Whether the control sidebar is currently visible
    pub sidebar_visible: bool,
    /// Color gradient scheme for fractal visualization
    pub gradient_color_preset: GradientColorPreset,
    /// Stripe count of the gradient colors
    pub gradient_color_stripes: u32,
    /// Offset to start when cyclint the gradient colors
    pub gradient_color_offset: u32,
    /// Function mapping iteration count to color position
    pub iteration_assignment: IterationAssignment,
    /// How the computed image fits within the display canvas
    pub render_scheme: ImageRenderScheme,
}

impl VizState {
    /// Creates new visual state with specified configuration.
    ///
    /// # Arguments
    ///
    /// * `math_preset` - Mathematical preset for coordinate selection
    /// * `auto_start_computation` - Enable automatic computation restart
    /// * `sidebar_visible` - Initial sidebar visibility
    /// * `gradient_color_preset` - Color scheme for fractal rendering
    /// * `iteration_assignment` - Iteration-to-color mapping function
    /// * `render_scheme` - Image scaling and positioning method
    pub fn new(
        math_preset: MathPreset,
        auto_start_computation: bool,
        sidebar_visible: bool,
        gradient_color_preset: GradientColorPreset,
        gradient_color_stripes: u32,
        gradient_color_offset: u32,
        iteration_assignment: IterationAssignment,
        render_scheme: ImageRenderScheme,
    ) -> Self {
        VizState {
            math_preset,
            auto_start_computation,
            sidebar_visible,
            gradient_color_preset,
            gradient_color_stripes,
            gradient_color_offset,
            iteration_assignment,
            render_scheme,
        }
    }
}

impl Default for VizState {
    /// Creates default visual state with sensible initial settings.
    ///
    /// Uses full Mandelbrot preset, auto-computation enabled, visible sidebar,
    /// sunrise color scheme, linear iteration assignment, and filled rendering.
    fn default() -> Self {
        Self::new(
            MathPreset::MandelbrotFull,
            true,
            true,
            GradientColorPreset::Sunrise,
            256,
            0,
            IterationAssignment::Linear,
            ImageRenderScheme::FilledWithBackground,
        )
    }
}

/// Manages zoom operation state during interactive zooming.
///
/// Tracks accumulated scroll wheel input and timing to implement
/// a zoom system with timeout-based completion. The zoom factor
/// is calculated using an exponential formula based on wheel ticks.
///
/// # Zoom Formula
///
/// The zoom factor is calculated as: `factor = 2^(0.1 * ticks)`
/// This provides smooth, exponential zooming where:
/// - 10 ticks forward = 2x zoom in
/// - 10 ticks backward = 2x zoom out
pub struct ZoomState {
    /// Screen coordinate where zoom operation was initiated
    pub origin: Point,
    /// Accumulated mouse wheel scroll ticks (positive = zoom in)
    pub ticks: i32,
    /// Timestamp of the most recent zoom input
    pub last_action: Instant,
    /// Current zoom factor calculated from accumulated ticks
    pub factor: f32,
}

impl ZoomState {
    /// Converts accumulated wheel ticks to zoom factor.
    ///
    /// Uses exponential scaling: `2^(0.1 * ticks)` for smooth zooming.
    /// Positive ticks increase zoom (zoom in), negative decrease (zoom out).
    fn ticks_to_factor(ticks: i32) -> f32 {
        2.0_f32.powf(0.1 * ticks as f32)
    }
    /// Initiates a new zoom operation.
    ///
    /// # Arguments
    ///
    /// * `origin` - Screen coordinate where zoom was initiated
    /// * `ticks` - Initial wheel scroll ticks
    pub fn start(origin: Point, ticks: i32) -> Self {
        ZoomState {
            origin,
            ticks,
            last_action: Instant::now(),
            factor: Self::ticks_to_factor(ticks),
        }
    }
    /// Updates zoom state with additional wheel scroll input.
    ///
    /// Accumulates the tick offset, updates the timestamp, and
    /// recalculates the zoom factor.
    ///
    /// # Arguments
    ///
    /// * `ticks_offset` - Additional scroll ticks to accumulate
    pub fn update_ticks(&mut self, ticks_offset: i32) {
        self.ticks += ticks_offset;
        self.last_action = Instant::now();
        self.factor = Self::ticks_to_factor(self.ticks);
    }
    /// Checks if zoom operation has timed out.
    ///
    /// Returns true if the elapsed time since the last zoom input
    /// exceeds the specified maximum delay, indicating the zoom
    /// operation should be completed.
    ///
    /// # Arguments
    ///
    /// * `max_delay` - Maximum allowed time between zoom inputs
    pub fn is_timeout(&self, max_delay: Duration) -> bool {
        self.last_action.elapsed() >= max_delay
    }
}

/// Dynamic runtime state of the application.
///
/// Tracks temporary state that changes during application execution,
/// including computation status, rendering cache, and interactive
/// operations like zooming. This state is not persisted.
pub struct RuntimeState {
    /// Whether fractal computation is currently in progress
    pub computing: bool,
    /// Iced canvas cache for optimized rendering
    pub canvas_cache: Cache,
    /// Current zoom operation state, None when not zooming
    pub zoom: Option<ZoomState>,
    /// Flag whether the FractalCanvas is currently dragging (controlled by canvas), this should be unified with the zoom stuff
    pub canvas_is_dragging: bool,
}

impl RuntimeState {
    /// Creates new runtime state with specified computation status.
    ///
    /// # Arguments
    ///
    /// * `computing` - Initial computation state
    pub fn new(computing: bool) -> Self {
        RuntimeState {
            computing,
            canvas_cache: Cache::new(),
            zoom: None,
            canvas_is_dragging: false,
        }
    }
}

impl Default for RuntimeState {
    /// Creates default runtime state with computation stopped.
    fn default() -> Self {
        Self::new(false)
    }
}

/// Complete application state container.
///
/// Aggregates all state categories (mathematical, visual, runtime) and
/// manages the dual-storage architecture. The storage components are
/// Optional to support initialization phases where they haven't been
/// created yet.
///
/// # Architecture
///
/// - `storage`: VizStorage for sequential visualization access
/// - `comp_storage`: CompStorage wrapped in Arc for parallel computation
/// - `engine`: Mandelbrot computation engine
/// - State is organized into logical categories for maintainability
pub struct AppState {
    /// Visualization storage for sequential rendering access
    pub storage: Option<VizStorage>,
    /// Computation storage wrapped in Arc for parallel access
    pub comp_storage: Option<Arc<CompStorage>>,
    /// Mandelbrot computation engine
    pub engine: Option<MandelbrotEngine>,
    /// Mathematical configuration and parameters
    pub math: MathState,
    /// Visual settings and UI configuration
    pub viz: VizState,
    /// Dynamic runtime state
    pub runtime: RuntimeState,
}

impl Default for AppState {
    /// Creates default application state with uninitialized storage.
    ///
    /// Storage components are None initially and will be created
    /// when the application starts computation.
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

/// Launches the Iced application with configured handlers.
///
/// Sets up the application with the update, view, and subscription
/// functions, then starts the Iced event loop.
///
/// # Returns
///
/// Returns `iced::Result` indicating success or failure of application startup.
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
