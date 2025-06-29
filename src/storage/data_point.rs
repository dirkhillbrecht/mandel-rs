// Data point for the computed data of a fractal image

use euclid::Point2D;

use crate::storage::coord_spaces::MathSpace;

/// What's the quality of the data this setting refers to?
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum DataQuality {
    // The data is completely unknown
    Unknown,
    // The data is guessed, can be right, can be completely wrong
    Guessed,
    // The data is derived from other data and correct (e.g. iteration depth in closed area)
    Derived,
    // The data is actually computed and definitely correct
    Computed,
}

/// Store information about one point of the fractal - whatever it may be
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct DataPoint {
    pub iteration_count: u32,
    pub iteration_count_quality: DataQuality,
    pub final_coordinate: Point2D<f64, MathSpace>,
    pub final_coordinate_quality: DataQuality,
}

impl DataPoint {
    // Constructor
    pub fn new(
        iteration_count: u32,
        iteration_count_quality: DataQuality,
        final_coordinate: Point2D<f64, MathSpace>,
        final_coordinate_quality: DataQuality,
    ) -> DataPoint {
        DataPoint {
            iteration_count,
            iteration_count_quality,
            final_coordinate,
            final_coordinate_quality,
        }
    }
    pub fn computed(iteration_count: u32, final_coordinate: Point2D<f64, MathSpace>) -> DataPoint {
        Self::new(
            iteration_count,
            DataQuality::Computed,
            final_coordinate,
            DataQuality::Computed,
        )
    }
    pub fn as_guessed(&self) -> DataPoint {
        Self::new(
            self.iteration_count,
            DataQuality::Guessed,
            self.final_coordinate,
            DataQuality::Guessed,
        )
    }
}

// end of file
