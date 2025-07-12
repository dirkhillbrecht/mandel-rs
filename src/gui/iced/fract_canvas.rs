//! Interactive fractal canvas with pan/zoom navigation and real-time rendering.
//!
//! This module implements a sophisticated interactive canvas that displays fractal
//! images with support for mouse-driven navigation (panning and zooming), multiple
//! rendering schemes, and real-time visual feedback during user interactions.
//!
//! # Architecture Overview
//!
//! ## Core Components
//!
//! - **FractalCanvas**: Main canvas implementation with rendering and event handling
//! - **Pixels**: Efficient pixel buffer management with transformation support
//! - **ImageInCanvas**: Coordinate transformation system for mouse ↔ image mapping
//! - **CanvasState**: State machine for interactive operations (Idle, Drag)
//!
//! ## Rendering Pipeline
//!
//! ```text
//! VizStorage → Pixel Generation → Transformation → Canvas Rendering
//!     ↓              ↓                ↓              ↓
//! DataPoints → RGBA Pixels → Pan/Zoom Effects → Iced Canvas
//! ```
//!
//! ## Interaction System
//!
//! ### Panning (Mouse Drag)
//! 1. **Start**: Left click captures start position
//! 2. **Preview**: Real-time visual shift during drag
//! 3. **Commit**: Release triggers coordinate update message
//!
//! ### Zooming (Mouse Wheel)
//! 1. **Initiate**: Wheel scroll starts zoom accumulation
//! 2. **Accumulate**: Additional scrolls add to zoom factor
//! 3. **Complete**: Timeout triggers coordinate transformation
//!
//! ## Rendering Schemes
//!
//! - **Cropped**: Scale to fill canvas, crop excess
//! - **Filled**: Scale to fit, show background if needed
//! - **Centered**: No scaling, center image in canvas
//!
//! # Performance Characteristics
//!
//! - **Pixel Interpolation**: Estimates uncomputed pixels from neighbors
//! - **Visual Feedback**: Non-blocking preview during interactions
//! - **Canvas Caching**: Iced's built-in caching for efficient redraws
//! - **Memory Efficiency**: RGBA pixel buffers with lazy allocation

use crate::{
    gui::iced::{
        app::{AppState, ImageRenderScheme, ZoomState},
        message::Message,
    },
    storage::{
        coord_spaces::StageSpace,
        data_point::DataPoint,
        visualization::{coloring::base::GradientColors, viz_storage::VizStorage},
    },
};
use euclid::Vector2D;
use iced::{
    Point, Rectangle, Size,
    mouse::{self, ScrollDelta},
    widget::{
        canvas::{self, Event, event},
        image::Handle,
    },
};

/// Efficient RGBA pixel buffer with transformation capabilities.
///
/// Manages a rectangular region of RGBA pixel data with support for
/// geometric transformations (shifting, zooming) and efficient extraction
/// of sub-regions. Used as the primary pixel storage for canvas rendering.
///
/// # Memory Layout
///
/// Pixels are stored in row-major order as RGBA bytes:
/// `[R, G, B, A, R, G, B, A, ...]`
///
/// # Performance
///
/// - Optimized for sequential access patterns
/// - Supports in-place transformations where possible
/// - Efficient partial extraction for different rendering schemes
struct Pixels {
    /// Top-left corner of this pixel region in parent coordinate system
    origin: Point<usize>,
    /// Dimensions of the pixel buffer (width × height)
    size: Size<usize>,
    /// RGBA pixel data in row-major order (4 bytes per pixel)
    pixels: Vec<u8>,
}

