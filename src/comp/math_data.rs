//! Mathematical definitions and presets for fractal computation.
//!
//! This module provides the mathematical foundation for fractal computation,
//! including coordinate system definitions, iteration parameters, and pre-defined
//! regions of mathematical interest. It serves as the bridge between abstract
//! mathematical concepts and concrete computational parameters.
//!
//! # Core Concepts
//!
//! ## Mathematical Coordinate Space
//! Uses the complex plane (real × imaginary) to define fractal computation regions:
//! - **Real Axis (X)**: Horizontal coordinate in the complex plane
//! - **Imaginary Axis (Y)**: Vertical coordinate in the complex plane
//! - **Rectangular Regions**: Defined by corner points in complex space
//!
//! ## Iteration Parameters
//! Controls the depth and quality of fractal computation:
//! - **Max Iterations**: Upper limit for escape-time algorithms
//! - **Precision Trade-off**: Higher iterations → more detail, longer computation
//! - **Zoom Sensitivity**: Deeper zooms require higher iteration counts
//!
//! # Mathematical Foundation
//!
//! ## Complex Plane Mapping
//! ```text
//! Mathematical Space (Complex Plane):
//!   Imaginary
//!       ↑
//!   -1+2i │ 0+2i │ 1+2i
//!   ------+------+------
//!   -1+1i │ 0+1i │ 1+1i
//!   ------+------+------  → Real
//!   -1+0i │ 0+0i │ 1+0i
//! ```
//!
//! ## Coordinate System Properties
//! - **Type Safety**: `MathSpace` coordinate system prevents unit confusion
//! - **Precision**: f64 coordinates for mathematical accuracy
//! - **Rectangular Regions**: Axis-aligned bounding boxes in complex plane
//!
//! # Preset Philosophy
//!
//! Pre-defined mathematical regions serve multiple purposes:
//! - **Educational**: Famous fractal features for learning
//! - **Benchmarking**: Standard regions for performance testing
//! - **Artistic**: Visually striking areas for demonstration
//! - **Reference**: Well-known coordinates from fractal literature

use std::str::FromStr;

use bigdecimal::{BigDecimal, One};
use euclid::{Point2D, Rect};

use crate::{comp::math_area::MathArea, storage::coord_spaces::MathSpace};

/// Enumeration of supported fractal types for future extensibility.
///
/// Currently supports only the Mandelbrot set, but designed to accommodate
/// additional fractal types such as Julia sets, Burning Ship, and others.
/// The enum serves as a type-safe way to specify fractal algorithms.
///
/// # Future Expansion
///
/// Planned fractal types for future implementation:
/// - **Julia Sets**: Parameter-dependent fractals c = constant
/// - **Burning Ship**: abs(z) variation of Mandelbrot
/// - **Tricorn**: Complex conjugate variation
/// - **Multibrot**: Higher-power generalizations (zⁿ + c)
///
/// # Current Implementation
///
/// Only Mandelbrot is currently supported, but the architecture is designed
/// to easily accommodate additional fractal types without breaking changes.
#[allow(dead_code)]
pub enum FractalType {
    /// The classic Mandelbrot set: z(n+1) = z(n)² + c, z(0) = 0
    /// Most famous fractal with rich boundary structure and infinite detail
    Mandelbrot,
}

/// Mathematical specification for a fractal computation region.
///
/// Defines a complete mathematical description of a fractal computation,
/// including the coordinate region in the complex plane and iteration
/// parameters. This structure encapsulates all the mathematical information
/// needed to perform fractal computation.
///
/// # Components
///
/// ## Human-Readable Name
/// Descriptive label for UI display and identification:
/// - **User Interface**: Shown in dropdowns and preset lists
/// - **Debugging**: Helps identify regions during development
/// - **Documentation**: Names reference mathematical features
///
/// ## Coordinate Region
/// Rectangular area in the complex plane to compute:
/// - **Type Safety**: Uses `MathSpace` to prevent coordinate confusion
/// - **Precision**: f64 coordinates for mathematical accuracy
/// - **Rectangular**: Axis-aligned bounding box in complex plane
///
/// ## Iteration Limit
/// Maximum escape-time iterations for each point:
/// - **Quality Control**: Higher values → more detail
/// - **Performance Trade-off**: More iterations → longer computation
/// - **Zoom Dependency**: Deep zooms require higher iteration counts
///
/// # Mathematical Properties
///
/// - **Complex Plane**: Coordinates represent real + imaginary components
/// - **Rectangular Region**: Defined by opposite corners of bounding box
/// - **Iteration Threshold**: Balances detail vs computation time
///
/// # Usage Context
///
/// Typically created from presets or user input, then used to configure
/// the computation engine for fractal calculation.
pub struct MathData {
    /// Human-readable name for display and identification
    name: String,
    /// Future extensibility: fractal algorithm type selection
    // fractal_type: FractalType, // Introduce this once we can compute other fractal types
    /// Rectangular region in the complex plane to compute
    area: MathArea,
    /// Maximum iteration count for escape-time algorithm
    max_iteration: u32,
}

