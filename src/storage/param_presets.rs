//! Parameter presets for computation
//!
//! This module contains a number of hard-coded presets for nice fractal images.

use crate::storage::{
    param_description::ParamDescription,
    visualization::coloring::presets::{GradientColorPreset, IterationAssignment},
};

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
    // An area which looks weaved
    MandelbrotWeaved,
    // A kraken-like area
    MandelbrotKraken,
    // Psychodelic spiral
    MandelbrotPsySpiral,
    // A caterpillar-like structure
    MandelbrotCaterpillar,
    // Bunches of spikes around a minibrot
    MandelbrotBunchOfSpikes,
    // A minibrot with lots of straight spikes
    MandelbrotStraightSpikes,
    // Minibrot on Mandelbrot's backside
    MandelbrotMinibrotOnBackside,
    // Flashes around a very tiny minibrot
    MandelbrotFlashes,
    // Minibrot in a jellyfish-like structure
    MandelbrotJellyfish,
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
            Self::MandelbrotWeaved,
            Self::MandelbrotKraken,
            Self::MandelbrotPsySpiral,
            Self::MandelbrotCaterpillar,
            Self::MandelbrotBunchOfSpikes,
            Self::MandelbrotStraightSpikes,
            Self::MandelbrotMinibrotOnBackside,
            Self::MandelbrotFlashes,
            Self::MandelbrotJellyfish,
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
            Self::MandelbrotWeaved => "Weave of Spirals",
            Self::MandelbrotKraken => "Kraken-like area",
            Self::MandelbrotPsySpiral => "Psychodelic spiral",
            Self::MandelbrotCaterpillar => "Caterpillar-like structure",
            Self::MandelbrotBunchOfSpikes => "Minibrot with bunches of spikes",
            Self::MandelbrotStraightSpikes => "Minibrot with straight spikes",
            Self::MandelbrotMinibrotOnBackside => "Minibrot on backside",
            Self::MandelbrotFlashes => "Flashes around a minibrot",
            Self::MandelbrotJellyfish => "Jellyfish with a minibrot",
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
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            // Elephant Valley: famous feature with trunk-like appendages
            Self::MandelbrotElephantValley => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.74728352972".to_owned(),
                center_y: "0.10757720113".to_owned(),
                radius: "0.00020306307".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 2000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            // Spiral formations: complex boundary spiral structures
            Self::MandelbrotSpirals => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.726516262498".to_owned(),
                center_y: "0.18783225".to_owned(),
                radius: "0.00003".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 2000, // High iterations for spiral boundary resolution
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            // Seahorse Valley: seahorse-like spiral patterns
            Self::MandelbrotSeahorseValley => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.74579999998".to_owned(),
                center_y: "0.10975".to_owned(),
                radius: "0.0005".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 2000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            // Squared spirals at a minibrot
            Self::MandelbrotSquaredSpirals => ParamDescription {
                name: self.name().to_string(),
                center_x: "-1.76622701902486844983".to_owned(),
                center_y: "0.01182325403486396853".to_owned(),
                radius: "1.749564E-13".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 20000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            // Minibrot with "ring of fire"
            Self::MandelbrotRingOfFire => ParamDescription {
                name: self.name().to_string(),
                center_x: "-1.15266540088230347".to_owned(),
                center_y: "0.30699874725259538".to_owned(),
                radius: "6.2385403E-10".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 20000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 190,
                stripe_offset: 160,
            },

            // Minibrot with "ring of fire"
            Self::MandelbrotWeaved => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.694241084688711".to_owned(),
                center_y: "0.369018494065763".to_owned(),
                radius: "1.7379089E-8".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 20000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            // Kraken-like area with a minibrot too small for f64 in the middle
            // Note: Use stripe count 2048 for this
            Self::MandelbrotKraken => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.36423818776604768336".to_owned(),
                center_y: "-0.65667699544311595692".to_owned(),
                radius: "1.1542801E-13".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 20000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 2048,
                stripe_offset: 0,
            },

            Self::MandelbrotPsySpiral => ParamDescription {
                name: self.name().to_string(),
                center_x: "-1.14856790732546".to_owned(),
                center_y: "0.2639603229136".to_owned(),
                radius: "3.6690958E-7".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 20000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            Self::MandelbrotCaterpillar => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.5382233952419".to_owned(),
                center_y: "0.6108236150811".to_owned(),
                radius: "0.0000011122613".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 2000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 256,
                stripe_offset: 0,
            },

            Self::MandelbrotBunchOfSpikes => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.995643151344189".to_owned(),
                center_y: "0.280397788186929".to_owned(),
                radius: "3.2430531E-8".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 40000,
                iteration_assignment: IterationAssignment::SquareRoot,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 64,
                stripe_offset: 0,
            },

            Self::MandelbrotStraightSpikes => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.99441587727171340975".to_owned(),
                center_y: "0.29980873842699326524".to_owned(),
                radius: "1.0769815E-13".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 8000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 250,
                stripe_offset: 0,
            },

            Self::MandelbrotMinibrotOnBackside => ParamDescription {
                name: self.name().to_string(),
                center_x: "0.250268969430133".to_owned(),
                center_y: "-0.000006636566143".to_owned(),
                radius: "1.4116211E-8".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 50000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 2048,
                stripe_offset: 0,
            },

            Self::MandelbrotFlashes => ParamDescription {
                name: self.name().to_string(),
                center_x: "-0.6922857838017087".to_owned(),
                center_y: "0.4785331215741747".to_owned(),
                radius: "9.3132215E-9".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 10000,
                iteration_assignment: IterationAssignment::Linear,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 500,
                stripe_offset: 365,
            },

            Self::MandelbrotJellyfish => ParamDescription {
                name: self.name().to_string(),
                center_x: "-1.749997863315900509".to_owned(),
                center_y: "0".to_owned(),
                radius: "1.1197185E-11".to_owned(),
                ratio: "1".to_owned(),
                max_iteration: 50000,
                iteration_assignment: IterationAssignment::SquareRoot,
                color_preset: GradientColorPreset::Sunrise,
                stripe_count: 2048,
                stripe_offset: 1995,
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