impl Pixels {
    /// Creates a new pixel buffer with specified origin, size, and data.
    ///
    /// # Arguments
    ///
    /// * `origin` - Top-left corner in parent coordinate system
    /// * `size` - Buffer dimensions (width × height)
    /// * `pixels` - RGBA pixel data (must be `width * height * 4` bytes)
    pub fn new(origin: Point<usize>, size: Size<usize>, pixels: Vec<u8>) -> Self {
        Pixels {
            origin,
            size,
            pixels,
        }
    }
    /// Creates a pixel buffer with origin at (0,0).
    ///
    /// Convenience constructor for pixel buffers that don't need offset.
    ///
    /// # Arguments
    ///
    /// * `size` - Buffer dimensions
    /// * `pixels` - RGBA pixel data
    pub fn at_zero_origin(size: Size<usize>, pixels: Vec<u8>) -> Self {
        Self::new(Point::new(0, 0), size, pixels)
    }
    /// Extracts a rectangular sub-region from this pixel buffer.
    ///
    /// Creates a new pixel buffer containing only the specified rectangular
    /// region. Efficiently handles both full-width extractions (single memcpy)
    /// and partial-width extractions (line-by-line copy).
    ///
    /// # Arguments
    ///
    /// * `image_part` - Rectangle defining the region to extract
    ///
    /// # Returns
    ///
    /// New `Pixels` buffer containing the extracted region
    ///
    /// # Performance
    ///
    /// - **Optimal**: Full-width extractions use single memory copy
    /// - **Standard**: Partial-width extractions copy line by line
    pub fn extract_part(&self, image_part: iced::Rectangle) -> Pixels {
        let new_linestart = image_part.x.abs() as usize;
        let new_firstline: usize = image_part.y.abs() as usize;
        let new_size = Size::new(image_part.width as usize, image_part.height as usize);
        let bytecount = new_size.width * new_size.height * 4;
        let mut new_pixels = Vec::with_capacity(bytecount);
        if new_linestart == 0 && new_size.width == self.size.width {
            // Copy one chunk covering the given number of lines
            let firstpix = self.size.width * image_part.y as usize * 4;
            new_pixels.extend_from_slice(&self.pixels[firstpix..firstpix + bytecount]);
        } else {
            // Copy part of each line over the whole height
            for line in new_firstline..new_firstline + new_size.height {
                let firstpix = (line * self.size.width + new_linestart) * 4;
                new_pixels.extend_from_slice(&self.pixels[firstpix..firstpix + new_size.width * 4]);
            }
        }
        Self::at_zero_origin(new_size, new_pixels)
    }
    /// Extracts a sub-region only if it differs from the current buffer.
    ///
    /// Optimizes the common case where the requested region matches
    /// the current buffer exactly, avoiding unnecessary memory allocation.
    ///
    /// # Arguments
    ///
    /// * `image_part` - Rectangle defining the desired region
    ///
    /// # Returns
    ///
    /// - `None` if the region matches current buffer exactly
    /// - `Some(Pixels)` with extracted region if different
    pub fn extract_part_if_needed(&self, image_part: iced::Rectangle) -> Option<Pixels> {
        if image_part.x.abs() as usize == self.origin.x
            && image_part.y.abs() as usize == self.origin.y
            && image_part.width.abs() as usize == self.size.width
            && image_part.height.abs() as usize == self.size.height
        {
            None
        } else {
            Some(self.extract_part(image_part))
        }
    }
    /// Creates a shifted copy of the pixel buffer for panning preview.
    ///
    /// Generates a new pixel buffer where the original image data is shifted
    /// by the specified offset. Areas not covered by the original data are
    /// filled with transparent black pixels. This provides real-time visual
    /// feedback during drag operations.
    ///
    /// # Algorithm
    ///
    /// 1. **Overlap Calculation**: Determines which pixels can be preserved
    /// 2. **Memory Layout**: Copies overlapping regions efficiently
    /// 3. **Gap Filling**: Fills empty areas with transparent pixels
    ///
    /// # Arguments
    ///
    /// * `offset` - Shift amount (positive = right/down)
    ///
    /// # Returns
    ///
    /// - `Some(Pixels)` with shifted image data
    /// - `None` if offset is effectively zero
    ///
    /// # Use Cases
    ///
    /// - Real-time panning preview during mouse drag
    /// - Non-destructive image positioning
    ///
    /// # Performance
    ///
    /// Optimized for common panning scenarios where most pixels are preserved.
    pub fn shift(&self, offset: Size) -> Option<Pixels> {
        if offset.width.abs() < 1e-2 && offset.height.abs() < 1e-2 {
            None
        } else {
            let ox = offset.width as i32;
            let oy = offset.height as i32;
            let empty_line_start = (ox.max(0) as usize).min(self.size.width);
            let empty_line_end = ((-ox).max(0) as usize).min(self.size.width);
            let empty_start_lines = (oy.max(0) as usize).min(self.size.height);
            let empty_end_lines = ((-oy).max(0) as usize).min(self.size.height);
            let line_width = self.size.width - (empty_line_start.max(empty_line_end));
            let first_line = empty_end_lines;
            let last_line = self.size.height - empty_start_lines;
            let mut new_pixels = Vec::with_capacity(self.size.width * self.size.height * 4);
            let one_pixel: [u8; 4] = [0, 0, 0, 0];
            for _ in 0..empty_start_lines {
                for _ in 0..self.size.width {
                    new_pixels.extend_from_slice(&one_pixel);
                }
            }
            for line in first_line..last_line {
                for _ in 0..empty_line_start {
                    new_pixels.extend_from_slice(&one_pixel);
                }
                let first_idx = (line * self.size.width + empty_line_end) * 4;
                let last_idx = first_idx + line_width as usize * 4;
                new_pixels.extend_from_slice(&self.pixels[first_idx..last_idx]);
                for _ in 0..empty_line_end {
                    new_pixels.extend_from_slice(&one_pixel);
                }
            }
            for _ in 0..empty_end_lines {
                for _ in 0..self.size.width {
                    new_pixels.extend_from_slice(&one_pixel);
                }
            }
            Some(Pixels::at_zero_origin(self.size, new_pixels))
        }
    }
    /// Creates a zoomed copy of the pixel buffer for zoom preview.
    ///
    /// Generates a transformed pixel buffer that visually represents the
    /// zoom operation in progress. Uses simple nearest-neighbor sampling
    /// to provide immediate visual feedback during zoom interactions.
    ///
    /// # Algorithm
    ///
    /// 1. **Zoom Center**: Uses zoom origin as the fixed point
    /// 2. **Scale Mapping**: Maps each output pixel to input coordinates
    /// 3. **Bounds Checking**: Fills out-of-bounds areas with black
    /// 4. **Nearest Neighbor**: Simple sampling for performance
    ///
    /// # Arguments
    ///
    /// * `zoom_state` - Current zoom operation state
    ///
    /// # Returns
    ///
    /// - `Some(Pixels)` with transformed image data
    /// - `None` if no zoom is active (ticks == 0)
    ///
    /// # Visual Quality
    ///
    /// This is a preview transformation optimized for speed over quality.
    /// The final zoom will use proper coordinate transformations.
    ///
    /// # Performance
    ///
    /// - Nearest-neighbor sampling for real-time feedback
    /// - Per-pixel coordinate calculation (not optimized for quality)
    pub fn zoom(&self, zoom_state: &ZoomState) -> Option<Pixels> {
        if zoom_state.ticks == 0 {
            None
        } else {
            let one_pixel: [u8; 4] = [0, 0, 0, 0];
            let zoom_part = 1.0 - 1.0 / zoom_state.factor;
            let leftpix = zoom_state.origin.x * zoom_part;
            let toppix = zoom_state.origin.y * zoom_part;
            let mut new_pixels = Vec::with_capacity(self.size.width * self.size.height * 4);
            let mut newx = Vec::with_capacity(self.size.width);
            for x in 0..self.size.width {
                newx.push(leftpix + x as f32 / zoom_state.factor);
            }
            for y in 0..self.size.height {
                let newy = (toppix + y as f32 / zoom_state.factor) as i32;
                for x in 0..self.size.width {
                    let newx = (leftpix + x as f32 / zoom_state.factor) as i32;
                    if newx < 0
                        || newx >= self.size.width as i32
                        || newy < 0
                        || newy >= self.size.height as i32
                    {
                        new_pixels.extend_from_slice(&one_pixel);
                    } else {
                        let first_idx = (self.size.width as i32 * newy + newx)
                            .max(0)
                            .min((self.size.width * self.size.height) as i32 - 1)
                            as usize
                            * 4;
                        new_pixels.extend_from_slice(&self.pixels[first_idx..first_idx + 4]);
                    }
                }
            }
            Some(Pixels::at_zero_origin(self.size, new_pixels))
        }
    }
    /// Modifies the alpha channel of all pixels.
    ///
    /// Updates the transparency of the entire pixel buffer, useful for
    /// creating background layers or visual effects during interactions.
    ///
    /// # Arguments
    ///
    /// * `new_alpha` - Alpha value (0.0 = transparent, 1.0 = opaque)
    ///
    /// # Performance
    ///
    /// Iterates through all pixels, modifying only the alpha channel.
    pub fn change_alpha(&mut self, new_alpha: f32) {
        let a = (new_alpha * 255.0) as u8;
        for p in 0..self.size.width * self.size.height {
            self.pixels[(p * 4) + 3] = a;
        }
    }
}

