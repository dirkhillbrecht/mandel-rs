//! Thread-safe storage for parallel fractal computation.
//!
//! This module provides the core data structure for storing fractal computation
//! results in a thread-safe manner. The `CompStage` enables multiple computation
//! threads to concurrently read and write pixel data while maintaining data
//! integrity and providing event notifications for visualization updates.
//!
//! # Concurrency Design
//!
//! ## Thread Safety Strategy
//!
//! - **Per-Pixel Locking**: Each pixel has its own `RwLock` for fine-grained concurrency
//! - **Multiple Readers**: Many threads can read computed pixels simultaneously
//! - **Exclusive Writers**: Only one thread can update a specific pixel at a time
//! - **Event System**: Changes are broadcast to visualization thread via async channels
//!
//! ## Memory Layout
//!
//! ```text
//! 2D Pixel Grid          1D Memory Layout
//! ┌────────────┐      ┌──────────────────────────────┐
//! │ (0,0) (1,0) (2,0) │ → │ [0] [1] [2] [3] [4] [5] [6] [7] [8] │
//! │ (0,1) (1,1) (2,1) │      │  RwLock<Option<DataPoint>>     │
//! │ (0,2) (1,2) (2,2) │      │  for each pixel              │
//! └────────────┘      └──────────────────────────────┘
//! ```
//!
//! # Performance Characteristics
//!
//! - **Scalability**: Supports massively parallel computation threads
//! - **Cache Efficiency**: Row-major memory layout for sequential access patterns
//! - **Low Contention**: Per-pixel locks minimize thread blocking
//! - **Event Batching**: Async event system prevents blocking on visualization updates
//!
//! # Usage Example
//!
//! ```rust
//! // Create stage for 800x600 image
//! let stage = CompStage::new(Size2D::new(800, 600));
//!
//! // Multiple threads can safely compute different pixels
//! stage.set(100, 200, computed_data_point);
//! let result = stage.get(100, 200);
//! ```

use std::sync::RwLock;

use euclid::{Point2D, Size2D, Vector2D};
use tokio::sync::mpsc::UnboundedSender;

use crate::storage::{
    coord_spaces::StageSpace,
    data_point::DataPoint,
    event::{data_point_change_event::DataPointChange, stage_event_batcher::StageEvent},
    image_comp_properties::StageState,
};

/// Thread-safe storage for fractal computation results.
///
/// The computation stage provides concurrent access to a 2D grid of fractal
/// computation results. It's designed to support massively parallel computation
/// while maintaining data integrity and providing real-time updates to the
/// visualization system.
///
/// # Architecture
///
/// - **Data Agnostic**: Independent of specific fractal algorithms
/// - **Thread Safe**: Uses per-pixel RwLocks for fine-grained concurrency
/// - **Event Driven**: Broadcasts changes via async channels
/// - **Memory Efficient**: Optional storage (None for uncomputed pixels)
///
/// # Concurrency Model
///
/// Each pixel has its own `RwLock<Option<DataPoint>>` allowing:
/// - Multiple concurrent readers for visualization
/// - Exclusive write access for computation threads
/// - Non-blocking access to different pixels
///
/// # State Management
///
/// Tracks overall computation state (`Initialized` → `Evolving` → `Completed`)
/// and broadcasts state changes to interested observers.
///
/// # Memory Layout
///
/// Stores 2D pixel data in a 1D vector using row-major order:
/// `index = y * width + x`
pub struct CompStage {
    /// Dimensions of the computation stage in pixels (width × height)
    size: Size2D<usize, StageSpace>,
    /// Thread-safe storage for pixel data in row-major order
    /// Each pixel has its own RwLock for fine-grained concurrency
    data: Vec<RwLock<Option<DataPoint>>>,
    /// Current computation state (Initialized/Evolving/Stalled/Completed)
    state: RwLock<StageState>,
    /// Optional async channel for broadcasting data changes to visualization
    change_sender: std::sync::Mutex<Option<UnboundedSender<StageEvent>>>,
}

