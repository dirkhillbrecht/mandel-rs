use euclid::{Point2D, Rect};

use crate::storage::coord_spaces::MathSpace;

/// Known fractal types of this program
#[allow(dead_code)]
pub enum FractalType {
    /// The all-famous Mandelbrot set
    Mandelbrot,
}

// Representation of some mathematical data describing an interesting region of a (mandelbrot) fractal.
pub struct MathData {
    name: String,
    // fractal_type: FractalType, // Introduce this once we can compute other fractal types
    coordinates: Rect<f64, MathSpace>,
    max_iteration: u32,
}

impl MathData {
    pub fn new(name: String, coordinates: Rect<f64, MathSpace>, max_iteration: u32) -> Self {
        MathData {
            name,
            coordinates,
            max_iteration,
        }
    }
    pub fn from_points(
        name: String,
        p1: Point2D<f64, MathSpace>,
        p2: Point2D<f64, MathSpace>,
        max_iteration: u32,
    ) -> Self {
        Self::new(name, Rect::from_points([p1, p2]), max_iteration)
    }
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
    #[allow(dead_code)]
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn coordinates(&self) -> Rect<f64, MathSpace> {
        self.coordinates
    }
    pub fn max_iteration(&self) -> u32 {
        self.max_iteration
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MathPreset {
    MandelbrotFull,
    MandelbrotElephantValley,
    MandelbrotSpirals,
    MandelbrotSeahorseValley,
}

impl MathPreset {
    pub fn all() -> &'static [Self] {
        &[
            Self::MandelbrotFull,
            Self::MandelbrotElephantValley,
            Self::MandelbrotSpirals,
            Self::MandelbrotSeahorseValley,
        ]
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::MandelbrotFull => "Full Mandelbrot Set",
            Self::MandelbrotElephantValley => "Mandelbrot Elephant Valley",
            Self::MandelbrotSpirals => "Mandelbrot Spirals",
            Self::MandelbrotSeahorseValley => "Mandelbrot Seahorse Valley",
        }
    }
    pub fn preset(&self) -> MathData {
        match self {
            Self::MandelbrotFull => {
                MathData::from_coordinates(self.name().to_string(), -2.1, -1.25, 0.75, 1.25, 200)
            }
            Self::MandelbrotElephantValley => MathData::from_coordinates(
                self.name().to_string(),
                -0.7512,
                0.1103,
                -0.7502,
                0.1093,
                2000,
            ),
            Self::MandelbrotSpirals => MathData::from_coordinates(
                self.name().to_string(),
                -0.7269,
                0.1879,
                -0.7259,
                0.1879,
                2000,
            ),
            Self::MandelbrotSeahorseValley => MathData::from_coordinates(
                self.name().to_string(),
                -0.7463,
                0.1092,
                -0.7453,
                0.1103,
                2000,
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