/// Defines the mapping between image and canvas regions for rendering.
///
/// Specifies which rectangular portion of the computed fractal image
/// should be rendered into which rectangular portion of the display canvas.
/// This system supports various aspect ratio handling strategies.
///
/// # Coordinate Systems
///
/// - **Image Coordinates**: Pixel positions in the computed fractal image
/// - **Canvas Coordinates**: Pixel positions in the display canvas
///
/// # Aspect Ratio Handling
///
/// Different rendering schemes use different strategies:
/// - **Crop**: Image fills canvas, excess cropped
/// - **Fit**: Entire image shown, canvas may have borders
/// - **Center**: No scaling, image centered in canvas
///
/// # Invariant
///
/// The aspect ratio of `used_image_part` should match the aspect ratio
/// of `used_canvas_part` to prevent distortion.
#[derive(Debug)]
struct UsedParts {
    /// Rectangular region of the fractal image to be displayed
    /// Coordinates: (x, y, width, height) in image pixel space
    /// Constraint: Must be within the actual image bounds
    pub used_image_part: iced::Rectangle,
    /// Rectangular region of the canvas where the image will be drawn
    /// Coordinates: (x, y, width, height) in canvas pixel space
    /// Constraint: Must be within the actual canvas bounds
    pub used_canvas_part: iced::Rectangle,
}

/// Complete coordinate transformation system for mouse-image interaction.
///
/// Provides bidirectional coordinate mapping between mouse positions on the
/// canvas and corresponding pixel positions in the fractal image. Essential
/// for interactive navigation and event handling.
///
/// # Coordinate Transformations
///
/// The system handles a multi-step transformation:
/// ```text
/// Mouse Screen Coords → Canvas Coords → Used Canvas → Used Image → Full Image
/// ```
///
/// Each step accounts for:
/// - Canvas positioning within the application window
/// - Rendering scheme (cropped, fitted, centered)
/// - Scaling and aspect ratio adjustments
///
/// # Usage
///
/// Primary use cases:
/// - Converting mouse clicks to fractal image pixels
/// - Determining if mouse is over the rendered image
/// - Mapping drag operations to coordinate shifts
#[derive(Debug)]
struct ImageInCanvas {
    /// Complete canvas bounds in application window coordinates
    /// Used for converting global mouse coordinates to canvas-relative coordinates
    pub canvas_bounds: iced::Rectangle,
    /// Dimensions of the complete fractal image being displayed
    /// Origin is always (0,0), this defines the full image size
    pub image_size: Size<f32>,
    /// Active rendering regions for current aspect ratio handling
    pub used_parts: UsedParts,
}

