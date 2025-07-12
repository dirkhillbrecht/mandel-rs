//! Pre-defined color schemes and mathematical assignment functions for fractal visualization.
//!
//! This module provides a carefully curated collection of color schemes and
//! mathematical transformation functions designed to reveal different aspects
//! of fractal structures. Each preset has been chosen for both aesthetic appeal
//! and mathematical insight into fractal behavior.
//!
//! # Color Scheme Design Philosophy
//!
//! ## Aesthetic Considerations
//! - **Visual Appeal**: Colors chosen for pleasing combinations
//! - **Contrast**: Sufficient differentiation between iteration levels
//! - **Cultural Themes**: Nature-inspired palettes (Sunrise, Woods, Moonlight)
//! - **Accessibility**: High contrast options (Gray) for visibility
//!
//! ## Mathematical Insight
//! Different color schemes can highlight different fractal properties:
//! - **Smooth Gradients**: Reveal continuous fractal boundaries
//! - **High Contrast**: Emphasize discrete iteration bands
//! - **Cyclic Patterns**: Show periodic behavior in iteration space
//!
//! # Assignment Function Categories
//!
//! ## Power Functions
//! - **Squared (x²)**: Emphasizes low iteration counts, compresses high values
//! - **Cubic (x³)**: Even stronger emphasis on early escape patterns
//!
//! ## Root Functions  
//! - **Square Root (√x)**: Expands low values, compresses high iteration detail
//! - **Cube Root (∛x)**: Gentler compression for more uniform distribution
//!
//! ## Logarithmic Functions
//! - **Natural Log (ln x)**: Handles extreme iteration ranges effectively
//! - **Double Log (ln ln x)**: For datasets with very large iteration counts
//!
//! ## Linear Function
//! - **Identity (x)**: Direct mapping preserving original iteration relationships
//!
//! # Usage Patterns
//!
//! Color schemes and assignment functions are typically used together:
//! ```rust
//! let scheme = GradientColorPreset::Sunrise.scheme();
//! let assignment = IterationAssignment::Logarithmic.assignment_function();
//! let colors = GradientColors::new(&scheme, 1024);
//! let pixel_color = colors.iteration_to_color(iterations, assignment, max_iter);
//! ```

use palette::Srgb;

use crate::storage::visualization::coloring::base::GradientColorScheme;

/// Enumeration of pre-defined gradient color schemes for fractal visualization.
///
/// Provides a curated collection of color palettes designed for aesthetic appeal
/// and mathematical insight. Each preset defines a complete gradient scheme with
/// carefully chosen anchor colors and appropriate body colors for non-escaped points.
///
/// # Color Scheme Categories
///
/// ## Nature-Inspired Themes
/// - **Sunrise**: Warm progression from deep blue through white to orange/red
/// - **Woods**: Earth tones with greens, yellows, and natural browns
/// - **Moonlight**: Cool palette with purples, blues, and silver highlights
///
/// ## Utility Schemes
/// - **Gray**: High-contrast monochrome for accessibility and analysis
/// - **UglyColors**: Deliberately harsh colors for testing and debugging
///
/// # Design Principles
///
/// - **Progressive Transitions**: Smooth color flow for continuous visualization
/// - **Sufficient Contrast**: Clear differentiation between iteration levels
/// - **Mathematical Relevance**: Colors that enhance fractal pattern visibility
/// - **Cross-Platform Consistency**: Colors specified in device-independent sRGB
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GradientColorPreset {
    /// Dawn/dusk color progression: deep blue → white → yellow → red
    Sunrise,
    /// Natural earth tones: green → yellow → cyan → light gray
    Woods,
    /// Cool nighttime palette: gray → purple → yellow → blue
    Moonlight,
    /// Simple monochrome gradient: black → white
    Gray,
    /// High-contrast test colors: red → purple → cyan → blue → white
    UglyColors,
}

