//! Fundamental data structures for fractal computation results.
//!
//! This module defines the core data types that represent computed fractal information
//! for individual pixels. Each data point captures both the mathematical result
//! (iteration count and final complex value) and metadata about the computation
//! quality and confidence.
//!
//! # Quality Tracking
//!
//! The system tracks data quality to support:
//! - Progressive computation with intermediate estimates
//! - Data interpolation from neighboring computed points
//! - Optimization techniques that preserve/derive values
//! - Visual feedback about computation completeness
//!
//! # Usage
//!
//! ```rust
//! // Create a computed data point from fractal algorithm
//! let point = DataPoint::computed(42, final_z_value);
//!
//! // Create an estimated point for progressive rendering
//! let estimated = some_computed_point.as_guessed();
//! ```

use euclid::Point2D;

use crate::storage::coord_spaces::MathSpace;

/// Represents the quality and confidence level of computed fractal data.
///
/// Data quality tracking enables sophisticated rendering strategies including
/// progressive computation, interpolation, and optimization. Different quality
/// levels indicate how the data was obtained and how much confidence we have
/// in its accuracy.
///
/// # Quality Hierarchy
///
/// From lowest to highest confidence:
/// `Unknown` < `Guessed` < `Derived` < `Computed`
///
/// # Use Cases
///
/// - **Progressive Rendering**: Show estimated values while computation proceeds
/// - **Interpolation**: Fill gaps with guessed values from nearby computed points
/// - **Optimization**: Preserve computed values during coordinate transformations
/// - **Visual Feedback**: Color-code pixels based on computation confidence
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum DataQuality {
    /// No information available - pixel has not been processed
    Unknown,
    /// Estimated value based on interpolation or heuristics - may be inaccurate
    Guessed,
    /// Mathematically derived from other computed data - accurate but not direct
    Derived,
    /// Directly computed through fractal iteration - highest accuracy
    Computed,
}

/// Complete fractal computation result for a single pixel.
///
/// Stores both the mathematical results of fractal iteration and metadata
/// about how those results were obtained. This rich data structure enables
/// advanced rendering techniques, progressive computation, and quality tracking.
///
/// # Fields
///
/// - **Iteration Data**: How many iterations before escape (or max reached)
/// - **Final Coordinate**: The final z-value after iteration (for smooth coloring)
/// - **Quality Tracking**: Confidence level for both iteration count and coordinate
///
/// # Mathematical Context
///
/// For Mandelbrot computation:
/// - `iteration_count`: Number of iterations before |z| > 2.0 (or max_iteration)
/// - `final_coordinate`: The z-value after the final iteration
/// - Quality indicates whether values are computed, estimated, or derived
///
/// # Memory Layout
///
/// This struct is designed to be `Copy` for efficient storage in large 2D arrays
/// representing the complete fractal image data.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct DataPoint {
    /// Number of iterations before escape (or max_iteration if point didn't escape)
    pub iteration_count: u32,
    /// Quality/confidence level of the iteration count value
    pub iteration_count_quality: DataQuality,
    /// Final complex number z after iteration (for smooth coloring algorithms)
    pub final_coordinate: Point2D<f64, MathSpace>,
    /// Quality/confidence level of the final coordinate value
    pub final_coordinate_quality: DataQuality,
}

impl DataPoint {
    /// Creates a new data point with specified values and quality levels.
    ///
    /// # Arguments
    ///
    /// * `iteration_count` - Number of iterations computed
    /// * `iteration_count_quality` - Confidence level for iteration count
    /// * `final_coordinate` - Final z-value in mathematical coordinates
    /// * `final_coordinate_quality` - Confidence level for final coordinate
    ///
    /// # Returns
    ///
    /// A new `DataPoint` with the specified values and quality metadata
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
    /// Creates a data point from direct fractal computation.
    ///
    /// Convenience constructor for results from actual Mandelbrot iteration.
    /// Both the iteration count and final coordinate are marked as `Computed`
    /// quality, indicating they were obtained through direct mathematical
    /// calculation rather than estimation or interpolation.
    ///
    /// # Arguments
    ///
    /// * `iteration_count` - Iterations before escape (or max_iteration)
    /// * `final_coordinate` - Final z-value from iteration
    ///
    /// # Returns
    ///
    /// A new `DataPoint` with `Computed` quality for all fields
    ///
    /// # Usage
    ///
    /// ```rust
    /// // From Mandelbrot iteration algorithm
    /// let result = DataPoint::computed(42, Point2D::new(1.5, 2.1));
    /// ```
    pub fn computed(iteration_count: u32, final_coordinate: Point2D<f64, MathSpace>) -> DataPoint {
        Self::new(
            iteration_count,
            DataQuality::Computed,
            final_coordinate,
            DataQuality::Computed,
        )
    }
    /// Creates a copy of this data point with quality downgraded to `Guessed`.
    ///
    /// Used when repurposing computed data for estimation or interpolation.
    /// The mathematical values remain unchanged, but the quality metadata
    /// is updated to reflect that these values are now being used as
    /// estimates rather than direct computation results.
    ///
    /// # Returns
    ///
    /// A new `DataPoint` with the same values but `Guessed` quality
    ///
    /// # Use Cases
    ///
    /// - Progressive rendering with placeholder values
    /// - Interpolation between computed points
    /// - Estimating values for zoomed or transformed coordinates
    ///
    /// # Example
    ///
    /// ```rust
    /// let computed = DataPoint::computed(100, final_z);
    /// let estimated = computed.as_guessed(); // Same values, different quality
    /// ```
    pub fn as_guessed(&self) -> DataPoint {
        Self::new(
            self.iteration_count,
            DataQuality::Guessed,
            self.final_coordinate,
            DataQuality::Guessed,
        )
    }
    /// Creates a copy of this data point containing the data for a changed maximum iteration.
    ///
    /// If the current data max iteration is deeper than then requested new maximum iteration depth,
    /// a point the the new max iteration depth (and unknown final coordinate) is returned.
    /// If the current data max iteration is equal to the old max iteration (and the new max iteration
    /// is deeper than the old max iteration) an empty data point is returned to invalidate this data.
    /// In all other cases, a copy of this data point is returned.
    pub fn for_new_max_iteration(
        &self,
        old_max_iteration: u32,
        new_max_iteration: u32,
    ) -> Option<Self> {
        if self.iteration_count > new_max_iteration {
            Some(Self::new(
                new_max_iteration,
                self.iteration_count_quality,
                Point2D::zero(),
                DataQuality::Unknown,
            ))
        } else if self.iteration_count == old_max_iteration && new_max_iteration > old_max_iteration
        {
            None
        } else {
            Some(*self)
        }
    }
}

// end of file