impl ImageInCanvas {
    /// Creates a coordinate transformation system for given parameters.
    ///
    /// Establishes the complete mapping between canvas and image coordinates
    /// based on the specified rendering scheme and dimensions.
    ///
    /// # Arguments
    ///
    /// * `canvas_bounds` - Canvas position and size in window coordinates
    /// * `image_size` - Fractal image dimensions
    /// * `render_scheme` - How to handle aspect ratio mismatches
    ///
    /// # Returns
    ///
    /// Configured `ImageInCanvas` ready for coordinate transformations
    pub fn init(
        canvas_bounds: iced::Rectangle,
        image_size: Size<f32>,
        render_scheme: ImageRenderScheme,
    ) -> Self {
        let canvas_size = canvas_bounds.size();
        ImageInCanvas {
            canvas_bounds,
            image_size,
            used_parts: match render_scheme {
                ImageRenderScheme::Cropped => UsedParts::cropped_bounds(canvas_size, image_size),
                ImageRenderScheme::FilledWithBackground | ImageRenderScheme::Filled => {
                    UsedParts::filled_bounds(canvas_size, image_size, true)
                }
                ImageRenderScheme::ShrunkWithBackground | ImageRenderScheme::Shrunk => {
                    UsedParts::filled_bounds(canvas_size, image_size, false)
                }
                ImageRenderScheme::CenteredWithBackground | ImageRenderScheme::Centered => {
                    UsedParts::centered_bounds(canvas_size, image_size)
                }
            },
        }
    }

    /// Creates coordinate system from application state and canvas bounds.
    ///
    /// Convenience method that extracts image dimensions from the application
    /// state and creates the appropriate coordinate transformation system.
    ///
    /// # Arguments
    ///
    /// * `app_state` - Current application state containing image data
    /// * `canvas_bounds` - Canvas position and size
    ///
    /// # Returns
    ///
    /// - `Some(ImageInCanvas)` if image data is available
    /// - `None` if no image is currently loaded
    pub fn for_app_state_and_bounds(
        app_state: &AppState,
        canvas_bounds: Rectangle,
    ) -> Option<Self> {
        if let Some(storage) = &app_state.storage {
            Some(ImageInCanvas::init(
                canvas_bounds,
                Size::new(storage.stage.width() as f32, storage.stage.height() as f32),
                app_state.viz.render_scheme,
            ))
        } else {
            None
        }
    }

    /// Converts mouse screen coordinates to fractal image coordinates.
    ///
    /// Performs the complete coordinate transformation from mouse position
    /// (in application window coordinates) to the corresponding pixel position
    /// in the fractal image, accounting for canvas positioning, scaling, and
    /// rendering scheme adjustments.
    ///
    /// # Transformation Steps
    ///
    /// 1. **Screen → Canvas**: Subtract canvas offset
    /// 2. **Canvas → Used Canvas**: Account for unused canvas areas
    /// 3. **Used Canvas → Used Image**: Apply scaling transformation
    /// 4. **Used Image → Full Image**: Add image region offset
    ///
    /// # Arguments
    ///
    /// * `mouse_on_screen` - Mouse position in window coordinates
    ///
    /// # Returns
    ///
    /// Corresponding position in fractal image coordinates (may be outside image bounds)
    ///
    /// # Usage
    ///
    /// Essential for:
    /// - Processing mouse clicks for zoom operations
    /// - Converting drag distances to pixel offsets
    /// - Interactive coordinate display
    pub fn mouse_to_image(&self, mouse_on_screen: Point) -> Point {
        // Convert app-global coordinates to canvas-origin
        let mouse_on_canvas = Point::new(
            mouse_on_screen.x - self.canvas_bounds.x,
            mouse_on_screen.y - self.canvas_bounds.y,
        );
        // Convert from canvas to actually used canvas
        let mouse_on_used_canvas = Point::new(
            mouse_on_canvas.x - self.used_parts.used_canvas_part.x,
            mouse_on_canvas.y - self.used_parts.used_canvas_part.y,
        );
        // Convert from on-screen pixels to the pixels of the - potentially scaled - image
        Point::new(
            self.used_parts.used_image_part.x
                + (mouse_on_used_canvas.x * self.used_parts.used_image_part.width
                    / self.used_parts.used_canvas_part.width),
            self.used_parts.used_image_part.y
                + (mouse_on_used_canvas.y * self.used_parts.used_image_part.height
                    / self.used_parts.used_canvas_part.height),
        )
    }

    /// Converts mouse coordinates to image coordinates with bounds checking.
    ///
    /// Like `mouse_to_image()` but returns `None` if the mouse position
    /// doesn't correspond to a valid pixel within the fractal image bounds.
    ///
    /// # Arguments
    ///
    /// * `mouse_on_screen` - Mouse position in window coordinates
    ///
    /// # Returns
    ///
    /// - `Some(Point)` if mouse is over the rendered fractal image
    /// - `None` if mouse is outside the image area
    ///
    /// # Usage
    ///
    /// - Validating interaction targets
    /// - Preventing invalid zoom origins
    /// - UI feedback about cursor location
    pub fn mouse_to_image_if_valid(&self, mouse_on_screen: Point) -> Option<Point> {
        Some(self.mouse_to_image(mouse_on_screen)).filter(|p| {
            p.x >= 0.0
                && p.x <= self.image_size.width
                && p.y >= 0.0
                && p.y <= self.image_size.height
        })
    }
}

