//! Visualization-optimized 2D data storage for fractal rendering.
//!
//! This module provides the visualization side of the dual-storage architecture,
//! offering sequential access patterns optimized for UI operations and rendering.
//! Unlike the computation storage with its fine-grained locking, this storage
//! prioritizes simple, fast access for the single-threaded visualization system.
//!
//! # Design Philosophy
//!
//! ## Sequential Access Optimization
//! The storage is designed for the access patterns typical in visualization:
//! - Row-by-row iteration for rendering
//! - Random access for pixel queries
//! - Bulk updates from event streams
//! - Progress tracking and completion detection
//!
//! ## Memory Layout
//!
//! ```text
//! Linear Memory Layout (row-major order):
//! ┌─────┬─────┬─────┬─────┐
//! │(0,0)│(1,0)│(2,0)│(3,0)│ ← Row 0
//! ├─────┼─────┼─────┼─────┤
//! │(0,1)│(1,1)│(2,1)│(3,1)│ ← Row 1
//! ├─────┼─────┼─────┼─────┤
//! │(0,2)│(1,2)│(2,2)│(3,2)│ ← Row 2
//! └─────┴─────┴─────┴─────┘
//! Index: 0    1    2    3    4    5    6    7    8    9   10   11
//! ```
//!
//! # Data Lifecycle
//!
//! 1. **Initialization**: Copies initial state from CompStage
//! 2. **Updates**: Receives individual pixel updates via events
//! 3. **Queries**: Provides data for rendering and UI operations
//! 4. **Progress Tracking**: Monitors computation completion status
//!
//! # Performance Characteristics
//!
//! - **O(1)** random access to any pixel
//! - **O(1)** progress ratio calculation (cached count)
//! - **O(n)** full data iteration (optimal for rendering)
//! - **Minimal overhead** for event-driven updates

use crate::storage::computation::comp_stage::CompStage;
use crate::storage::data_point::DataPoint;
use crate::storage::event::data_point_change_event::DataPointChange;

/// Sequential-access fractal data storage optimized for visualization operations.
///
/// Provides a simple, fast storage system designed for the single-threaded
/// visualization system. Unlike the computation storage with its complex
/// locking mechanisms, this storage prioritizes simplicity and speed for
/// UI operations.
///
/// # Storage Strategy
///
/// - **Linear Layout**: Row-major order for cache-friendly iteration
/// - **Optional Data**: Uses `Option<DataPoint>` to track computation progress
/// - **Cached Metrics**: Maintains computed pixel count for O(1) progress queries
/// - **Event Integration**: Seamlessly integrates with the event system
///
/// # Access Patterns
///
/// Optimized for common visualization operations:
/// - **Rendering**: Efficient row-by-row or full-buffer iteration
/// - **UI Queries**: Fast random access for pixel inspection
/// - **Progress Display**: Instant computation completion percentage
/// - **Event Updates**: Efficient single-pixel modifications
///
/// # Memory Efficiency
///
/// - **Compact Storage**: Single `Vec` with minimal overhead
/// - **Optional Pattern**: Only stores computed pixels
/// - **Clone Support**: Full stage copying for transformations
///
/// # Thread Safety
///
/// Designed for single-threaded access by the visualization system.
/// Thread safety achieved through event-driven synchronization rather
/// than internal locking mechanisms.
#[derive(Debug, Clone)]
pub struct VizStage {
    /// Stage width in pixels
    width: usize,
    /// Stage height in pixels
    height: usize,
    /// Linear storage for fractal data (row-major order)
    /// None = not yet computed, Some(data) = computed pixel
    data: Vec<Option<DataPoint>>,
    /// Cached count of computed pixels for O(1) progress queries
    set_count: usize,
}

