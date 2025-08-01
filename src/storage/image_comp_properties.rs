//! Mathematical coordinate system transformations for fractal computation.
//!
//! This module provides the mathematical foundation for converting between different
//! coordinate systems used in fractal visualization:
//!
//! - **Pixel Coordinates** (`StageSpace`): Integer pixel positions in the image
//! - **Mathematical Coordinates** (`MathSpace`): Real number coordinates in the complex plane
//!
//! # Core Concepts
//!
//! ## Coordinate Systems
//!
//! The system manages transformations between:
//! - **Screen Space**: UI pixel coordinates (top-left origin, y increases downward)
//! - **Stage Space**: Computation pixel coordinates (matches screen space dimensions)
//! - **Math Space**: Mathematical complex plane coordinates (arbitrary origin, y increases upward)
//!
//! ## Key Transformations
//!
//! - **Pixel ↔ Math**: Convert between discrete pixels and continuous mathematical coordinates
//! - **Panning**: Translate the viewed area while preserving scale
//! - **Zooming**: Scale the viewed area around a specific point
//! - **Rectification**: Ensure square pixels for accurate aspect ratios
//!
//! # Architecture
//!
//! - `StageProperties`: Core coordinate transformation engine
//! - `ImageCompProperties`: Adds iteration limits to stage properties
//! - `StageState`: Tracks computation progress state
//!
//! This system enables interactive navigation (pan/zoom) while maintaining
//! mathematical precision and supporting incremental computation.

use euclid::{Point2D, Rect, Size2D, Vector2D};

use crate::storage::coord_spaces::{MathSpace, StageSpace};

/// Core coordinate transformation engine for fractal computation.
///
/// Manages the relationship between pixel coordinates (discrete screen positions)
/// and mathematical coordinates (continuous complex plane values). This struct
/// provides the mathematical foundation for interactive fractal exploration
/// including panning, zooming, and coordinate conversion.
///
/// # Mathematical Model
///
/// The coordinate system maps a rectangular region of the complex plane
/// onto a discrete pixel grid:
///
/// ```text
/// Mathematical Space (continuous)     Pixel Space (discrete)
/// ┌──────────────────────────────┐     ┌─────────────────┐
/// │ (min_x, max_y)               │ ←→  │ (0,0)           │
/// │                              │     │                 │
/// │         Complex              │     │     Pixel       │
/// │         Plane                │     │     Grid        │
/// │                              │     │                 │
/// │              (max_x, min_y)  │     │       (w-1,h-1) │
/// └──────────────────────────────┘     └─────────────────┘
/// ```
///
/// # Fields
///
/// The transformation is defined by:
/// - `coo`: Mathematical rectangle being viewed
/// - `pixels`: Pixel dimensions of the image
/// - `dotsize`: Size of each pixel in mathematical units
/// - `coo_base`: Mathematical coordinate of pixel (0,0) center
/// - `coo_correction`: Offset to center pixels on their mathematical coordinates
#[derive(Debug, Clone, Copy)]
pub struct StageProperties {
    /// Mathematical rectangle defining the viewed area in the complex plane
    pub coo: Rect<f64, MathSpace>,
    /// Pixel dimensions of the computation stage (width × height)
    pub pixels: Size2D<u32, StageSpace>,
    /// Size of each pixel in mathematical units (real × imaginary)
    pub dotsize: Size2D<f64, MathSpace>,
    /// Mathematical coordinate corresponding to the center of pixel (0,0)
    pub coo_base: Point2D<f64, MathSpace>,
    /// Offset vector for centering pixels on their mathematical coordinates
    pub coo_correction: Vector2D<f64, MathSpace>,
}

