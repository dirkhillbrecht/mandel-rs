/// Presets for the coloring system, especially pre-defined color schemes
use palette::Srgb;

use crate::storage::visualization::coloring::base::GradientColorScheme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GradientColorPreset {
    Sunrise,
    Woods,
    Moonlight,
    Gray,
    UglyColors,
}

impl GradientColorPreset {
    pub fn all() -> &'static [Self] {
        &[
            Self::Sunrise,
            Self::Woods,
            Self::Moonlight,
            Self::Gray,
            Self::UglyColors,
        ]
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sunrise => "Sunrise",
            Self::Woods => "Woods",
            Self::Moonlight => "Moonlight",
            Self::Gray => "Gray",
            Self::UglyColors => "Ugly Colors",
        }
    }
    pub fn scheme(&self) -> GradientColorScheme {
        match self {
            Self::Sunrise => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0),
                vec![
                    Srgb::new(0.0, 0.2, 0.7),
                    Srgb::new(1.0, 1.0, 1.0),
                    Srgb::new(1.0, 1.0, 0.2),
                    Srgb::new(0.8, 0.05, 0.0),
                ],
            ),
            Self::Woods => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0),
                vec![
                    Srgb::new(59.0 / 255.0, 216.0 / 255.0, 17.0 / 255.0),
                    Srgb::new(215.0 / 255.0, 179.0 / 255.0, 24.0 / 255.0),
                    Srgb::new(83.0 / 255.0, 209.0 / 255.0, 218.0 / 255.0),
                    Srgb::new(212.0 / 255.0, 212.0 / 255.0, 212.0 / 255.0),
                ],
            ),
            Self::Moonlight => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0),
                vec![
                    Srgb::new(103.0 / 255.0, 103.0 / 255.0, 103.0 / 255.0),
                    Srgb::new(166.0 / 255.0, 67.0 / 255.0, 167.0 / 255.0),
                    Srgb::new(255.0 / 255.0, 252.0 / 255.0, 0.0 / 255.0),
                    Srgb::new(111.0 / 255.0, 176.0 / 255.0, 255.0 / 255.0),
                ],
            ),
            Self::Gray => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0),
                vec![Srgb::new(0.0, 0.0, 0.0), Srgb::new(1.0, 1.0, 1.0)],
            ),
            Self::UglyColors => GradientColorScheme::new(
                Srgb::new(0.0, 0.0, 0.0),
                vec![
                    Srgb::new(1.0, 0.0, 0.0),
                    Srgb::new(0.875, 0.0, 0.375),
                    Srgb::new(0.0, 0.5, 0.5),
                    Srgb::new(0.0, 0.0, 1.0),
                    Srgb::new(1.0, 1.0, 1.0),
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IterationAssignment {
    Cubic,
    Squared,
    Linear,
    SquareRoot,
    CubicRoot,
    Logarithmic,
    LogLog,
}

impl IterationAssignment {
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
    pub fn assignment_function(&self) -> fn(u32) -> u32 {
        match self {
            Self::Cubic => |it| it * it * it,
            Self::Squared => |it| it * it,
            Self::Linear => |it| it,
            Self::SquareRoot => |it| (it as f64).sqrt() as u32,
            Self::CubicRoot => |it| (it as f64).powf(1.0 / 3.0) as u32,
            Self::Logarithmic => |it| (it as f64).ln() as u32,
            Self::LogLog => |it| (it as f64).ln().ln() as u32,
        }
    }
}

impl std::fmt::Display for IterationAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// end of file
