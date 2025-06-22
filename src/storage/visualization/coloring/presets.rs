/// Presets for the coloring system, especially pre-defined color schemes
use palette::Srgb;

use crate::storage::visualization::coloring::base::GradientColorScheme;

#[allow(dead_code)]
pub enum GradientColorPresets {
    Sunrise,
    Woods,
    Moonlight,
    Gray,
    UglyColors,
}

impl GradientColorPresets {
    #[allow(dead_code)]
    pub fn all() -> &'static [Self] {
        &[
            Self::Sunrise,
            Self::Woods,
            Self::Moonlight,
            Self::Gray,
            Self::UglyColors,
        ]
    }
    #[allow(dead_code)]
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

// end of file
