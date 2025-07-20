use iced::Size;

use crate::{
    gui::iced::app::{AppState, ZoomState},
    storage::{
        data_point::DataPoint,
        visualization::{coloring::base::GradientColors, viz_storage::VizStorage},
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
pub struct Pixels {
    /// Dimensions of the pixel buffer (width × height)
    pub size: Size<usize>,
    /// RGBA pixel data in row-major order (4 bytes per pixel)
    pub pixels: Vec<u8>,
}

impl Pixels {
    /// Creates a new pixel buffer with size, and data.
    ///
    /// # Arguments
    ///
    /// * `size` - Buffer dimensions (width × height)
    /// * `pixels` - RGBA pixel data (must be `width * height * 4` bytes)
    pub fn new(size: Size<usize>, pixels: Vec<u8>) -> Self {
        Pixels { size, pixels }
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
        Self::new(new_size, new_pixels)
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
        if image_part.x.abs() as usize == 0
            && image_part.y.abs() as usize == 0
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
            Some(Pixels::new(self.size, new_pixels))
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
            Some(Pixels::new(self.size, new_pixels))
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
fn guess_pixel(storage: &VizStorage, x: usize, y: usize) -> Option<DataPoint> {
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
    app_state: &AppState,
    storage: &VizStorage,
    color_scheme: &GradientColors,
    point: &DataPoint,
) -> [u8; 4] {
    color_scheme.iteration_to_color(
        point.iteration_count,
        app_state.viz.iteration_assignment.assignment_function(),
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
pub fn create_pixels_from_app_state(app_state: &AppState) -> Option<Pixels> {
    if let Some(storage) = app_state.storage.as_ref() {
        let width = storage.stage.width();
        let height = storage.stage.height();

        // TODO: Move color_scheme to the app_state to prevent permanent recomputation
        let color_scheme = GradientColors::new(
            &app_state.viz.gradient_color_preset.scheme(),
            app_state.viz.gradient_color_stripes as usize,
            app_state.viz.gradient_color_offset as usize,
        );

        let mut pixels = Vec::with_capacity(width * height * 4);
        for y in 0..height {
            for x in 0..width {
                if let Some(point) = storage.stage.get(x, y) {
                    // computed points: handled as reference in the storage
                    pixels.extend_from_slice(&generate_pixel(
                        app_state,
                        storage,
                        &color_scheme,
                        point,
                    ));
                } else if let Some(point) = guess_pixel(storage, x, y) {
                    // guessed points: Have to be generated on the fly
                    pixels.extend_from_slice(&generate_pixel(
                        app_state,
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
        Some(Pixels::new(Size::new(width, height), pixels))
    } else {
        None
    }
}
// end of file