impl VizStage {
    /// Creates a new visualization stage from a computation stage.
    ///
    /// Performs a complete snapshot of the computation stage data, creating
    /// an independent copy optimized for visualization operations. This
    /// initialization establishes the baseline state for event-driven updates.
    ///
    /// # Arguments
    ///
    /// * `comp_stage` - Source computation stage to copy from
    ///
    /// # Returns
    ///
    /// New visualization stage with identical dimensions and data
    ///
    /// # Data Transfer Process
    ///
    /// 1. **Dimension Copy**: Captures width/height from computation stage
    /// 2. **Data Snapshot**: Creates independent copy of all pixel data
    /// 3. **Count Calculation**: Computes initial progress metrics
    /// 4. **Independence**: Result is fully independent of source stage
    ///
    /// # Performance
    ///
    /// - **O(n)** where n = width × height (full data copy)
    /// - **Memory allocation**: Single `Vec` allocation for optimal layout
    /// - **Cache friendly**: Linear memory layout for future access
    ///
    /// # Usage
    ///
    /// Typically called once during VizStorage initialization to establish
    /// the visualization baseline before event-driven updates begin.
    pub fn new(comp_stage: &CompStage) -> Self {
        let data = comp_stage.get_full_data();
        let set_count = data
            .iter()
            .filter(|p| p.is_some_and(|q| q.iteration_count_quality.is_accurate()))
            .count();
        VizStage {
            width: comp_stage.width(),
            height: comp_stage.height(),
            data,
            set_count,
        }
    }
    /// Returns the stage width in pixels.
    ///
    /// # Returns
    ///
    /// Width of the fractal computation area in pixels
    pub fn width(&self) -> usize {
        self.width
    }
    /// Returns the stage height in pixels.
    ///
    /// # Returns
    ///
    /// Height of the fractal computation area in pixels
    pub fn height(&self) -> usize {
        self.height
    }
    /// Returns the number of computed pixels in this stage.
    ///
    /// Provides the raw count of pixels that have been computed and stored.
    /// This count is maintained incrementally for O(1) access.
    ///
    /// # Returns
    ///
    /// Number of pixels with computed fractal data
    ///
    /// # Usage
    ///
    /// Useful for progress tracking, debugging, and metrics collection.
    /// Consider using `computed_ratio()` for percentage-based progress.
    #[allow(dead_code)]
    pub fn set_count(&self) -> usize {
        self.set_count
    }
    /// Returns the ratio of computed pixels as a value between 0.0 and 1.0.
    ///
    /// Calculates the percentage of pixels that have been computed, providing
    /// a normalized progress indicator suitable for progress bars and UI display.
    ///
    /// # Returns
    ///
    /// - `0.0` = No pixels computed
    /// - `1.0` = All pixels computed
    /// - Values between 0.0 and 1.0 = Partial completion
    ///
    /// # Performance
    ///
    /// O(1) operation using cached `set_count` for efficiency.
    ///
    /// # Usage
    ///
    /// ```rust
    /// // Display progress percentage
    /// let progress = viz_stage.computed_ratio() * 100.0;
    /// println!("Computation: {:.1}% complete", progress);
    /// ```
    pub fn computed_ratio(&self) -> f32 {
        (self.set_count as f32 / self.data.len() as f32)
            .min(1.0)
            .max(0.0)
    }
    /// Checks if all pixels in the stage have been computed.
    ///
    /// Determines completion status by comparing the computed pixel count
    /// against the total number of pixels in the stage.
    ///
    /// # Returns
    ///
    /// - `true` if all pixels have been computed
    /// - `false` if computation is still in progress
    ///
    /// # Performance
    ///
    /// O(1) operation using cached counts for efficiency.
    ///
    /// # Usage
    ///
    /// Used to determine when computation can be considered complete
    /// and whether UI should show "finished" status.
    pub fn is_fully_computed(&self) -> bool {
        self.set_count >= self.data.len()
    }
    /// Converts 2D coordinates to linear array index (row-major order).
    ///
    /// Transforms (x, y) pixel coordinates into the corresponding index
    /// in the linear data array. Uses row-major ordering for cache-friendly
    /// access patterns during rendering operations.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    ///
    /// # Returns
    ///
    /// Linear index into the data array
    ///
    /// # Panics
    ///
    /// Panics if coordinates are outside the stage boundaries.
    /// This is a programming error that should be caught during development.
    ///
    /// # Formula
    ///
    /// ```text
    /// index = y × width + x
    /// ```
    ///
    /// This ensures row-major ordering where consecutive x values
    /// are stored consecutively in memory.
    fn index(&self, x: usize, y: usize) -> usize {
        if x >= self.width || y >= self.height {
            panic!(
                "Coordinates ({},{}) out of bounds for visualization stage of size {}*{}",
                x, y, self.width, self.height
            );
        }
        y * self.width + x
    }
    /// Retrieves fractal data for a specific pixel.
    ///
    /// Provides read-only access to the computed fractal data at the
    /// specified coordinates. Returns `None` if the pixel has not been
    /// computed yet.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    ///
    /// # Returns
    ///
    /// - `Some(&DataPoint)` if pixel has been computed
    /// - `None` if pixel is not yet computed
    ///
    /// # Performance
    ///
    /// O(1) random access using direct array indexing.
    ///
    /// # Panics
    ///
    /// Panics if coordinates are out of bounds (see `index()` method).
    ///
    /// # Usage
    ///
    /// ```rust
    /// // Check if a specific pixel has been computed
    /// if let Some(data) = viz_stage.get(x, y) {
    ///     println!("Pixel ({}, {}) escaped after {} iterations", x, y, data.iterations);
    /// }
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Option<&DataPoint> {
        self.data[self.index(x, y)].as_ref()
    }
    /// Updates fractal data for a specific pixel.
    ///
    /// Stores computed fractal data at the specified coordinates and
    /// maintains the progress tracking count. This is the primary method
    /// for updating visualization data during computation.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (0 to width-1)
    /// * `y` - Y coordinate (0 to height-1)
    /// * `data_point` - Computed fractal data to store
    ///
    /// # Behavior
    ///
    /// - **New Data**: Increments `set_count` if pixel was previously uncomputed
    /// - **Update**: Overwrites existing data without changing count
    /// - **Progress Tracking**: Automatically maintains completion metrics
    ///
    /// # Performance
    ///
    /// O(1) operation with minimal overhead for count maintenance.
    ///
    /// # Panics
    ///
    /// Panics if coordinates are out of bounds (see `index()` method).
    ///
    /// # Usage
    ///
    /// Typically called through `set_from_change()` as part of the
    /// event-driven update system.
    pub fn set(&mut self, x: usize, y: usize, data_point: DataPoint) {
        let index = self.index(x, y);
        if self.data[index].is_none_or(|p| !p.iteration_count_quality.is_accurate()) {
            self.set_count += 1
        }
        self.data[index] = Some(data_point);
    }
    /// Applies a pixel change event to update visualization data.
    ///
    /// Convenience method that extracts coordinates and data from a
    /// `DataPointChange` event and applies it to the visualization stage.
    /// This is the primary interface for event-driven updates.
    ///
    /// # Arguments
    ///
    /// * `data_point_change` - Event containing pixel coordinates and new data
    ///
    /// # Event Processing
    ///
    /// - **Coordinate Extraction**: Safely converts u32 to usize coordinates
    /// - **Data Application**: Updates pixel using standard `set()` method
    /// - **Progress Tracking**: Automatically maintains completion metrics
    ///
    /// # Performance
    ///
    /// O(1) operation equivalent to direct `set()` call.
    ///
    /// # Usage
    ///
    /// Primary method used by `VizStorage::process_events()` to apply
    /// computation updates to the visualization system:
    ///
    /// ```rust
    /// // In event processing loop
    /// for change in pixel_changes {
    ///     viz_stage.set_from_change(change);
    /// }
    /// ```
    pub fn set_from_change(&mut self, data_point_change: DataPointChange) {
        self.set(
            data_point_change.x as usize,
            data_point_change.y as usize,
            data_point_change.data,
        );
    }
}

// end of file