impl MathData {
    pub fn new(name: String, area: MathArea, max_iteration: u32) -> Self {
        MathData {
            name,
            area,
            max_iteration,
        }
    }

    /// Create new math data from fixed presets, convenience method for static data
    ///
    /// Mathematical values are given as strings so that they can have arbitrary precision.
    /// Internally, they are converted into BigDecimals.
    pub fn from_presets(
        name: String,
        center_x: &str,
        center_y: &str,
        radius: &str,
        max_iteration: u32,
    ) -> Self {
        MathData::new(
            name,
            MathArea::new(
                Point2D::new(
                    BigDecimal::from_str(center_x).unwrap(),
                    BigDecimal::from_str(center_y).unwrap(),
                ),
                BigDecimal::from_str(radius).unwrap(),
                BigDecimal::one(),
            ),
            max_iteration,
        )
    }

    /// Returns a copy of the human-readable name.
    ///
    /// Provides access to the descriptive name for UI display,
    /// logging, and debugging purposes.
    ///
    /// # Returns
    ///
    /// Owned string copy of the mathematical region's name
    ///
    /// # Usage
    ///
    /// Typically used for UI elements like dropdown labels,
    /// window titles, and debug output.
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    // Return the math area of the data
    #[allow(dead_code)]
    pub fn math_area(&self) -> &MathArea {
        &self.area
    }

    /// Returns the rectangular region in the complex plane.
    ///
    /// Provides access to the mathematical coordinate bounds that define
    /// the area of the complex plane to compute. The rectangle uses
    /// type-safe coordinates to prevent unit confusion.
    ///
    /// # Returns
    ///
    /// Euclid rectangle with f64 precision in MathSpace coordinates
    ///
    /// # Coordinate System
    ///
    /// - **Real Axis**: Horizontal (X) component of complex numbers
    /// - **Imaginary Axis**: Vertical (Y) component of complex numbers
    /// - **Rectangle**: Axis-aligned bounding box in complex plane
    ///
    /// # Usage
    ///
    /// Used by computation engine to determine pixel-to-complex-number
    /// coordinate transformations.
    pub fn coordinates(&self) -> Rect<f64, MathSpace> {
        self.area.rect_f64().unwrap()
    }
    /// Returns the maximum iteration count for escape-time computation.
    ///
    /// Provides the iteration limit used in the fractal escape-time
    /// algorithm. Points that don't escape within this limit are
    /// considered to be in the fractal set.
    ///
    /// # Returns
    ///
    /// Maximum number of iterations before considering a point non-escaping
    ///
    /// # Mathematical Significance
    ///
    /// - **Higher Values**: More detail, longer computation time
    /// - **Lower Values**: Less detail, faster computation
    /// - **Zoom Dependency**: Deep zooms require higher iteration counts
    /// - **Set Boundary**: Determines precision of fractal boundary detection
    ///
    /// # Typical Values
    ///
    /// - **Overview (1:1 scale)**: 100-500 iterations
    /// - **Detailed views**: 1000-5000 iterations
    /// - **Deep zooms**: 10,000+ iterations
    pub fn max_iteration(&self) -> u32 {
        self.max_iteration
    }
}

/// Pre-defined mathematical regions of interest in the Mandelbrot set.
///
/// Provides a curated collection of famous and visually interesting regions
/// of the Mandelbrot set, each chosen for educational value, aesthetic appeal,
/// or mathematical significance. These presets serve as starting points for
/// exploration and demonstrate the fractal's diverse structures.
///
/// # Preset Categories
///
/// ## Overview Preset
/// - **MandelbrotFull**: Complete set view showing overall structure
///
/// ## Famous Features
/// - **ElephantValley**: Classic feature resembling elephant trunks
/// - **SeahorseValley**: Intricate seahorse-like spiral structures
/// - **Spirals**: Complex spiral formations in the fractal boundary
///
/// # Mathematical Significance
///
/// Each preset represents mathematically interesting regions:
/// - **Boundary Complexity**: Areas with rich fractal boundary structure
/// - **Self-Similarity**: Regions showing fractal self-similar patterns
/// - **Fine Detail**: Areas requiring high iteration counts for full detail
///
/// # Coordinate Precision
///
/// Deep zoom presets use high-precision coordinates to accurately locate
/// tiny features in the fractal boundary. These coordinates come from
/// mathematical literature and fractal exploration.
///
/// # Educational Value
///
/// - **Progressive Exploration**: From overview to detailed features
/// - **Iteration Requirements**: Different detail levels demonstrate iteration needs
/// - **Visual Diversity**: Shows the range of fractal structures
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MathPreset {
    /// Complete Mandelbrot set overview (-2.1 to 0.75 real, -1.25 to 1.25 imaginary)
    MandelbrotFull,
    /// Famous "Elephant Valley" feature with intricate trunk-like structures
    MandelbrotElephantValley,
    /// Complex spiral formations in the fractal boundary region
    MandelbrotSpirals,
    /// "Seahorse Valley" with detailed seahorse-like spiral patterns
    MandelbrotSeahorseValley,
    // Squared spirals at a minibrot
    MandelbrotSquaredSpirals,
}