impl StageProperties {
    /// Creates a new coordinate transformation system.
    ///
    /// Establishes the mapping between a mathematical rectangle and a pixel grid.
    /// The transformation ensures that:
    /// - Pixel (0,0) maps to the top-left area of the mathematical rectangle
    /// - Y-axis is flipped (positive Y goes up in math space, down in pixel space)
    /// - Pixels are centered on their corresponding mathematical coordinates
    ///
    /// # Arguments
    ///
    /// * `coo` - Mathematical rectangle to be displayed
    /// * `pixels` - Pixel dimensions of the output image
    ///
    /// # Mathematical Setup
    ///
    /// - `dotsize`: Size of each pixel in mathematical units
    /// - `coo_base`: Mathematical coordinate of pixel (0,0) center
    /// - `coo_correction`: Offset to handle pixel centering and Y-axis flip
    ///
    /// # Returns
    ///
    /// A new `StageProperties` instance ready for coordinate transformations
    pub fn new(coo: Rect<f64, MathSpace>, pixels: Size2D<u32, StageSpace>) -> StageProperties {
        let dotsize = Size2D::new(
            coo.width() / pixels.width as f64,
            coo.height() / pixels.height as f64,
        );
        let coo_correction = Vector2D::new(dotsize.width / 2.0, -dotsize.height / 2.0);
        let coo_base = Point2D::new(coo.min_x(), coo.max_y()) + coo_correction;
        StageProperties {
            coo,
            pixels,
            dotsize,
            coo_base,
            coo_correction,
        }
    }

    /// Converts pixel displacement to mathematical displacement.
    ///
    /// Transforms a pixel offset vector into the corresponding mathematical
    /// offset vector, accounting for pixel size and Y-axis orientation.
    ///
    /// # Coordinate System Notes
    ///
    /// - X-axis: Positive pixel offset → negative mathematical offset
    /// - Y-axis: Positive pixel offset → positive mathematical offset
    /// - This handles the Y-axis flip between coordinate systems
    ///
    /// # Arguments
    ///
    /// * `offset` - Displacement vector in pixel coordinates
    ///
    /// # Returns
    ///
    /// Equivalent displacement vector in mathematical coordinates
    ///
    /// # Usage
    ///
    /// Used for panning operations where UI drag distances need to be
    /// converted to mathematical coordinate shifts.
    pub fn pixel_to_math_offset(
        &self,
        offset: Vector2D<i32, StageSpace>,
    ) -> Vector2D<f64, MathSpace> {
        Vector2D::new(
            offset.x as f64 * -self.dotsize.width,
            offset.y as f64 * self.dotsize.height,
        )
    }

    /// Creates a copy with the viewed area shifted by a mathematical offset.
    ///
    /// Translates the mathematical rectangle while preserving all scaling
    /// and aspect ratio properties. This is the fundamental operation for
    /// panning the fractal view.
    ///
    /// # Arguments
    ///
    /// * `offset` - Mathematical displacement vector
    ///
    /// # Returns
    ///
    /// New `StageProperties` with translated coordinate system
    ///
    /// # Preserves
    ///
    /// - Pixel dimensions
    /// - Dot size (scale)
    /// - Aspect ratio
    /// - All relative positioning
    ///
    /// # Use Cases
    ///
    /// - Interactive panning
    /// - Programmatic view adjustment
    /// - Animation sequences
    pub fn shifted_clone_by_math(&self, offset: Vector2D<f64, MathSpace>) -> StageProperties {
        let new_coo = self.coo.translate(offset);
        let coo_base = Point2D::new(
            new_coo.min_x() + (self.dotsize.width / 2.0),
            new_coo.max_y() - (self.dotsize.height / 2.0),
        );
        StageProperties {
            coo: new_coo,
            pixels: self.pixels,
            dotsize: self.dotsize,
            coo_base,
            coo_correction: self.coo_correction,
        }
    }

