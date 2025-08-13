//! Core color mapping infrastructure for gradient-based fractal visualization.
//!
//! This module provides the fundamental data structures and algorithms for
//! transforming fractal iteration counts into colors using gradient-based
//! color schemes. It handles color interpolation, gradient generation, and
//! the final mapping from mathematical iteration data to RGB pixel values.
//!
//! # Core Concepts
//!
//! ## Gradient Color Schemes
//! Define the mathematical blueprint for color gradients using:
//! - **Body Color**: Color for points that never escape (in-set points)
//! - **Anchor Colors**: Key colors that define the gradient's character
//! - **Interpolation**: Smooth transitions between anchor colors
//!
//! ## Color Interpolation
//! Uses linear interpolation in linear RGB color space for smooth transitions:
//! - **Linear RGB**: Mathematically correct interpolation space
//! - **sRGB Conversion**: Final output in standard RGB for display
//! - **Cyclic Gradients**: Seamless wrapping for infinite iteration ranges
//!
//! # Mathematical Foundation
//!
//! Color interpolation uses linear mixing in linear RGB space:
//! ```text
//! color(t) = color₁ × (1-t) + color₂ × t
//! where t ∈ [0,1] represents the interpolation factor
//! ```
//!
//! # Performance Optimization
//!
//! - **Pre-computation**: Gradients calculated once and cached
//! - **Fast Lookup**: O(1) color retrieval during rendering
//! - **Memory Efficient**: Compact storage of interpolated color tables

use palette::{LinSrgb, Mix, Srgb};

/// Generates linear color interpolation between two colors for gradient creation.
///
/// Creates a smooth color transition by interpolating between two anchor colors
/// in linear RGB color space. This ensures mathematically correct color mixing
/// without the gamma-correction artifacts that occur in sRGB space.
///
/// # Arguments
///
/// * `target` - Destination vector to append interpolated colors
/// * `stripe_count` - Number of interpolation steps to generate
/// * `first_color` - Starting color of the interpolation (sRGB f32)
/// * `last_color` - Ending color of the interpolation (sRGB f32)
///
/// # Interpolation Behavior
///
/// - **Inclusive Start**: `first_color` is included in the output
/// - **Exclusive End**: `last_color` is NOT included (allows seamless gradient chains)
/// - **Linear RGB**: Interpolation performed in linear color space
/// - **Quantization**: Results converted to 8-bit sRGB for final output
///
/// # Color Space Mathematics
///
/// 1. **sRGB → Linear RGB**: Remove gamma correction for proper interpolation
/// 2. **Linear Interpolation**: Mathematical mixing in linear space
/// 3. **Linear RGB → sRGB**: Apply gamma correction for display
///
/// # Performance
///
/// O(stripe_count) with efficient color space conversions using the palette crate.
///
/// # Usage in Gradient Generation
///
/// Called repeatedly to build complete gradients by chaining interpolations
/// between consecutive anchor colors in a color scheme.
fn push_interpolation_part(
    target: &mut Vec<Srgb<u8>>,
    stripe_count: usize,
    first_color: Srgb<f32>,
    last_color: Srgb<f32>,
) {
    if stripe_count > 0 {
        let first_lin: LinSrgb = first_color.into();
        let last_lin: LinSrgb = last_color.into();
        for stripe in 0..stripe_count {
            let ratio = stripe as f32 / stripe_count as f32;
            let stripe_lin = first_lin.mix(last_lin, ratio);
            target.push(stripe_lin.into());
        }
    }
}

/// Mathematical definition of a gradient color scheme for fractal visualization.
///
/// Defines the abstract specification of how colors should transition across
/// the iteration space of fractal computation. This is the "blueprint" that
/// gets converted into concrete color lookup tables by `GradientColors`.
///
/// # Color Scheme Components
///
/// ## Body Color
/// Special color for points that never escape the fractal iteration:
/// - Used when `iteration_count == max_iterations`
/// - Typically black or another contrasting color
/// - Represents the "interior" of the fractal set
///
/// ## Anchor Colors
/// Key colors that define the gradient's character:
/// - **Sequential**: Colors are interpolated in order
/// - **Cyclic**: Last color interpolates back to first for seamless wrapping
/// - **Aesthetic**: Chosen for visual appeal and mathematical insight
///
/// # Design Philosophy
///
/// - **Separation of Concerns**: Abstract scheme separate from concrete implementation
/// - **Reusable**: One scheme can generate multiple gradients with different resolutions
/// - **Flexible**: Easy to modify anchor colors without changing interpolation logic
///
/// # Mathematical Properties
///
/// - **Continuous**: Smooth transitions between all anchor points
/// - **Periodic**: Cyclic behavior for infinite iteration ranges
/// - **Deterministic**: Same scheme always produces identical results
pub struct GradientColorScheme {
    /// Color for points that never escape (iteration = max_iterations)
    body_color: Srgb<f32>,
    /// Sequence of colors defining the gradient character
    anchor_colors: Vec<Srgb<f32>>,
}

