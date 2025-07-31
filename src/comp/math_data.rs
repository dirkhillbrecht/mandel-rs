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

use euclid::{Point2D, Rect};

use crate::storage::coord_spaces::MathSpace;

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
    coordinates: Rect<f64, MathSpace>,
    /// Maximum iteration count for escape-time algorithm
    max_iteration: u32,
}

impl MathData {
    /// Creates new mathematical data from complete parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable description of the mathematical region
    /// * `coordinates` - Rectangular region in complex plane (euclid::Rect)
    /// * `max_iteration` - Maximum escape-time iterations
    ///
    /// # Returns
    ///
    /// Complete mathematical specification ready for fractal computation
    ///
    /// # Coordinate System
    ///
    /// The `coordinates` parameter uses euclid's type-safe `Rect<f64, MathSpace>`:
    /// - **Origin**: Bottom-left corner of rectangle
    /// - **Size**: Width and height in complex plane units
    /// - **Type Safety**: MathSpace prevents coordinate system confusion
    pub fn new(name: String, coordinates: Rect<f64, MathSpace>, max_iteration: u32) -> Self {
        MathData {
            name,
            coordinates,
            max_iteration,
        }
    }
    /// Creates mathematical data from two corner points in the complex plane.
    ///
    /// Convenience constructor that builds a rectangular region from two
    /// points, automatically determining the bounding box. The points can
    /// be specified in any order (top-left + bottom-right or vice versa).
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable description of the region
    /// * `p1` - First corner point in complex plane
    /// * `p2` - Opposite corner point in complex plane
    /// * `max_iteration` - Maximum escape-time iterations
    ///
    /// # Returns
    ///
    /// Mathematical specification with rectangular region from point bounds
    ///
    /// # Point Ordering
    ///
    /// Points can be specified in any order - the rectangle will be computed
    /// to encompass both points regardless of which is "top-left" vs "bottom-right".
    ///
    /// # Example Usage
    ///
    /// ```rust
    /// // Define classic Mandelbrot viewing window
    /// let math_data = MathData::from_points(
    ///     "Full Mandelbrot".to_string(),
    ///     Point2D::new(-2.5, -1.25),  // Bottom-left
    ///     Point2D::new(1.0, 1.25),    // Top-right
    ///     100
    /// );
    /// ```
    pub fn from_points(
        name: String,
        p1: Point2D<f64, MathSpace>,
        p2: Point2D<f64, MathSpace>,
        max_iteration: u32,
    ) -> Self {
        Self::new(name, Rect::from_points([p1, p2]), max_iteration)
    }
    /// Creates mathematical data from raw coordinate values.
    ///
    /// Most convenient constructor for specifying rectangular regions using
    /// individual coordinate components. Automatically constructs the complex
    /// plane points and bounding rectangle.
    ///
    /// # Arguments
    ///
    /// * `name` - Human-readable description of the region
    /// * `x1` - Real component of first corner point
    /// * `y1` - Imaginary component of first corner point
    /// * `x2` - Real component of opposite corner point
    /// * `y2` - Imaginary component of opposite corner point
    /// * `max_iteration` - Maximum escape-time iterations
    ///
    /// # Returns
    ///
    /// Complete mathematical specification ready for computation
    ///
    /// # Coordinate Interpretation
    ///
    /// In the complex plane coordinate system:
    /// - **x1, x2**: Real axis coordinates (horizontal)
    /// - **y1, y2**: Imaginary axis coordinates (vertical)
    /// - **Order**: Points can be in any order (min/max computed automatically)
    ///
    /// # Example
    ///
    /// ```rust
    /// // Classic Mandelbrot set view
    /// let math_data = MathData::from_coordinates(
    ///     "Full Set".to_string(),
    ///     -2.5, -1.25,  // Bottom-left: -2.5 + -1.25i
    ///     1.0, 1.25,    // Top-right: 1.0 + 1.25i
    ///     100
    /// );
    /// ```
    pub fn from_coordinates(
        name: String,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        max_iteration: u32,
    ) -> Self {
        Self::from_points(
            name,
            Point2D::new(x1, y1),
            Point2D::new(x2, y2),
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
        self.coordinates
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
                MathData::from_coordinates(
                    self.name().to_string(),
                    -2.1, -1.25,  // Bottom-left: captures main cardioid and bulb
                    0.75, 1.25,   // Top-right: includes major features
                    200            // Sufficient iterations for overall structure
                )
            }
            
            // Elephant Valley: famous feature with trunk-like appendages
            Self::MandelbrotElephantValley => MathData::from_coordinates(
                self.name().to_string(),
                -0.7512, 0.1103,  // High-precision coordinates for tiny feature
                -0.7502, 0.1093,  // Very small region requiring deep zoom
                2000,              // High iteration count for boundary detail
            ),
            
            // Spiral formations: complex boundary spiral structures
            Self::MandelbrotSpirals => MathData::from_coordinates(
                self.name().to_string(),
                -0.7269, 0.1879,  // Precise location of spiral formations
                -0.7259, 0.1879,  // Narrow region showing spiral detail
                2000,              // High iterations for spiral boundary resolution
            ),
            
            // Seahorse Valley: seahorse-like spiral patterns
            Self::MandelbrotSeahorseValley => MathData::from_coordinates(
                self.name().to_string(),
                -0.7463, 0.1092,  // Coordinates targeting seahorse structures
                -0.7453, 0.1103,  // Small region for detailed feature view
                2000,              // High iteration count for fine spiral detail
            ),

            // Squared spirals at a minibrot
            Self::MandelbrotSquaredSpirals => MathData::from_coordinates(
                self.name().to_string(),
                -1.7662270190246352,
                0.011823254035038925,
                -1.7662270190251017,
                0.011823254034689012,
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