    /// Creates a copy with the viewed area shifted by a pixel offset.
    ///
    /// Convenience method that converts pixel displacement to mathematical
    /// displacement and then applies the shift. Commonly used for UI-driven
    /// panning where drag distances are measured in pixels.
    ///
    /// # Arguments
    ///
    /// * `offset` - Pixel displacement vector
    ///
    /// # Returns
    ///
    /// New `StageProperties` with translated coordinate system
    ///
    /// # Implementation
    ///
    /// This is equivalent to:
    /// ```rust
    /// self.shifted_clone_by_math(self.pixel_to_math_offset(offset))
    /// ```
    pub fn shifted_clone_by_pixels(&self, offset: Vector2D<i32, StageSpace>) -> StageProperties {
        self.shifted_clone_by_math(self.pixel_to_math_offset(offset))
    }

    /// Creates a copy with the viewed area zoomed around a specific pixel.
    ///
    /// Scales the mathematical coordinate system around a given pixel point,
    /// effectively zooming in (factor > 1.0) or out (factor < 1.0) while
    /// keeping the specified pixel at the same mathematical coordinate.
    ///
    /// # Algorithm
    ///
    /// 1. Convert origin pixel to mathematical coordinate
    /// 2. Scale dot size by the zoom factor
    /// 3. Recompute coordinate base to keep origin fixed
    /// 4. Adjust rectangle bounds to match new scale
    ///
    /// # Arguments
    ///
    /// * `origin` - Pixel coordinate that remains fixed during zoom
    /// * `factor` - Zoom factor (>1.0 = zoom in, <1.0 = zoom out)
    ///
    /// # Returns
    ///
    /// New `StageProperties` with scaled coordinate system
    ///
    /// # Mathematical Invariant
    ///
    /// The mathematical coordinate of the origin pixel remains unchanged:
    /// ```text
    /// old_props.pix_to_math(origin) == new_props.pix_to_math(origin)
    /// ```
    ///
    /// # Usage
    ///
    /// - Mouse wheel zooming (origin = cursor position)
    /// - Pinch-to-zoom gestures
    /// - Programmatic zoom animations
    pub fn zoomed_clone_by_pixels(&self, origin: Point2D<i32, StageSpace>, factor: f64) -> Self {
        let math_origin = self.pix_to_math(origin);
        let new_dotsize = self.dotsize / factor;
        let new_coo_correction = Vector2D::new(new_dotsize.width / 2.0, -new_dotsize.height / 2.0);
        let new_coo_base = Point2D::new(
            math_origin.x - (origin.x as f64 * new_dotsize.width),
            math_origin.y + (origin.y as f64 * new_dotsize.height),
        );
        let new_top_left = new_coo_base - new_coo_correction;
        let new_bottom_right = new_top_left
            + Vector2D::new(
                new_dotsize.width * self.pixels.width as f64,
                -new_dotsize.height * self.pixels.height as f64,
            );
        let new_coo = Rect::from_points([new_top_left, new_bottom_right]);
        StageProperties {
            coo: new_coo,
            pixels: self.pixels,
            dotsize: new_dotsize,
            coo_base: new_coo_base,
            coo_correction: new_coo_correction,
        }
    }

    /// Converts pixel X coordinate to mathematical X coordinate.
    ///
    /// # Arguments
    ///
    /// * `x_pix` - Pixel X coordinate (0 to width-1)
    ///
    /// # Returns
    ///
    /// Mathematical X coordinate (real part of complex number)
    pub fn x(&self, x_pix: i32) -> f64 {
        self.coo_base.x + x_pix as f64 * self.dotsize.width
    }

    /// Converts pixel Y coordinate to mathematical Y coordinate.
    ///
    /// Note the Y-axis flip: pixel Y increases downward,
    /// mathematical Y increases upward.
    ///
    /// # Arguments
    ///
    /// * `y_pix` - Pixel Y coordinate (0 to height-1)
    ///
    /// # Returns
    ///
    /// Mathematical Y coordinate (imaginary part of complex number)
    pub fn y(&self, y_pix: i32) -> f64 {
        self.coo_base.y - y_pix as f64 * self.dotsize.height
    }