impl GradientColorPreset {
    /// Returns all available gradient color presets.
    ///
    /// Provides a complete list of pre-defined color schemes for UI enumeration,
    /// testing, and programmatic access to all available options.
    ///
    /// # Returns
    ///
    /// Static slice containing all gradient color preset variants
    ///
    /// # Usage
    ///
    /// Commonly used for populating UI dropdowns and iterating through
    /// all available color schemes:
    ///
    /// ```rust
    /// // UI dropdown population
    /// for preset in GradientColorPreset::all() {
    ///     dropdown.add_option(preset.name(), *preset);
    /// }
    /// ```
    pub fn all() -> &'static [Self] {
        &[
            Self::Sunrise,
            Self::Woods,
            Self::Moonlight,
            Self::Gray,
            Self::UglyColors,
        ]
    }
    /// Returns the human-readable name of the color preset.
    ///
    /// Provides user-friendly names suitable for display in UI elements
    /// such as dropdown menus, tooltips, and configuration dialogs.
    ///
    /// # Returns
    ///
    /// Static string with the preset's display name
    ///
    /// # Naming Convention
    ///
    /// Names are chosen to be:
    /// - **Descriptive**: Suggest the color scheme's character
    /// - **Concise**: Suitable for UI space constraints
    /// - **Intuitive**: Match user expectations for the colors
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sunrise => "Sunrise",
            Self::Woods => "Woods",
            Self::Moonlight => "Moonlight",
            Self::Gray => "Gray",
            Self::UglyColors => "Ugly Colors",
        }
    }
    /// Converts the preset into a concrete gradient color scheme.
    ///
    /// Creates the actual `GradientColorScheme` with specific color values
    /// for the preset. This scheme can then be used to generate color lookup
    /// tables for fractal visualization.
    ///
    /// # Returns
    ///
    /// `GradientColorScheme` with body color and anchor colors defined
    ///
    /// # Color Value Format
    ///
    /// All colors specified as `Srgb<f32>` with components in [0.0, 1.0] range:
    /// - **0.0**: Minimum intensity (black)
    /// - **1.0**: Maximum intensity (full color)
    /// - **sRGB**: Standard RGB color space for consistent display
    ///
    /// # Color Scheme Details
    ///
    /// Each preset includes carefully chosen anchor colors to create
    /// visually appealing and mathematically useful gradients.
    pub fn scheme(&self) -> GradientColorScheme {
        match self {
            // Sunrise: Deep blue dawn → bright white → golden yellow → deep red sunset
            Self::Sunrise => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0), // Black body color for non-escaped points
                vec![
                    Srgb::new(0.0, 0.2, 0.7),   // Deep blue (early dawn)
                    Srgb::new(1.0, 1.0, 1.0),   // Bright white (midday sun)
                    Srgb::new(1.0, 1.0, 0.2),   // Golden yellow (late afternoon)
                    Srgb::new(0.8, 0.05, 0.0),  // Deep red (sunset)
                ],
            ),
            // Woods: Forest green → autumn yellow → stream cyan → weathered gray
            Self::Woods => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0), // Black body color
                vec![
                    Srgb::new(59.0 / 255.0, 216.0 / 255.0, 17.0 / 255.0),   // Vibrant green (leaves)
                    Srgb::new(215.0 / 255.0, 179.0 / 255.0, 24.0 / 255.0),  // Autumn yellow (foliage)
                    Srgb::new(83.0 / 255.0, 209.0 / 255.0, 218.0 / 255.0),  // Stream cyan (water)
                    Srgb::new(212.0 / 255.0, 212.0 / 255.0, 212.0 / 255.0), // Weathered gray (stone)
                ],
            ),
            // Moonlight: Dusky gray → mystical purple → lunar yellow → starlight blue
            Self::Moonlight => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0), // Black body color
                vec![
                    Srgb::new(103.0 / 255.0, 103.0 / 255.0, 103.0 / 255.0), // Dusky gray (twilight)
                    Srgb::new(166.0 / 255.0, 67.0 / 255.0, 167.0 / 255.0),  // Mystical purple (night)
                    Srgb::new(255.0 / 255.0, 252.0 / 255.0, 0.0 / 255.0),   // Lunar yellow (moon)
                    Srgb::new(111.0 / 255.0, 176.0 / 255.0, 255.0 / 255.0), // Starlight blue (sky)
                ],
            ),
            // Gray: Simple monochrome gradient for high contrast and accessibility
            Self::Gray => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0), // Black body color
                vec![
                    Srgb::new(0.0, 0.0, 0.0), // Pure black (start)
                    Srgb::new(1.0, 1.0, 1.0), // Pure white (end)
                ],
            ),
            // UglyColors: Deliberately harsh colors for testing and debugging
            Self::UglyColors => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0), // Black body color
                vec![
                    Srgb::new(1.0, 0.0, 0.0),     // Pure red (maximum contrast)
                    Srgb::new(0.875, 0.0, 0.375), // Magenta-red (harsh transition)
                    Srgb::new(0.0, 0.5, 0.5),     // Teal (contrasting hue)
                    Srgb::new(0.0, 0.0, 1.0),     // Pure blue (opposite spectrum)
                    Srgb::new(1.0, 1.0, 1.0),     // Pure white (maximum brightness)
                ],
            ),
        }
    }
}

