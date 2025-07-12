//! Event data structures for pixel-level change notifications.
//!
//! This module defines the fundamental event types used to communicate
//! individual pixel changes from the computation system to the visualization
//! system. These events form the basis of the real-time synchronization
//! between parallel computation and UI rendering.
//!
//! # Event Architecture
//!
//! ```text
//! CompStage (Parallel) → DataPointChange Events → VizStorage (Sequential)
//!     ↓                        ↓                       ↓
//! Computation Threads  →  Event Batching        →   UI Thread
//! ```
//!
//! ## Event Types
//!
//! - **DataPointChange**: Single pixel update with coordinate and data
//! - **DataPointMultiChange**: Batched collection of pixel updates
//!
//! ## Design Principles
//!
//! - **Minimal Size**: Events are `Copy` for efficient transmission
//! - **Complete Information**: Each event contains coordinate + fractal data
//! - **Batching Support**: Multiple changes can be grouped for efficiency
//! - **Type Safety**: Strongly typed to prevent data corruption
//!
//! # Usage in Event Pipeline
//!
//! 1. **Generation**: CompStage creates events when pixels are computed
//! 2. **Batching**: StageEventBatcher groups events for efficiency
//! 3. **Transmission**: Events flow through async channels
//! 4. **Application**: VizStorage applies events to update visualization data
//!
//! This design enables real-time visualization updates during fractal
//! computation without blocking either the computation or UI threads.

use crate::storage::data_point::DataPoint;

/// Represents a single pixel update event in the computation-to-visualization pipeline.
///
/// Contains the complete information needed to update a specific pixel in the
/// visualization system: the pixel coordinates and the new fractal computation
/// result. This is the atomic unit of change in the event system.
///
/// # Event Content
///
/// - **Coordinates**: Pixel position in the computation grid
/// - **Data**: Complete fractal computation result (iteration count + final z-value)
///
/// # Memory Efficiency
///
/// This struct is `Copy` to enable efficient transmission through async channels
/// without allocation overhead. The contained `DataPoint` is also `Copy` for
/// the same reason.
///
/// # Usage
///
/// Created by `CompStage` when computation threads update pixel data,
/// transmitted through the event system, and applied by `VizStorage`
/// to maintain visualization synchronization.
#[derive(Debug, Clone, Copy)]
pub struct DataPointChange {
    /// X coordinate of the changed pixel (0 to width-1)
    pub x: u32,
    /// Y coordinate of the changed pixel (0 to height-1)
    pub y: u32,
    /// New fractal computation data for this pixel
    pub data: DataPoint,
}

impl DataPointChange {
    /// Creates a new pixel change event.
    ///
    /// # Arguments
    ///
    /// * `x` - Pixel X coordinate
    /// * `y` - Pixel Y coordinate
    /// * `data` - Fractal computation result to store
    ///
    /// # Returns
    ///
    /// Event ready for transmission through the event system
    pub fn new(x: u32, y: u32, data: &DataPoint) -> Self {
        DataPointChange { x, y, data: *data }
    }
}

/// Batched collection of pixel update events for efficient processing.
///
/// Groups multiple independent pixel changes into a single event to reduce
/// the overhead of individual event transmission and processing. This is
/// particularly important during intensive computation phases where many
/// pixels are updated rapidly.
///
/// # Batching Benefits
///
/// - **Reduced Channel Overhead**: Fewer async channel operations
/// - **Efficient Processing**: Batch application in visualization system
/// - **Lower Memory Pressure**: Fewer individual allocations
/// - **Better Cache Locality**: Related updates processed together
///
/// # Event Lifecycle
///
/// 1. **Collection**: `StageEventBatcher` accumulates individual changes
/// 2. **Batching**: Multiple `DataPointChange`s grouped into `DataPointMultiChange`
/// 3. **Transmission**: Single multi-change event sent through async channel
/// 4. **Application**: `VizStorage` iterates through batch and applies each change
///
/// # Independence Assumption
///
/// All changes in a batch are assumed to be independent (different pixels)
/// to ensure correct parallel processing and avoid data races.
#[derive(Debug, Clone)]
pub struct DataPointMultiChange {
    /// Collection of independent pixel changes to apply as a batch
    changes: Vec<DataPointChange>,
}

impl DataPointMultiChange {
    /// Creates a new batched change event.
    ///
    /// # Arguments
    ///
    /// * `changes` - Vector of individual pixel changes to batch together
    ///
    /// # Returns
    ///
    /// Batched event ready for efficient transmission and processing
    ///
    /// # Performance Note
    ///
    /// The input vector is moved (not copied) for efficiency.
    pub fn new(changes: Vec<DataPointChange>) -> Self {
        DataPointMultiChange { changes }
    }
    /// Returns a slice of all batched pixel changes.
    ///
    /// Provides access to the individual changes for iteration and
    /// application by the visualization system.
    ///
    /// # Returns
    ///
    /// Slice containing all pixel changes in this batch
    pub fn changes(&self) -> &[DataPointChange] {
        &self.changes
    }
    /// Returns the number of pixel changes in this batch.
    ///
    /// Useful for metrics, debugging, and batch size optimization.
    ///
    /// # Returns
    ///
    /// Count of individual pixel changes in the batch
    #[allow(dead_code)] // Public API for future use and debugging
    pub fn len(&self) -> usize {
        self.changes.len()
    }
}

// end of file