impl UsedParts {
    /// Calculates regions for cropped rendering (image fills canvas).
    ///
    /// The image is scaled to completely fill the canvas, with excess portions
    /// cropped if aspect ratios don't match. The entire canvas is used, but
    /// only part of the image may be visible.
    ///
    /// # Algorithm
    ///
    /// - Compares aspect ratios to determine limiting dimension
    /// - Crops the center portion of the image that matches canvas aspect
    /// - Scales the cropped portion to fill the entire canvas
    ///
    /// # Arguments
    ///
    /// * `canvas_size` - Available canvas dimensions
    /// * `image_size` - Source image dimensions
    ///
    /// # Returns
    ///
    /// `UsedParts` with full canvas and center-cropped image region
    pub fn cropped_bounds(canvas_size: Size<f32>, image_size: Size<f32>) -> Self {
        let used_canvas_part = Rectangle::new(Point::new(0.0, 0.0), canvas_size);
        let canvas_aspect_ratio = canvas_size.width / canvas_size.height;
        let image_aspect_ratio = image_size.width / image_size.height;
        if image_aspect_ratio < canvas_aspect_ratio {
            // image narrower than canvas, takes all image width, mid of image height
            let new_image_height = image_size.width / canvas_aspect_ratio;
            let new_image_top = (image_size.height - new_image_height).max(0.0) / 2.0;
            UsedParts {
                used_image_part: Rectangle::new(
                    Point::new(0.0, new_image_top),
                    Size::new(image_size.width, new_image_height),
                ),
                used_canvas_part,
            }
        } else {
            // image wider than canvas, takes all image height, mid of image width
            let new_image_width = image_size.height * canvas_aspect_ratio;
            let new_image_left = (image_size.width - new_image_width).max(0.0) / 2.0;
            UsedParts {
                used_image_part: Rectangle::new(
                    Point::new(new_image_left, 0.0),
                    Size::new(new_image_width, image_size.height),
                ),
                used_canvas_part,
            }
        }
    }
    /// Calculates regions for fitted rendering (entire image visible).
    ///
    /// The complete image is scaled to fit within the canvas, potentially
    /// leaving unused canvas areas if aspect ratios don't match. No image
    /// content is cropped.
    ///
    /// # Algorithm
    ///
    /// - Determines maximum scale factor that keeps entire image visible
    /// - Optionally restricts scaling to prevent upscaling beyond 1:1
    /// - Centers the scaled image within the available canvas
    ///
    /// # Arguments
    ///
    /// * `canvas_size` - Available canvas dimensions
    /// * `image_size` - Source image dimensions
    /// * `upscale` - Whether to allow scaling beyond 1:1 ratio
    ///
    /// # Returns
    ///
    /// `UsedParts` with complete image and centered canvas region
    fn filled_bounds(canvas_size: Size<f32>, image_size: Size<f32>, upscale: bool) -> Self {
        let used_image_part = Rectangle::new(Point::new(0.0, 0.0), image_size);

        let canvas_by_stage = Size::new(
            canvas_size.width / image_size.width,
            canvas_size.height / image_size.height,
        );

        let mut scale_min = canvas_by_stage.width.min(canvas_by_stage.height);
        if !upscale {
            scale_min = scale_min.min(1.0);
        }

        let used_canvas_size =
            Size::new(image_size.width * scale_min, image_size.height * scale_min);

        UsedParts {
            used_image_part,
            used_canvas_part: Rectangle::new(
                Point::new(
                    ((canvas_size.width - used_canvas_size.width) / 2.0).max(0.0),
                    ((canvas_size.height - used_canvas_size.height) / 2.0).max(0.0),
                ),
                used_canvas_size,
            ),
        }
    }
    /// Calculates regions for centered rendering (1:1 pixel ratio).
    ///
    /// The image is displayed at its natural 1:1 pixel scale, centered
    /// within the canvas. If the image is larger than the canvas, only
    /// the center portion is visible. If smaller, it's surrounded by
    /// empty canvas area.
    ///
    /// # Algorithm
    ///
    /// - No scaling applied (1:1 pixel correspondence)
    /// - Centers image within canvas for both dimensions
    /// - Crops image or canvas as needed to fit available space
    ///
    /// # Arguments
    ///
    /// * `canvas_size` - Available canvas dimensions
    /// * `image_size` - Source image dimensions
    ///
    /// # Returns
    ///
    /// `UsedParts` with 1:1 scaling and centered positioning
    fn centered_bounds(canvas_size: Size<f32>, image_size: Size<f32>) -> Self {
        let (image_left, image_width, canvas_left, canvas_width) =
            if image_size.width <= canvas_size.width {
                (
                    0.0,
                    image_size.width,
                    (canvas_size.width - image_size.width) / 2.0,
                    image_size.width,
                )
            } else {
                (
                    (image_size.width - canvas_size.width) / 2.0,
                    canvas_size.width,
                    0.0,
                    canvas_size.width,
                )
            };
        let (image_top, image_height, canvas_top, canvas_height) =
            if image_size.height <= canvas_size.height {
                (
                    0.0,
                    image_size.height,
                    (canvas_size.height - image_size.height) / 2.0,
                    image_size.height,
                )
            } else {
                (
                    (image_size.height - canvas_size.height) / 2.0,
                    canvas_size.height,
                    0.0,
                    canvas_size.height,
                )
            };
        UsedParts {
            used_image_part: Rectangle::new(
                Point::new(image_left, image_top),
                Size::new(image_width, image_height),
            ),
            used_canvas_part: Rectangle::new(
                Point::new(canvas_left, canvas_top),
                Size::new(canvas_width, canvas_height),
            ),
        }
    }
}