impl std::fmt::Display for GradientColorPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Mathematical functions for transforming iteration counts into color indices.
///
/// Provides various mathematical transformations that can be applied to fractal
/// iteration counts before mapping them to colors. Different assignment functions
/// reveal different aspects of fractal structure and can dramatically change
/// the visual appearance of the same mathematical data.
///
/// # Mathematical Categories
///
/// ## Power Functions (Emphasis)
/// Functions that raise the iteration to a power, emphasizing different ranges:
/// - **Cubic (x³)**: Extreme emphasis on low iterations
/// - **Squared (x²)**: Strong emphasis on low iterations
///
/// ## Root Functions (Compression)
/// Functions that take roots, compressing high values:
/// - **Square Root (√x)**: Gentle compression of high iterations
/// - **Cube Root (∛x)**: Moderate compression across the range
///
/// ## Logarithmic Functions (Extreme Range)
/// Functions for handling very large iteration ranges:
/// - **Logarithmic (ln x)**: Compresses exponential growth patterns
/// - **Double Log (ln ln x)**: For extremely large iteration counts
///
/// ## Identity Function (Baseline)
/// - **Linear (x)**: Direct mapping preserving original relationships
///
/// # Visual Effects
///
/// - **Power Functions**: Create "focused" views emphasizing escape boundaries
/// - **Root Functions**: Create "expanded" views showing more iteration detail
/// - **Logarithmic**: Handle fractals with very high iteration counts
/// - **Linear**: Preserve the mathematical relationships in the raw data
///
/// # Usage Strategy
///
/// Different functions work better with different fractal types and zoom levels:
/// - **Deep Zooms**: Logarithmic functions handle extreme iteration ranges
/// - **Boundary Details**: Root functions reveal fine escape patterns
/// - **Overview Images**: Linear or squared functions provide clear structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IterationAssignment {
    /// x → x³ - Extreme emphasis on low iteration counts
    Cubic,
    /// x → x² - Strong emphasis on low iteration counts  
    Squared,
    /// x → x - Direct linear mapping (identity function)
    Linear,
    /// x → √x - Gentle compression emphasizing high iteration detail
    SquareRoot,
    /// x → ∛x - Moderate compression across iteration range
    CubicRoot,
    /// x → ln(x) - Logarithmic compression for extreme ranges
    Logarithmic,
    /// x → ln(ln(x)) - Double logarithmic for very large iteration counts
    LogLog,
}