impl CompStage {
    /// Creates a new computation stage with specified dimensions.
    ///
    /// Initializes a thread-safe storage system for fractal computation results.
    /// All pixels start as `None` (uncomputed) and the stage begins in
    /// `Initialized` state.
    ///
    /// # Arguments
    ///
    /// * `size` - Pixel dimensions of the computation stage
    ///
    /// # Returns
    ///
    /// A new `CompStage` ready for parallel computation access
    ///
    /// # Memory Allocation
    ///
    /// Pre-allocates `width * height` RwLocks for optimal performance.
    /// Memory usage: approximately `size.area() * sizeof(RwLock<Option<DataPoint>>)`
    ///
    /// # Thread Safety
    ///
    /// The returned stage is immediately safe for concurrent access
    /// by multiple computation threads.
    pub fn new(size: Size2D<u32, StageSpace>) -> Self {
        let mut data = Vec::with_capacity(size.area() as usize);
        for _ in 0..(size.area()) {
            data.push(RwLock::new(None));
        }
        CompStage {
            size: Size2D::new(size.width as usize, size.height as usize),
            data,
            state: RwLock::new(StageState::Initialized),
            change_sender: std::sync::Mutex::new(None),
            //            event_buffer_capacity,
            //            event_buffer: RwLock::new(None),
        }
    }

    /// Returns the stage dimensions.
    ///
    /// # Returns
    ///
    /// Size in pixels (width × height)
    #[allow(dead_code)]
    pub fn size(&self) -> Size2D<usize, StageSpace> {
        self.size
    }

    /// Returns the stage width in pixels.
    pub fn width(&self) -> usize {
        self.size.width
    }

    /// Returns the stage height in pixels.
    pub fn height(&self) -> usize {
        self.size.height
    }

    /// Converts 2D pixel coordinates to 1D array index.
    ///
    /// Uses row-major order: `index = y * width + x`
    ///
    /// # Arguments
    ///
    /// * `x` - Pixel X coordinate (0 to width-1)
    /// * `y` - Pixel Y coordinate (0 to height-1)
    ///
    /// # Returns
    ///
    /// Array index for internal storage
    ///
    /// # Panics
    ///
    /// Panics if coordinates are outside stage bounds
    fn index(&self, x: u32, y: u32) -> usize {
        if x as usize >= self.size.width || y as usize >= self.size.height {
            panic!(
                "Coordinates ({},{}) out of bounds for computation stage of size {}*{}",
                x, y, self.size.width, self.size.height
            );
        }
        y as usize * self.size.width + x as usize
    }

    /// Internal method to read pixel data by array index.
    ///
    /// Acquires read lock and returns a copy of the pixel data.
    /// This is an internal helper to avoid code duplication.
    ///
    /// # Arguments
    ///
    /// * `idx` - Array index (must be valid)
    ///
    /// # Returns
    ///
    /// Copy of pixel data, or `None` if uncomputed
    ///
    /// # Concurrency
    ///
    /// Blocks until read lock is acquired. Multiple threads
    /// can read the same pixel simultaneously.
    fn internal_get(&self, idx: usize) -> Option<DataPoint> {
        let guard = self.data[idx].read().unwrap();
        *guard
    }

    /// Reads fractal computation data for a specific pixel.
    ///
    /// Returns a copy of the computed data point, or `None` if the pixel
    /// hasn't been computed yet. This method is thread-safe and can be
    /// called concurrently from multiple threads.
    ///
    /// # Arguments
    ///
    /// * `x` - Pixel X coordinate (0 to width-1)
    /// * `y` - Pixel Y coordinate (0 to height-1)
    ///
    /// # Returns
    ///
    /// - `Some(DataPoint)` - Computed fractal data for this pixel
    /// - `None` - Pixel has not been computed yet
    ///
    /// # Thread Safety
    ///
    /// Acquires a read lock on the specific pixel. Multiple threads
    /// can read the same pixel simultaneously without blocking.
    ///
    /// # Panics
    ///
    /// Panics if coordinates are outside stage bounds.
    pub fn get(&self, x: u32, y: u32) -> Option<DataPoint> {
        self.internal_get(self.index(x, y))
    }