/// Current interactive operation state of the canvas.
///
/// Tracks the active user interaction to properly handle mouse events
/// and provide appropriate visual feedback during operations.
///
/// # State Transitions
///
/// ```text
/// Idle ←→ Drag
/// ```
///
/// - **Idle**: Ready for new interactions
/// - **Drag**: Active panning operation in progress
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanvasOperation {
    /// No active interaction - ready for mouse input
    Idle,
    /// Panning operation in progress - tracking mouse movement
    Drag,
}

/// State tracking for canvas interactive operations.
///
/// Maintains the current state of user interactions including active
/// operations, reference points, and temporary visual adjustments.
/// Used by the Iced canvas system to track state between events.
///
/// # Lifecycle
///
/// - **Idle**: All fields except `operation` are `None`
/// - **Drag Start**: `operation` and `start_pixel` set
/// - **Drag Progress**: `drag_shift` updated with current offset
/// - **Drag End**: Reset to idle, emit coordinate update message
pub struct CanvasState {
    /// Current interactive operation (Idle or Drag)
    operation: CanvasOperation,
    /// Starting image coordinate for drag operations
    start_pixel: Option<Point>,
    /// Current visual shift offset during drag preview
    drag_shift: Option<Size>,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            operation: CanvasOperation::Idle,
            start_pixel: None,
            drag_shift: None,
        }
    }
}

/// Interactive fractal visualization canvas with pan/zoom navigation.
///
/// Implements the Iced `canvas::Program` trait to provide a high-performance
/// interactive canvas for fractal visualization. Handles real-time rendering,
/// mouse interactions, and coordinate transformations.
///
/// # Features
///
/// ## Rendering
/// - **Pixel Generation**: Converts fractal data to RGBA pixels
/// - **Color Mapping**: Applies gradient color schemes to iteration data
/// - **Interpolation**: Estimates uncomputed pixels from neighbors
/// - **Multiple Schemes**: Supports various aspect ratio handling modes
///
/// ## Interaction
/// - **Panning**: Mouse drag to shift the viewed area
/// - **Zooming**: Mouse wheel to zoom in/out around cursor
/// - **Real-time Preview**: Visual feedback during operations
///
/// ## Performance
/// - **Canvas Caching**: Leverages Iced's built-in caching system
/// - **Efficient Updates**: Only redraws when necessary
/// - **Memory Management**: Optimized pixel buffer operations
///
/// # Usage
///
/// This canvas is the central interactive component of the fractal viewer,
/// connecting user input to mathematical coordinate transformations.
pub struct FractalCanvas<'a> {
    /// Reference to current application state (fractal data, settings, etc.)
    pub app_state: &'a AppState,
}