    /// Checks if a pixel coordinate is within the stage bounds.
    ///
    /// # Arguments
    ///
    /// * `p` - Pixel coordinate to validate
    ///
    /// # Returns
    ///
    /// `true` if the coordinate is within [0, width) × [0, height)
    #[allow(dead_code)]
    pub fn is_valid_pix(&self, p: &Point2D<i32, StageSpace>) -> bool {
        p.x >= 0 && p.x < self.pixels.width as i32 && p.y >= 0 && p.y < self.pixels.height as i32
    }

    /// Converts pixel coordinates to mathematical coordinates.
    ///
    /// Transforms a discrete pixel position to the corresponding
    /// continuous mathematical coordinate in the complex plane.
    ///
    /// # Arguments
    ///
    /// * `pix` - Pixel coordinate
    ///
    /// # Returns
    ///
    /// Mathematical coordinate (complex number as Point2D)
    ///
    /// # Usage
    ///
    /// Essential for fractal computation - determines which complex
    /// number to iterate for each pixel.
    #[allow(dead_code)]
    pub fn pix_to_math(&self, pix: Point2D<i32, StageSpace>) -> Point2D<f64, MathSpace> {
        Point2D::new(self.x(pix.x), self.y(pix.y))
    }

    /// Converts pixel to mathematical coordinates with bounds checking.
    ///
    /// Safely transforms pixel coordinates to mathematical coordinates,
    /// returning `None` if the pixel is outside the stage bounds.
    ///
    /// # Arguments
    ///
    /// * `pix` - Pixel coordinate to convert
    ///
    /// # Returns
    ///
    /// `Some(math_coord)` if pixel is valid, `None` otherwise
    #[allow(dead_code)]
    pub fn pix_to_math_if_valid(
        &self,
        pix: Point2D<i32, StageSpace>,
    ) -> Option<Point2D<f64, MathSpace>> {
        Some(pix)
            .filter(|p| self.is_valid_pix(p))
            .map(|p| self.pix_to_math(p))
    }

    /// Converts mathematical coordinates to pixel coordinates.
    ///
    /// Transforms a continuous mathematical coordinate to the corresponding
    /// discrete pixel position. May return coordinates outside the valid
    /// pixel range if the mathematical coordinate is outside the viewed area.
    ///
    /// # Arguments
    ///
    /// * `math` - Mathematical coordinate (complex number as Point2D)
    ///
    /// # Returns
    ///
    /// Pixel coordinate (may be outside valid bounds)
    ///
    /// # Usage
    ///
    /// Useful for mapping mathematical features back to screen positions,
    /// such as highlighting specific mathematical points.
    #[allow(dead_code)]
    pub fn math_to_pix(&self, math: Point2D<f64, MathSpace>) -> Point2D<i32, StageSpace> {
        let x = ((math.x - self.coo_base.x) / self.dotsize.width).floor() as i32;
        let y = ((self.coo_base.y - math.y) / self.dotsize.height).floor() as i32;
        Point2D::new(x, y)
    }

    /// Converts mathematical to pixel coordinates with bounds checking.
    ///
    /// Safely transforms mathematical coordinates to pixel coordinates,
    /// returning `None` if the resulting pixel would be outside the stage bounds.
    ///
    /// # Arguments
    ///
    /// * `math` - Mathematical coordinate to convert
    ///
    /// # Returns
    ///
    /// `Some(pixel_coord)` if within bounds, `None` otherwise
    #[allow(dead_code)]
    pub fn math_to_pix_if_valid(
        &self,
        math: Point2D<f64, MathSpace>,
    ) -> Option<Point2D<i32, StageSpace>> {
        Some(self.math_to_pix(math)).filter(|p| self.is_valid_pix(p))
    }