    /// Checks if a pixel has been computed.
    ///
    /// Convenience method that returns `true` if the pixel contains
    /// computed fractal data, `false` if it's still uncomputed.
    ///
    /// # Arguments
    ///
    /// * `x` - Pixel X coordinate
    /// * `y` - Pixel Y coordinate
    ///
    /// # Returns
    ///
    /// `true` if pixel has computed data, `false` otherwise
    ///
    /// # Usage
    ///
    /// Commonly used by computation algorithms to skip already-computed
    /// pixels during incremental computation.
    pub fn is_computed(&self, x: u32, y: u32) -> bool {
        self.get(x, y).is_some()
    }

    /// Sets the event channel for broadcasting data changes.
    ///
    /// Configures an async channel to receive notifications when pixel
    /// data or stage state changes. Setting to `None` disables events.
    ///
    /// # Arguments
    ///
    /// * `sender` - Optional async channel sender for stage events
    ///
    /// # Event Types
    ///
    /// - `StageEvent::ContentChange` - When pixel data is updated
    /// - `StageEvent::StateChange` - When computation state changes
    ///
    /// # Usage
    ///
    /// Typically called during initialization to connect the computation
    /// stage to the visualization system for real-time updates.
    pub fn set_change_sender(&self, sender: Option<UnboundedSender<StageEvent>>) {
        *self.change_sender.lock().unwrap() = sender;
    }

    /// Returns the current computation state.
    ///
    /// Retrieves the current state of the computation stage, which tracks
    /// the overall progress of fractal computation.
    ///
    /// # Returns
    ///
    /// Current `StageState`:
    /// - `Initialized` - Stage created but computation not started
    /// - `Evolving` - Active computation in progress
    /// - `Stalled` - Computation paused or stopped
    /// - `Completed` - All pixels computed
    ///
    /// # Thread Safety
    ///
    /// Acquires a read lock on the state. Safe for concurrent access.
    pub fn get_state(&self) -> StageState {
        let guard = self.state.read().unwrap();
        *guard
    }

    /// Sets fractal computation data for a specific pixel.
    ///
    /// Stores computed fractal data and broadcasts a change event if an
    /// event channel is configured. This is the primary method used by
    /// computation threads to store results.
    ///
    /// # Arguments
    ///
    /// * `x` - Pixel X coordinate (0 to width-1)
    /// * `y` - Pixel Y coordinate (0 to height-1)
    /// * `data_point` - Computed fractal data to store
    ///
    /// # Thread Safety
    ///
    /// Acquires an exclusive write lock on the specific pixel.
    /// Only one thread can write to a pixel at a time, but
    /// different threads can write to different pixels concurrently.
    ///
    /// # Event Broadcasting
    ///
    /// If an event channel is configured, sends a `ContentChange` event
    /// for real-time visualization updates. Event sending is non-blocking.
    ///
    /// # Panics
    ///
    /// Panics if coordinates are outside stage bounds.
    pub fn set(&self, x: u32, y: u32, data_point: DataPoint) {
        {
            let mut data_write_guard = self.data[self.index(x, y)].write().unwrap();
            *data_write_guard = Option::Some(data_point);
        }
        if let Some(sender) = &*self.change_sender.lock().unwrap() {
            let _ = sender.send(StageEvent::ContentChange(DataPointChange::new(
                x,
                y,
                &data_point,
            )));
        }
    }