impl<'a> FractalCanvas<'a> {
    /// Creates a new fractal canvas with the given application state.
    ///
    /// # Arguments
    ///
    /// * `app_state` - Reference to current application state
    ///
    /// # Returns
    ///
    /// Canvas ready for rendering and interaction
    pub fn new(app_state: &'a AppState) -> Self {
        FractalCanvas { app_state }
    }
    /// Retrieves computed fractal data for a specific pixel.
    ///
    /// # Arguments
    ///
    /// * `storage` - Visualization storage containing fractal data
    /// * `x` - Pixel X coordinate
    /// * `y` - Pixel Y coordinate
    ///
    /// # Returns
    ///
    /// - `Some(DataPoint)` if pixel has been computed
    /// - `None` if pixel is still uncomputed
    fn get_pixel(&self, storage: &'a VizStorage, x: usize, y: usize) -> Option<&'a DataPoint> {
        storage.stage.get(x, y)
    }
    /// Estimates pixel data from nearby computed values.
    ///
    /// Uses a progressive sampling strategy to find the nearest computed
    /// pixel and use its value as an estimate. This provides better visual
    /// continuity during progressive computation.
    ///
    /// # Algorithm
    ///
    /// 1. Start with small sampling grid (2x2)
    /// 2. Look for computed pixels at grid intersections
    /// 3. Progressively increase grid size (4x4, 8x8, ...)
    /// 4. Return first found value marked as "guessed" quality
    ///
    /// # Arguments
    ///
    /// * `storage` - Visualization storage containing fractal data
    /// * `x` - Pixel X coordinate
    /// * `y` - Pixel Y coordinate
    ///
    /// # Returns
    ///
    /// - `Some(DataPoint)` with estimated value and `Guessed` quality
    /// - `None` if no nearby computed pixels found
    ///
    /// # Usage
    ///
    /// Only call this if `get_pixel()` returned `None`. Provides better
    /// visual appearance during progressive fractal computation.
    fn guess_pixel(&self, storage: &VizStorage, x: usize, y: usize) -> Option<DataPoint> {
        let mut modrest = 2;
        while modrest < x || modrest < y {
            if let Some(guesspix) = storage.stage.get(x - (x % modrest), y - (y % modrest)) {
                return Some(guesspix.as_guessed());
            }
            modrest *= 2;
        }
        None
    }
    /// Converts fractal data point to RGBA pixel color.
    ///
    /// Applies the current color scheme and iteration assignment function
    /// to transform mathematical fractal data into display colors.
    ///
    /// # Arguments
    ///
    /// * `storage` - Visualization storage (for max iteration reference)
    /// * `color_scheme` - Gradient color mapping system
    /// * `point` - Fractal computation result to colorize
    ///
    /// # Returns
    ///
    /// RGBA pixel data as `[red, green, blue, alpha]` bytes
    ///
    /// # Color Mapping Process
    ///
    /// 1. Apply iteration assignment function (linear, logarithmic, etc.)
    /// 2. Map result to color gradient position
    /// 3. Extract RGBA values from gradient
    fn generate_pixel(
        &self,
        storage: &VizStorage,
        color_scheme: &GradientColors,
        point: &DataPoint,
    ) -> [u8; 4] {
        color_scheme.iteration_to_color(
            point.iteration_count,
            self.app_state
                .viz
                .iteration_assignment
                .assignment_function(),
            storage.properties.max_iteration,
        )
    }
    /// Generates the complete RGBA pixel buffer for canvas rendering.
    ///
    /// This is the core rendering method that converts the entire fractal
    /// computation state into a displayable pixel buffer. Handles computed,
    /// estimated, and uncomputed pixels with appropriate visual representation.
    ///
    /// # Rendering Pipeline
    ///
    /// For each pixel in the image:
    /// 1. **Check Computed**: Use actual fractal data if available
    /// 2. **Estimate Missing**: Use interpolation from nearby computed pixels
    /// 3. **Default Uncomputed**: Show neutral gray for completely unknown areas
    /// 4. **Apply Colors**: Convert mathematical data to visual colors
    ///
    /// # Performance Notes
    ///
    /// - Creates new color scheme instance (TODO: move to app state)
    /// - Processes pixels in row-major order for cache efficiency
    /// - Pre-allocates pixel buffer to avoid reallocations
    ///
    /// # Returns
    ///
    /// - `Some(Pixels)` with complete RGBA image data
    /// - `None` if no fractal data is currently available
    ///
    /// # Visual Quality
    ///
    /// - **Computed pixels**: Full quality with exact colors
    /// - **Estimated pixels**: Smooth approximation from neighbors
    /// - **Uncomputed pixels**: Neutral gray (128, 128, 128, 255)
    fn create_pixels(&self) -> Option<Pixels> {
        if let Some(storage) = self.app_state.storage.as_ref() {
            let width = storage.stage.width();
            let height = storage.stage.height();

            // TODO: Move color_scheme to the app_state to prevent permanent recomputation
            let color_scheme =
                GradientColors::new(&self.app_state.viz.gradient_color_preset.scheme(), 256);

            let mut pixels = Vec::with_capacity(width * height * 4);
            for y in 0..height {
                for x in 0..width {
                    if let Some(point) = self.get_pixel(storage, x, y) {
                        // computed points: handled as reference in the storage
                        pixels.extend_from_slice(&self.generate_pixel(
                            storage,
                            &color_scheme,
                            point,
                        ));
                    } else if let Some(point) = self.guess_pixel(storage, x, y) {
                        // guessed points: Have to be generated on the fly
                        pixels.extend_from_slice(&self.generate_pixel(
                            storage,
                            &color_scheme,
                            &point,
                        ));
                    } else {
                        // unknown points: A nice neutral grey…
                        let pix = 128;
                        pixels.extend_from_slice(&[pix, pix, pix, 255]);
                    }
                }
            }
            Some(Pixels::at_zero_origin(Size::new(width, height), pixels))
        } else {
            None
        }
    }
    /// Converts mouse wheel scroll events to zoom tick increments.
    ///
    /// Normalizes different types of scroll input (line-based and pixel-based)
    /// into discrete zoom ticks for consistent zoom behavior across platforms.
    ///
    /// # Arguments
    ///
    /// * `delta` - Mouse wheel scroll delta from Iced
    ///
    /// # Returns
    ///
    /// - `+1` for scroll up (zoom in)
    /// - `-1` for scroll down (zoom out)  
    /// - `0` for no significant movement
    ///
    /// # Platform Handling
    ///
    /// Handles both line-based scrolling (trackpad) and pixel-based
    /// scrolling (mouse wheel) with appropriate sensitivity.
    fn mouse_wheel_to_zoom_tick(delta: ScrollDelta) -> i32 {
        match delta {
            mouse::ScrollDelta::Lines { y, .. } => {
                if y.abs() > 1e-5 {
                    y.signum() as i32
                } else {
                    0
                }
            }
            mouse::ScrollDelta::Pixels { y, .. } => {
                if y.abs() > 1e-5 {
                    y.signum() as i32
                } else {
                    0
                }
            }
        }
    }
}

impl<'a> canvas::Program<Message> for FractalCanvas<'a> {
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        canvas_bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        let canvas_size = canvas_bounds.size();
        let geometry = self
            .app_state
            .runtime
            .canvas_cache
            .draw(renderer, canvas_size, |frame| {
                if let Some(rawpixels) = self.create_pixels() {
                    let pixels = if let Some(drag_shift) = state.drag_shift {
                        rawpixels.shift(drag_shift).unwrap_or(rawpixels)
                    } else if let Some(zoom) = &self.app_state.runtime.zoom
                        && zoom.ticks != 0
                    {
                        rawpixels.zoom(zoom).unwrap_or(rawpixels)
                    } else {
                        rawpixels
                    };
                    let render_scheme = self.app_state.viz.render_scheme;
                    let image_size = Size::new(pixels.size.width as f32, pixels.size.height as f32);
                    if render_scheme.needs_background_cropped()
                        && let None = state.start_pixel
                        && let None = self.app_state.runtime.zoom
                    {
                        let background_mgr = ImageInCanvas::init(
                            canvas_bounds,
                            image_size,
                            ImageRenderScheme::Cropped,
                        );
                        if let Some(mut background_pixels) =
                            pixels.extract_part_if_needed(background_mgr.used_parts.used_image_part)
                        {
                            background_pixels.change_alpha(0.4);
                            let image = canvas::Image::new(Handle::from_rgba(
                                background_pixels.size.width as u32,
                                background_pixels.size.height as u32,
                                background_pixels.pixels,
                            ))
                            .filter_method(iced::widget::image::FilterMethod::Linear);
                            frame.draw_image(background_mgr.used_parts.used_canvas_part, image);
                        }
                    }
                    let foreground_mgr =
                        ImageInCanvas::init(canvas_bounds, image_size, render_scheme);
                    let foreground_pixels = pixels
                        .extract_part_if_needed(foreground_mgr.used_parts.used_image_part)
                        .unwrap_or(pixels);
                    let image = canvas::Image::new(Handle::from_rgba(
                        foreground_pixels.size.width as u32,
                        foreground_pixels.size.height as u32,
                        foreground_pixels.pixels,
                    ))
                    .filter_method(iced::widget::image::FilterMethod::Linear);
                    frame.draw_image(foreground_mgr.used_parts.used_canvas_part, image);
                }
            });
        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if state.operation == CanvasOperation::Idle
                            && let Some(position) = cursor.position()
                            && let Some(point) =
                                ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                                    .and_then(|iic| iic.mouse_to_image_if_valid(position))
                        {
                            state.operation = CanvasOperation::Drag;
                            state.start_pixel = Some(point);
                            state.drag_shift = None;
                            (event::Status::Captured, None)
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        if state.operation == CanvasOperation::Drag
                            && let Some(drag_start) = state.start_pixel
                            && let Some(image_in_canvas) =
                                ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                        {
                            let point = image_in_canvas.mouse_to_image(position);
                            state.drag_shift =
                                Some(Size::new(point.x - drag_start.x, point.y - drag_start.y))
                                    .filter(|p| p.width.abs() >= 1e-2 || p.height.abs() >= 1e-2);
                            self.app_state.runtime.canvas_cache.clear();
                            (event::Status::Captured, None)
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.operation == CanvasOperation::Drag
                            && let Some(drag_start) = state.start_pixel
                        {
                            state.operation = CanvasOperation::Idle;
                            state.start_pixel = None; // In any case, dragging is ended.
                            state.drag_shift = None;
                            if let Some(position) = cursor.position()
                                && let Some(image_in_canvas) =
                                    ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                            {
                                let drag_stop = image_in_canvas.mouse_to_image(position);
                                let pixel_offset: Vector2D<i32, StageSpace> = Vector2D::new(
                                    (drag_stop.x - drag_start.x) as i32,
                                    (drag_stop.y - drag_start.y) as i32,
                                );
                                self.app_state.runtime.canvas_cache.clear();
                                (
                                    event::Status::Captured,
                                    Some(Message::ShiftStage(pixel_offset)),
                                )
                            } else {
                                (event::Status::Ignored, None)
                            }
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    mouse::Event::WheelScrolled { delta } => {
                        if self.app_state.runtime.zoom.is_none()
                            && let Some(position) = cursor.position()
                            && let Some(point) =
                                ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                                    .and_then(|iic| iic.mouse_to_image_if_valid(position))
                        {
                            let zoom_tick_sum = Self::mouse_wheel_to_zoom_tick(delta);
                            if zoom_tick_sum != 0 {
                                (
                                    event::Status::Captured,
                                    Some(Message::ZoomStart((point, zoom_tick_sum))),
                                )
                            } else {
                                (event::Status::Ignored, None)
                            }
                        } else {
                            let this_tick = Self::mouse_wheel_to_zoom_tick(delta);
                            if this_tick != 0 {
                                (event::Status::Captured, Some(Message::ZoomTick(this_tick)))
                            } else {
                                (event::Status::Ignored, None)
                            }
                        }
                    }
                    _ => (event::Status::Ignored, None),
                }
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