impl IterationAssignment {
    /// Returns all available iteration assignment functions.
    ///
    /// Provides a complete list of mathematical transformation functions
    /// for UI enumeration and programmatic access.
    ///
    /// # Returns
    ///
    /// Static slice containing all assignment function variants
    ///
    /// # Ordering
    ///
    /// Functions are ordered from most emphasis (Cubic) to most compression (LogLog),
    /// providing a logical progression for UI presentation.
    pub fn all() -> &'static [Self] {
        &[
            Self::Cubic,
            Self::Squared,
            Self::Linear,
            Self::SquareRoot,
            Self::CubicRoot,
            Self::Logarithmic,
            Self::LogLog,
        ]
    }
    /// Returns the mathematical notation and description for the assignment function.
    ///
    /// Provides user-friendly mathematical notation suitable for display in UI
    /// elements. Each name includes both the mathematical formula and a
    /// descriptive term for clarity.
    ///
    /// # Returns
    ///
    /// Static string with mathematical notation and descriptive name
    ///
    /// # Format
    ///
    /// Names follow the pattern "x → f(x) (description)" where:
    /// - **x → f(x)**: Mathematical transformation notation
    /// - **(description)**: Human-readable function type
    ///
    /// # Mathematical Notation
    ///
    /// Uses standard mathematical symbols for clarity:
    /// - **√x**: Square root (not sqrt(x))
    /// - **∛x**: Cube root (not cbrt(x))  
    /// - **ln(x)**: Natural logarithm (not log(x))
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cubic => "x → x³ (cubic)",
            Self::Squared => "x → x² (squared)",
            Self::Linear => "x → x (linear)",
            Self::SquareRoot => "x → √x (square root)",
            Self::CubicRoot => "x → ∛x (cube root)",
            Self::Logarithmic => "x → ln(x) (logarithmic)",
            Self::LogLog => "x → ln(ln(x)) (double log)",
        }
    }
    /// Returns the actual mathematical function for iteration transformation.
    ///
    /// Provides the concrete implementation of the mathematical transformation
    /// that can be used in the color mapping pipeline. Each function takes
    /// the iteration count and modulo parameter and returns the transformed value.
    ///
    /// # Returns
    ///
    /// Function pointer with signature `fn(u32, u32) -> u32` where:
    /// - **First parameter**: Iteration count from fractal computation
    /// - **Second parameter**: Modulo value (stripe count for wrapping)
    /// - **Return value**: Transformed iteration value for color lookup
    ///
    /// # Function Implementations
    ///
    /// ## Power Functions
    /// - **Cubic**: `(it % modulo)³` - Uses modulo to prevent overflow
    /// - **Squared**: `(it % modulo)²` - Uses modulo to prevent overflow
    ///
    /// ## Root Functions
    /// - **Square Root**: `⌊√it⌋` - Ignores modulo, direct square root
    /// - **Cube Root**: `⌊it^(1/3)⌋` - Ignores modulo, direct cube root
    ///
    /// ## Logarithmic Functions
    /// - **Logarithmic**: `⌊ln(it)⌋` - Natural log, ignores modulo
    /// - **Double Log**: `⌊ln(ln(it))⌋` - Double natural log, ignores modulo
    ///
    /// ## Linear Function
    /// - **Linear**: `it` - Identity function, ignores modulo
    ///
    /// # Overflow Handling
    ///
    /// Power functions use modulo to prevent integer overflow, while other
    /// functions rely on the natural compression of their mathematical properties.
    ///
    /// # Floating Point Precision
    ///
    /// Root and logarithmic functions use f64 precision internally and
    /// truncate to u32 for final result.
    pub fn assignment_function(&self) -> fn(u32, u32) -> u32 {
        match self {
            // Power functions: Use modulo to prevent overflow
            Self::Cubic => |it, modulo| (it % modulo) * (it % modulo) * (it % modulo),
            Self::Squared => |it, modulo| (it % modulo) * (it % modulo),
            
            // Linear function: Direct mapping
            Self::Linear => |it, _modulo| it,
            
            // Root functions: Natural compression, ignore modulo
            Self::SquareRoot => |it, _modulo| (it as f64).sqrt() as u32,
            Self::CubicRoot => |it, _modulo| (it as f64).powf(1.0 / 3.0) as u32,
            
            // Logarithmic functions: Extreme compression, ignore modulo
            Self::Logarithmic => |it, _modulo| (it as f64).ln() as u32,
            Self::LogLog => |it, _modulo| (it as f64).ln().ln() as u32,
        }
    }
}

impl std::fmt::Display for IterationAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// end of file