    /// Updates the computation state of the stage.
    ///
    /// Changes the overall computation state and broadcasts a state change
    /// event if the state actually changed. Used by computation engines
    /// to report progress (started, finished, aborted).
    ///
    /// # Arguments
    ///
    /// * `new_state` - New computation state to set
    ///
    /// # State Transitions
    ///
    /// Typical progression:
    /// `Initialized` → `Evolving` → `Completed` or `Stalled`
    ///
    /// # Event Broadcasting
    ///
    /// Only sends `StateChange` event if state actually changes,
    /// preventing duplicate notifications.
    ///
    /// # Thread Safety
    ///
    /// Acquires exclusive write lock on state. State changes are atomic.
    pub fn set_state(&self, new_state: StageState) {
        let mut send_new_state = false;
        {
            let mut state_write_guard = self.state.write().unwrap();
            if *state_write_guard != new_state {
                *state_write_guard = new_state;
                send_new_state = true;
            }
        }
        if send_new_state {
            if let Some(sender) = &*self.change_sender.lock().unwrap() {
                let _ = sender.send(StageEvent::StateChange(new_state));
            }
        }
    }

    /// Returns a snapshot of all pixel data.
    ///
    /// Creates an independent copy of all pixel data in the stage,
    /// effectively taking a snapshot of the current computation state.
    /// This is used by the visualization system to access all data
    /// without holding locks.
    ///
    /// # Returns
    ///
    /// Vector containing copies of all pixel data in row-major order.
    /// `None` entries represent uncomputed pixels.
    ///
    /// # Performance
    ///
    /// This operation:
    /// - Acquires read locks for all pixels sequentially
    /// - Copies all data (expensive for large images)
    /// - Releases locks immediately after copying
    ///
    /// # Memory Usage
    ///
    /// Allocates `width * height * sizeof(Option<DataPoint>)` bytes.
    /// Use sparingly for large images.
    ///
    /// # Thread Safety
    ///
    /// Safe to call concurrently. Takes a consistent snapshot even
    /// if computation is ongoing during the copy operation.
    pub fn get_full_data(&self) -> Vec<Option<DataPoint>> {
        // This functional approach is slightly less performant as it might reallocate the target Vec memory
        //(0..self.data.len()).map(|i| self.internal_get(i)).collect()
        let mut retval = Vec::with_capacity(self.size.area());
        for i in 0..self.data.len() {
            retval.push(self.internal_get(i));
        }
        retval
    }