impl GradientColorScheme {
    /// Creates a new gradient color scheme.
    ///
    /// # Arguments
    ///
    /// * `body_color` - Color for points that never escape (max iterations)
    /// * `anchor_colors` - Sequence of colors defining the gradient progression
    ///
    /// # Returns
    ///
    /// New gradient scheme ready for conversion to concrete color tables
    ///
    /// # Design Notes
    ///
    /// - **Body Color**: Usually black (0,0,0) for traditional fractal appearance
    /// - **Anchor Colors**: Should have sufficient contrast for visual appeal
    /// - **Color Count**: More anchors provide finer gradient control
    pub fn new(body_color: Srgb<f32>, anchor_colors: Vec<Srgb<f32>>) -> Self {
        GradientColorScheme {
            body_color,
            anchor_colors,
        }
    }
    /// Converts the abstract color scheme into a concrete interpolated color table.
    ///
    /// Generates a lookup table of colors by interpolating between anchor colors,
    /// creating smooth transitions suitable for fast iteration-to-color mapping.
    /// The resulting table provides O(1) color lookup during rendering.
    ///
    /// # Arguments
    ///
    /// * `stripe_count` - Number of discrete colors in the final lookup table
    ///
    /// # Returns
    ///
    /// Vector of 8-bit sRGB colors ready for pixel rendering
    ///
    /// # Interpolation Algorithm
    ///
    /// 1. **Anchor Distribution**: Anchor colors spread evenly across stripe range
    /// 2. **Segment Interpolation**: Linear interpolation between consecutive anchors
    /// 3. **Cyclic Completion**: Final segment interpolates from last to first anchor
    /// 4. **Quantization**: Convert to 8-bit sRGB for final output
    ///
    /// # Mathematical Approach
    ///
    /// ```text
    /// For n anchor colors and s stripes:
    /// - Anchor positions: 0, s/n, 2s/n, ..., (n-1)s/n, s
    /// - Each segment gets ⌊s/n⌋ interpolation steps
    /// - Remainder distributed across segments
    /// ```
    ///
    /// # Performance
    ///
    /// O(stripe_count) time complexity with efficient color space conversions.
    fn create_interpolation(&self, stripe_count: usize) -> Vec<Srgb<u8>> {
        let mut target = Vec::with_capacity(stripe_count);
        let anchor_count = self.anchor_colors.len();
        let mut anchor_stripe = Vec::with_capacity(anchor_count + 1);
        let stripe_factor = stripe_count as f32 / anchor_count as f32;
        anchor_stripe.push(0);
        for i in 1..anchor_count {
            anchor_stripe.push((stripe_factor * i as f32) as usize);
        }
        anchor_stripe.push(stripe_count);
        for i in 0..anchor_count {
            push_interpolation_part(
                &mut target,
                anchor_stripe[i + 1] - anchor_stripe[i],
                self.anchor_colors[i],
                self.anchor_colors[(i + 1) % anchor_count],
            );
        }
        target
    }
}

/// Concrete color lookup table for fast iteration-to-color mapping.
///
/// Provides the runtime color mapping system that converts fractal iteration
/// counts into RGB colors using pre-computed interpolation tables. This is
/// the "compiled" version of a `GradientColorScheme` optimized for fast
/// pixel-by-pixel color lookup during visualization.
///
/// # Performance Design
///
/// - **Pre-computed Table**: All colors calculated once during initialization
/// - **O(1) Lookup**: Direct array indexing for color retrieval
/// - **Cache Friendly**: Compact memory layout for efficient access
/// - **8-bit Colors**: Ready for direct use in RGB image buffers
///
/// # Color Mapping Strategy
///
/// ## Escaped Points (iteration < max_iterations)
/// 1. **Assignment Function**: Mathematical transformation of iteration count
/// 2. **Modulo Wrapping**: Handle values exceeding stripe table length
/// 3. **Table Lookup**: Direct indexing into pre-computed color array
///
/// ## Non-escaped Points (iteration = max_iterations)
/// - **Body Color**: Special color for points that never escape
/// - **Set Membership**: Represents the "interior" of the fractal
///
/// # Memory Efficiency
///
/// - **Compact Storage**: Single vector of 8-bit RGB values
/// - **No Runtime Calculation**: All interpolation done at initialization
/// - **Minimal Overhead**: Direct mapping without additional data structures
pub struct GradientColors {
    /// Color for points that never escape (iteration = max_iterations)
    body_color: Srgb<u8>,
    /// Pre-computed color lookup table for escaped points
    stripes: Vec<Srgb<u8>>,
    /// Offset when applying color
    offset: usize,
}