    /// Creates a rectified version ensuring square pixels.
    ///
    /// Adjusts the mathematical coordinate system to guarantee that each pixel
    /// represents a square area in the complex plane. This prevents distortion
    /// when the mathematical rectangle has a different aspect ratio than the
    /// pixel dimensions.
    ///
    /// # Process
    ///
    /// 1. Calculates uniform dot size (min or max of current dot sizes)
    /// 2. Centers the new rectangular area on the original area's center
    /// 3. Adjusts mathematical bounds while preserving pixel dimensions
    ///
    /// # Arguments
    ///
    /// * `inner` - Determines sizing strategy:
    ///   - `true`: New area fits entirely within original (may show less)
    ///   - `false`: New area entirely contains original (may show more)
    ///
    /// # Returns
    ///
    /// New `StageProperties` with square pixels, or clone if already square
    ///
    /// # Mathematical Guarantee
    ///
    /// After rectification: `dotsize.width == dotsize.height`
    ///
    /// # Use Cases
    ///
    /// - Preparing images for accurate mathematical visualization
    /// - Correcting aspect ratio distortions
    /// - Ensuring geometric features appear correctly
    pub fn rectified(&self, inner: bool) -> StageProperties {
        let dotsize_min = self.dotsize.width.min(self.dotsize.height);
        let dotsize_max = self.dotsize.width.max(self.dotsize.height);
        if (1.0 - (dotsize_min / dotsize_max)) < 1e-5 {
            self.clone()
        } else {
            let dotsize = if inner { dotsize_min } else { dotsize_max };
            let center = Point2D::new(
                self.coo.min_x() + (self.coo.width() / 2.0),
                self.coo.min_y() + (self.coo.height() / 2.0),
            );
            let dist = Size2D::new(
                dotsize * ((self.pixels.width as f64) / 2.0),
                dotsize * ((self.pixels.height as f64) / 2.0),
            );
            StageProperties::new(
                Rect::from_points([center - dist, center + dist]),
                self.pixels,
            )
        }
    }
}

/// Complete mathematical configuration for fractal image computation.
///
/// Combines coordinate transformation capabilities with computation parameters
/// to provide a complete specification for fractal image generation. This
/// struct represents everything needed to reproduce a specific fractal image:
/// the mathematical area being viewed, the pixel resolution, and the
/// computation depth.
///
/// # Architecture
///
/// - Builds on `StageProperties` for coordinate transformations
/// - Adds `max_iteration` for computation control
/// - Provides high-level operations for interactive manipulation
///
/// # Usage
///
/// This is the primary configuration object passed to the computation engine
/// and used throughout the visualization pipeline.
#[derive(Debug, Clone, Copy)]
pub struct ImageCompProperties {
    /// Coordinate transformation system for pixel ↔ mathematical conversion
    pub stage_properties: StageProperties,
    /// Maximum iteration count for fractal computation
    pub max_iteration: u32,
}

impl ImageCompProperties {
    /// Creates new image computation properties.
    ///
    /// # Arguments
    ///
    /// * `stage_properties` - Coordinate transformation system
    /// * `max_iteration` - Maximum iteration count for fractal computation
    ///
    /// # Returns
    ///
    /// A new `ImageCompProperties` instance ready for computation
    pub fn new(stage_properties: StageProperties, max_iteration: u32) -> Self {
        ImageCompProperties {
            stage_properties,
            max_iteration,
        }
    }
    /// Creates a rectified copy with square pixels.
    ///
    /// Delegates to the underlying `StageProperties::rectified()` method
    /// while preserving the iteration count.
    ///
    /// # Arguments
    ///
    /// * `inner` - Sizing strategy (see `StageProperties::rectified()`)
    ///
    /// # Returns
    ///
    /// New `ImageCompProperties` with square pixels
    pub fn rectified(&self, inner: bool) -> Self {
        ImageCompProperties {
            stage_properties: self.stage_properties.rectified(inner),
            max_iteration: self.max_iteration,
        }
    }