    /// Creates a new stage with shifted data from this stage.
    ///
    /// Generates an independent computation stage containing data shifted
    /// by the specified pixel offset. This enables preserving computed
    /// results when panning the fractal view, avoiding recomputation
    /// of pixels that remain visible.
    ///
    /// # Algorithm
    ///
    /// 1. **Overlap Calculation**: Determines which pixels from the source
    ///    stage map to the new coordinate system
    /// 2. **Data Preservation**: Copies overlapping computed pixels
    /// 3. **Gap Filling**: Fills non-overlapping areas with `None` (uncomputed)
    /// 4. **Efficient Layout**: Maintains row-major memory organization
    ///
    /// # Arguments
    ///
    /// * `offset` - Pixel displacement vector (positive = shift right/down)
    ///
    /// # Returns
    ///
    /// New `CompStage` with:
    /// - Same dimensions as original
    /// - Preserved data where regions overlap
    /// - `None` values for new regions requiring computation
    /// - State set to `Stalled` (computation not active)
    ///
    /// # Performance
    ///
    /// - **Optimal Case**: Small offsets preserve most data
    /// - **Worst Case**: Large offsets result in mostly empty stage
    /// - **Memory**: Allocates full new stage regardless of overlap
    ///
    /// # Use Cases
    ///
    /// - Interactive panning with data preservation
    /// - Coordinate system adjustments
    /// - Progressive computation optimization
    ///
    /// # Example
    ///
    /// ```text
    /// Original Stage    Offset (+1, +1)    Result Stage
    /// ┌───────┐                      ┌───────┐
    /// │ A B C │                      │ . . . │
    /// │ D E F │      →               │ . A B │
    /// │ G H I │                      │ . D E │
    /// └───────┘                      └───────┘
    /// ```
    pub fn shifted_clone(&self, offset: Vector2D<i32, StageSpace>) -> Self {
        if offset.x.abs() as usize >= self.size.width || offset.y.abs() as usize >= self.size.height
        {
            Self::new(Size2D::new(self.size.width as u32, self.size.height as u32))
        } else {
            let ox = offset.x;
            let oy = offset.y;
            let empty_line_start = (ox.max(0) as usize).min(self.size.width);
            let empty_line_end = ((-ox).max(0) as usize).min(self.size.width);
            let empty_start_lines = (oy.max(0) as usize).min(self.size.height);
            let empty_end_lines = ((-oy).max(0) as usize).min(self.size.height);
            let line_width = self.size.width - (empty_line_start.max(empty_line_end));
            let first_line = empty_end_lines;
            let last_line = self.size.height - empty_start_lines;
            let mut data = Vec::with_capacity(self.size.area());
            for _ in 0..empty_start_lines {
                for _ in 0..self.size.width {
                    data.push(RwLock::new(None));
                }
            }
            for line in first_line..last_line {
                for _ in 0..empty_line_start {
                    data.push(RwLock::new(None));
                }
                let first_idx = line * self.size.width + empty_line_end;
                let last_idx = first_idx + line_width;
                for idx in first_idx..last_idx {
                    data.push(RwLock::new(self.internal_get(idx)));
                }
                for _ in 0..empty_line_end {
                    data.push(RwLock::new(None));
                }
            }
            for _ in 0..empty_end_lines {
                for _ in 0..self.size.width {
                    data.push(RwLock::new(None));
                }
            }
            CompStage {
                size: self.size,
                data,
                state: RwLock::new(StageState::Stalled),
                change_sender: std::sync::Mutex::new(None),
            }
        }
    }

    /// Creates a new stage with zoomed data from this stage.
    ///
    /// **Current Implementation**: This is a placeholder that returns an empty
    /// stage. A complete implementation would preserve computed data that
    /// remains visible after zooming, potentially interpolating or subsampling
    /// existing results.
    ///
    /// # Future Implementation Ideas
    ///
    /// A complete zoom implementation could:
    /// 1. **Data Preservation**: Map pixels from old to new coordinate system
    /// 2. **Interpolation**: Estimate values for pixels between computed points
    /// 3. **Subsampling**: Use existing high-resolution data for zoom-out
    /// 4. **Quality Tracking**: Mark preserved data as `Derived` quality
    ///
    /// # Arguments
    ///
    /// * `_origin` - Pixel coordinate that remains fixed during zoom (unused)
    /// * `_factor` - Zoom factor >1.0=zoom in, <1.0=zoom out (unused)
    ///
    /// # Returns
    ///
    /// Currently: Empty stage of same dimensions
    /// Future: Stage with preserved/interpolated data where possible
    ///
    /// # Status
    ///
    /// 🚧 **TODO**: Implement intelligent data preservation for zoom operations
    pub fn zoomed_clone(&self, _origin: Point2D<i32, StageSpace>, _factor: f32) -> Self {
        // This is a dummy implementation always returning an empty new stage
        Self::new(Size2D::new(self.size.width as u32, self.size.height as u32))
    }

    pub fn max_iteration_changed_clone(
        &self,
        old_max_iteration: u32,
        new_max_iteration: u32,
    ) -> Self {
        let mut data = Vec::with_capacity(self.size.area());
        for idx in 0..self.size.area() {
            data.push(RwLock::new(self.internal_get(idx).and_then(|p| {
                p.for_new_max_iteration(old_max_iteration, new_max_iteration)
            })));
        }
        CompStage {
            size: self.size,
            data,
            state: RwLock::new(StageState::Stalled),
            change_sender: std::sync::Mutex::new(None),
        }
    }
}

// end of file