impl GradientColors {
    /// Creates a concrete color table from an abstract gradient scheme.
    ///
    /// Converts a `GradientColorScheme` into a fast lookup table optimized
    /// for real-time color mapping during fractal visualization.
    ///
    /// # Arguments
    ///
    /// * `scheme` - Abstract gradient definition with body and anchor colors
    /// * `stripe_count` - Size of the color lookup table (higher = smoother gradients)
    ///
    /// # Returns
    ///
    /// Optimized color mapper ready for iteration-to-color conversion
    ///
    /// # Stripe Count Considerations
    ///
    /// - **Higher Values**: Smoother gradients, more memory usage
    /// - **Lower Values**: More banded appearance, less memory
    /// - **Typical Range**: 256-4096 stripes provide good balance
    /// - **Power of 2**: Often preferred for efficient modulo operations
    ///
    /// # Performance Impact
    ///
    /// Initialization: O(stripe_count) for interpolation calculation
    /// Runtime: O(1) for each color lookup
    pub fn new(scheme: &GradientColorScheme, stripe_count: usize, offset: usize) -> Self {
        GradientColors {
            body_color: scheme.body_color.into(),
            stripes: scheme.create_interpolation(stripe_count),
            offset,
        }
    }

    /// Converts sRGB color to RGBA byte array for image rendering.
    ///
    /// Transforms palette crate color format into the 4-byte RGBA format
    /// commonly used by image rendering libraries and GPU textures.
    ///
    /// # Arguments
    ///
    /// * `color` - 8-bit sRGB color from palette crate
    ///
    /// # Returns
    ///
    /// 4-byte array: [red, green, blue, alpha] with alpha = 255 (opaque)
    ///
    /// # Format Details
    ///
    /// - **Channel Order**: RGBA (red, green, blue, alpha)
    /// - **Alpha Value**: Always 255 (fully opaque)
    /// - **Bit Depth**: 8 bits per channel (0-255 range)
    /// - **Color Space**: sRGB (standard display color space)
    fn rgb_to_u84(color: &Srgb<u8>) -> [u8; 4] {
        [color.red, color.green, color.blue, 255]
    }

    /// Converts fractal iteration count to RGBA color using mathematical assignment.
    ///
    /// This is the core color mapping function that transforms raw fractal
    /// computation results into visually appealing colors. It handles both
    /// escaped points (finite iterations) and non-escaped points (max iterations).
    ///
    /// # Arguments
    ///
    /// * `it` - Iteration count from fractal computation
    /// * `assigner` - Mathematical function to transform iteration count
    /// * `maxit` - Maximum iteration limit used in fractal computation
    ///
    /// # Returns
    ///
    /// 4-byte RGBA color array ready for pixel rendering
    ///
    /// # Color Mapping Logic
    ///
    /// ## Non-escaped Points (it == maxit)
    /// - **Body Color**: Points that never escaped the iteration loop
    /// - **Set Membership**: Represents the "interior" of the fractal
    /// - **Typical Color**: Black or other contrasting color
    ///
    /// ## Escaped Points (it < maxit)
    /// 1. **Assignment Function**: Apply mathematical transformation to iteration
    /// 2. **Modulo Wrapping**: Handle values exceeding stripe table length
    /// 3. **Table Lookup**: Direct indexing into pre-computed gradient
    ///
    /// # Assignment Function Examples
    ///
    /// - **Linear**: `assigner(it, _) = it` (direct mapping)
    /// - **Logarithmic**: `assigner(it, _) = ln(it)` (compress high values)
    /// - **Power**: `assigner(it, _) = it²` (emphasize low values)
    /// - **Modulo**: `assigner(it, modulo) = it % modulo` (periodic patterns)
    ///
    /// # Performance
    ///
    /// O(1) operation with direct array lookup after assignment calculation.
    pub fn iteration_to_color(
        &self,
        it: u32,
        assigner: fn(u32, u32) -> u32,
        maxit: u32,
    ) -> [u8; 4] {
        if it == maxit || self.stripes.len() < 1 {
            Self::rgb_to_u84(&self.body_color)
        } else {
            Self::rgb_to_u84(
                &self.stripes[(assigner(it, self.stripes.len() as u32) as usize)
                    .wrapping_add(self.offset)
                    % self.stripes.len()],
            )
        }
    }
}

// end of file