    /// Creates a copy shifted by a pixel offset.
    ///
    /// Delegates to `StageProperties::shifted_clone_by_pixels()` while
    /// preserving the iteration count. Used for interactive panning.
    ///
    /// # Arguments
    ///
    /// * `offset` - Pixel displacement vector
    ///
    /// # Returns
    ///
    /// New `ImageCompProperties` with translated coordinate system
    pub fn shifted_clone_by_pixels(&self, offset: Vector2D<i32, StageSpace>) -> Self {
        ImageCompProperties {
            stage_properties: self.stage_properties.shifted_clone_by_pixels(offset),
            max_iteration: self.max_iteration,
        }
    }

    /// Creates a copy zoomed around a specific pixel.
    ///
    /// Delegates to `StageProperties::zoomed_clone_by_pixels()` while
    /// preserving the iteration count. Used for interactive zooming.
    ///
    /// # Arguments
    ///
    /// * `origin` - Pixel coordinate that remains fixed during zoom
    /// * `factor` - Zoom factor (f32 for UI compatibility)
    ///
    /// # Returns
    ///
    /// New `ImageCompProperties` with scaled coordinate system
    pub fn zoomed_clone_by_pixels(&self, origin: Point2D<i32, StageSpace>, factor: f32) -> Self {
        ImageCompProperties {
            stage_properties: self
                .stage_properties
                .zoomed_clone_by_pixels(origin, factor as f64),
            max_iteration: self.max_iteration,
        }
    }

    /// Create a cloned properties storage where the max_iteration setting is takenfrom the app's model.
    pub fn max_iteration_changed_clone(&self, new_max_iteration: u32) -> Self {
        ImageCompProperties {
            stage_properties: self.stage_properties.clone(),
            max_iteration: new_max_iteration,
        }
    }

    /// Converts pixel displacement to mathematical displacement.
    ///
    /// Convenience method delegating to the underlying `StageProperties`.
    ///
    /// # Arguments
    ///
    /// * `offset` - Pixel displacement vector
    ///
    /// # Returns
    ///
    /// Equivalent mathematical displacement vector
    pub fn pixel_to_math_offset(
        &self,
        offset: Vector2D<i32, StageSpace>,
    ) -> Vector2D<f64, MathSpace> {
        self.stage_properties.pixel_to_math_offset(offset)
    }

    /// Creates a copy shifted by a mathematical offset.
    ///
    /// Delegates to `StageProperties::shifted_clone_by_math()` while
    /// preserving the iteration count.
    ///
    /// # Arguments
    ///
    /// * `offset` - Mathematical displacement vector
    ///
    /// # Returns
    ///
    /// New `ImageCompProperties` with translated coordinate system
    pub fn shifted_clone_by_math(&self, offset: Vector2D<f64, MathSpace>) -> Self {
        ImageCompProperties {
            stage_properties: self.stage_properties.shifted_clone_by_math(offset),
            max_iteration: self.max_iteration,
        }
    }
}

/// Represents the current computation state of a fractal computation stage.
///
/// Tracks the lifecycle of fractal computation from initialization through
/// completion, including intermediate states for pausing and resuming.
/// This state information is used for progress tracking, UI updates, and
/// coordination between computation and visualization threads.
///
/// # State Transitions
///
/// ```text
/// Initialized → Evolving ←→ Stalled
///                  ↓
///               Completed
/// ```
///
/// - **Forward Progress**: Initialized → Evolving → Completed
/// - **Interruption**: Evolving ↔ Stalled (can resume)
/// - **Completion**: Evolving → Completed (terminal state)
///
/// # Thread Safety
///
/// This enum is `Copy` and designed for atomic updates in concurrent
/// computation scenarios.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StageState {
    /// Stage created but computation has not yet begun
    Initialized,
    /// Active computation in progress - content changes continuously
    Evolving,
    /// Computation paused or stopped - no changes expected until resumed
    Stalled,
    /// Computation finished - content matches stage properties exactly
    Completed,
}

// end of file