impl MathPreset {
    /// Returns all available mathematical presets.
    ///
    /// Provides a complete list of pre-defined mathematical regions for
    /// UI enumeration, programmatic access, and testing purposes.
    ///
    /// # Returns
    ///
    /// Static slice containing all mathematical preset variants
    ///
    /// # Ordering
    ///
    /// Presets are ordered from general to specific:
    /// 1. **Full**: Complete set overview
    /// 2. **Features**: Specific named mathematical features
    ///
    /// # Usage
    ///
    /// Commonly used for UI dropdown population and preset iteration:
    ///
    /// ```rust
    /// // Populate UI with all available presets
    /// for preset in MathPreset::all() {
    ///     dropdown.add_option(preset.name(), *preset);
    /// }
    /// ```
    pub fn all() -> &'static [Self] {
        &[
            Self::MandelbrotFull,
            Self::MandelbrotElephantValley,
            Self::MandelbrotSpirals,
            Self::MandelbrotSeahorseValley,
            Self::MandelbrotSquaredSpirals,
        ]
    }
    /// Returns the human-readable name of the mathematical preset.
    ///
    /// Provides descriptive names suitable for UI display, referencing
    /// either the mathematical scope (Full) or the visual characteristics
    /// of famous fractal features.
    ///
    /// # Returns
    ///
    /// Static string with the preset's display name
    ///
    /// # Naming Convention
    ///
    /// Names follow the pattern "Mandelbrot [Feature]" where:
    /// - **Mandelbrot**: Identifies the fractal type
    /// - **Feature**: Describes the mathematical or visual characteristic
    ///
    /// # Mathematical References
    ///
    /// Names reference established terminology from fractal literature:
    /// - **"Elephant Valley"**: Named for trunk-like appendages
    /// - **"Seahorse Valley"**: Named for seahorse-like spiral structures
    /// - **"Spirals"**: Generic term for spiral boundary formations
    pub fn name(&self) -> &'static str {
        match self {
            Self::MandelbrotFull => "Full Mandelbrot Set",
            Self::MandelbrotElephantValley => "Mandelbrot Elephant Valley",
            Self::MandelbrotSpirals => "Mandelbrot Spirals",
            Self::MandelbrotSeahorseValley => "Mandelbrot Seahorse Valley",
            Self::MandelbrotSquaredSpirals => "Mandelbrot Squared Spirals",
        }
    }
    /// Converts the preset into concrete mathematical data for computation.
    ///
    /// Creates a complete `MathData` specification with precise coordinates
    /// and appropriate iteration counts for the mathematical region. Each
    /// preset uses coordinates derived from fractal exploration and literature.
    ///
    /// # Returns
    ///
    /// Complete mathematical specification ready for fractal computation
    ///
    /// # Coordinate Precision
    ///
    /// - **Full Set**: Broad coordinates showing complete structure
    /// - **Feature Zooms**: High-precision coordinates locating tiny features
    /// - **Literature Values**: Coordinates from established fractal references
    ///
    /// # Iteration Selection
    ///
    /// Iteration counts chosen based on mathematical requirements:
    /// - **Full Set**: 200 iterations sufficient for overall structure
    /// - **Deep Features**: 2000+ iterations needed for fine boundary detail
    /// - **Quality vs Speed**: Balanced for educational exploration
    ///
    /// # Mathematical Regions
    ///
    /// Each preset defines specific areas of mathematical interest:
    /// - **Boundary Regions**: Areas with complex fractal boundary structure
    /// - **Feature Centers**: Coordinates targeting famous visual formations
    /// - **Zoom Factors**: Appropriate scale for viewing each feature
    pub fn preset(&self) -> MathData {
        match self {
            // Full Mandelbrot set view: classic overview coordinates
            Self::MandelbrotFull => {
                MathData::from_presets(self.name().to_string(), "-0.675", "0", "1.25", 200)
            }

            // Elephant Valley: famous feature with trunk-like appendages
            Self::MandelbrotElephantValley => MathData::from_presets(
                self.name().to_string(),
                "-0.74728352972",
                "0.10757720113",
                "0.00020306307",
                2000,
            ),

            // Spiral formations: complex boundary spiral structures
            Self::MandelbrotSpirals => MathData::from_presets(
                self.name().to_string(),
                "-0.726516262498",
                "0.18783225",
                "0.00003",
                2000, // High iterations for spiral boundary resolution
            ),

            // Seahorse Valley: seahorse-like spiral patterns
            Self::MandelbrotSeahorseValley => MathData::from_presets(
                self.name().to_string(),
                "-0.74579999998",
                "0.10975",
                "0.0005",
                2000,
            ),

            // Squared spirals at a minibrot
            Self::MandelbrotSquaredSpirals => MathData::from_presets(
                self.name().to_string(),
                "-1.76622701902486844983",
                "0.01182325403486396853",
                "1.749564E-13",
                20000, // High iteration count for fine spiral detail
            ),
        }
    }
}

impl std::fmt::Display for MathPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// end of file
