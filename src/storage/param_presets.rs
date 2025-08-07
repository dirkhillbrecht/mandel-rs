//! Parameter presets for computation
//!
//! This module contains a number of hard-coded presets for nice fractal images.

use crate::storage::param_description::ParamDescription;

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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamPreset {
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
    // Mandelbrot with ring of fire around it
    MandelbrotRingOfFire,
}

impl ParamPreset {
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
            Self::MandelbrotRingOfFire,
        ]
    }

    /// Returns the human-readable name of the mathematical preset.
    pub fn name(&self) -> &'static str {
        match self {
            Self::MandelbrotFull => "Full Mandelbrot Set",
            Self::MandelbrotElephantValley => "Mandelbrot Elephant Valley",
            Self::MandelbrotSpirals => "Mandelbrot Spirals",
            Self::MandelbrotSeahorseValley => "Mandelbrot Seahorse Valley",
            Self::MandelbrotSquaredSpirals => "Mandelbrot Squared Spirals",
            Self::MandelbrotRingOfFire => "Minibrot with Ring of Fire",
        }
    }

    /// Actual preset data
    pub fn preset(&self) -> ParamDescription {
        match self {
            // Full Mandelbrot set view: classic overview coordinates
            Self::MandelbrotFull => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.675".to_owned(),
                center_y: "0".to_owned(),
                radius: "1.25".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 200,
            },

            // Elephant Valley: famous feature with trunk-like appendages
            Self::MandelbrotElephantValley => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.74728352972".to_owned(),
                center_y: "0.10757720113".to_owned(),
                radius: "0.00020306307".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 2000,
            },

            // Spiral formations: complex boundary spiral structures
            Self::MandelbrotSpirals => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.726516262498".to_owned(),
                center_y: "0.18783225".to_owned(),
                radius: "0.00003".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 2000, // High iterations for spiral boundary resolution
            },

            // Seahorse Valley: seahorse-like spiral patterns
            Self::MandelbrotSeahorseValley => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.74579999998".to_owned(),
                center_y: "0.10975".to_owned(),
                radius: "0.0005".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 2000,
            },

            // Squared spirals at a minibrot
            Self::MandelbrotSquaredSpirals => ParamDescription {
                name: self.name().to_string(),
                center_x: "-1.76622701902486844983".to_owned(),
                center_y: "0.01182325403486396853".to_owned(),
                radius: "1.749564E-13".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 20000,
            },

            // Minibrot with "ring of fire"
            Self::MandelbrotRingOfFire => ParamDescription {
                name: self.name().to_string(),
                center_x: "-1.15266540088230347".to_owned(),
                center_y: "0.30699874725259538".to_owned(),
                radius: "6.2385403E-10".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 20000,
            },
        }
    }
}

impl std::fmt::Display for ParamPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// end of file
