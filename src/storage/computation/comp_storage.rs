//! High-level coordination for parallel fractal computation.
//!
//! This module provides the orchestration layer that coordinates between
//! the thread-safe computation stage and the asynchronous event system.
//! It manages the complete lifecycle of fractal computation including
//! event streaming, coordinate transformations, and state synchronization.
//!
//! # Architecture Overview
//!
//! ## Component Integration
//!
//! ```text
//! ┌──────────────────────────────────────────────────────┐
//! │                    CompStorage                       │
//! │  ┌──────────────────┐  ┌──────────────────────────┐  │
//! │  │ ImageCompProps   │  │       CompStage          │  │
//! │  │                  │  │                          │  │
//! │  │ • Coordinate Sys │  │ • Thread-safe Storage    │  │
//! │  │ • Math ↔ Pixel   │  │ • Per-pixel RwLocks      │  │
//! │  │ • Zoom/Pan Trans │  │ • Parallel Access        │  │
//! │  └──────────────────┘  └──────────────────────────┘  │
//! │                                                      │
//! │  ┌─────────────────────────────────────────────────┐ │
//! │  │                Event System                     │ │
//! │  │                                                 │ │
//! │  │  CompStage → Batcher → VizStorage               │ │
//! │  │  (async event streaming with batching)          │ │
//! │  └─────────────────────────────────────────────────┘ │
//! └──────────────────────────────────────────────────────┘
//! ```
//!
//! ## Event System Lifecycle
//!
//! 1. **Setup**: Create computation storage with coordinate system
//! 2. **Activation**: Start async event system with batching parameters
//! 3. **Computation**: Parallel threads write to CompStage, events stream to VizStorage
//! 4. **Navigation**: Create transformed clones for pan/zoom operations
//! 5. **Cleanup**: Tear down event system when switching coordinate systems
//!
//! ## Key Responsibilities
//!
//! - **Coordinate Management**: Maintains both original and rectified coordinate systems
//! - **Event Orchestration**: Manages async communication between computation and visualization
//! - **Navigation Support**: Provides efficient cloning with coordinate transformations
//! - **Resource Management**: Handles proper setup/teardown of async resources
//!
//! # Thread Safety
//!
//! All operations are thread-safe. The event system uses async channels for
//! non-blocking communication between computation threads and the visualization system.

use std::time::Duration;

use euclid::{Point2D, Vector2D};
use tokio::sync::mpsc;

use super::comp_stage::CompStage;
use crate::storage::{
    coord_spaces::StageSpace,
    event::stage_event_batcher::{StageEvent, StageEventBatcher},
    image_comp_properties::ImageCompProperties,
};

/// Manages the async event system for streaming computation updates.
///
/// Encapsulates the async infrastructure that connects computation threads
/// to the visualization system. Handles spawning the event batching task
/// and managing communication channels.
///
/// # Lifecycle
///
/// - **Inactive**: Both fields are `None`, no async resources allocated
/// - **Active**: Task spawned, sender connected to CompStage
/// - **Cleanup**: Task aborted, channels dropped
struct EventSystem {
    /// Handle to the async event batching task, `None` when inactive
    task_handle: Option<tokio::task::JoinHandle<()>>,
    /// Channel sender for CompStage to send events, `None` when inactive
    sender: Option<mpsc::UnboundedSender<StageEvent>>,
}

impl EventSystem {
    pub fn new() -> Self {
        EventSystem {
            task_handle: None,
            sender: None,
        }
    }
}

/// Errors that can occur during event system management.
///
/// These errors help prevent invalid state transitions and provide
/// clear feedback about event system lifecycle issues.
pub enum EventSystemError {
    /// Attempted to start event system when already active
    /// Only one event system can be active per CompStorage instance
    AlreadyActive,
    /// Attempted to stop event system when not currently active
    /// Must start event system before stopping it
    NotActive,
}

/// Comprehensive storage system for parallel fractal computation.
///
/// Orchestrates the complete fractal computation pipeline by combining
/// mathematical coordinate systems, thread-safe data storage, and async
/// event streaming. This is the primary interface for computation engines
/// and the central coordination point for the dual-storage architecture.
///
/// # Architecture Components
///
/// ## Coordinate Systems
/// - **Original Properties**: User-specified mathematical area and resolution
/// - **Rectified Properties**: Adjusted for square pixels and optimal computation
///
/// ## Data Storage
/// - **CompStage**: Thread-safe storage with per-pixel RwLocks
/// - **Event System**: Async streaming of computation updates
///
/// ## Key Features
/// - **Parallel Access**: Multiple computation threads can work simultaneously
/// - **Real-time Updates**: Changes stream to visualization system via async events
/// - **Navigation Support**: Efficient coordinate transformations for pan/zoom
/// - **Resource Management**: Proper async task lifecycle management
///
/// # Usage Example
///
/// ```rust
/// // Create storage for specific coordinate area
/// let storage = CompStorage::new(image_properties);
///
/// // Start event streaming to visualization
/// let receiver = storage.get_event_receiver(1000, Duration::from_millis(50))?;
///
/// // Multiple computation threads can now safely write to storage.stage
/// // Events automatically stream to visualization system
///
/// // Clean shutdown
/// storage.drop_event_receiver()?;
/// ```
pub struct CompStorage {
    /// Original user-specified coordinate system and computation parameters
    /// Preserved for reference and coordinate transformations
    pub original_properties: ImageCompProperties,
    /// Rectified coordinate system optimized for computation
    /// Ensures square pixels and proper aspect ratios
    pub properties: ImageCompProperties,
    /// Thread-safe fractal data storage with per-pixel concurrency
    pub stage: CompStage,
    /// Async event system management (protected by mutex for thread safety)
    event_system: std::sync::Mutex<EventSystem>,
}

impl CompStorage {
    /// Creates a new computation storage system.
    ///
    /// Initializes the complete computation infrastructure including coordinate
    /// systems, thread-safe storage, and event system management. The storage
    /// is ready for parallel computation but the event system remains inactive
    /// until explicitly started.
    ///
    /// # Arguments
    ///
    /// * `original_properties` - User-specified coordinate system and parameters
    ///
    /// # Returns
    ///
    /// Fully initialized `CompStorage` ready for computation
    ///
    /// # Coordinate Processing
    ///
    /// The original properties are automatically rectified to ensure:
    /// - Square pixels (dotsize.width == dotsize.height)
    /// - Optimal aspect ratios for computation
    /// - Proper mathematical coordinate alignment
    ///
    /// # Thread Safety
    ///
    /// The returned storage is immediately safe for concurrent access
    /// by multiple computation threads.
    pub fn new(original_properties: ImageCompProperties) -> CompStorage {
        let properties = original_properties.rectified(false);
        CompStorage {
            original_properties,
            properties: properties.clone(),
            stage: CompStage::new(properties.stage_properties.pixels.clone()),
            event_system: std::sync::Mutex::new(EventSystem::new()),
        }
    }

    /// Activates the async event system and returns the visualization receiver.
    ///
    /// Spawns the event batching infrastructure that streams computation updates
    /// from the CompStage to the visualization system. Events are batched for
    /// efficiency while maintaining responsiveness.
    ///
    /// # Arguments
    ///
    /// * `max_capacity` - Maximum events per batch (performance tuning)
    /// * `max_interval` - Maximum time between batches (responsiveness tuning)
    ///
    /// # Returns
    ///
    /// - `Ok(receiver)` - Channel receiver for visualization system
    /// - `Err(AlreadyActive)` - Event system is already running
    ///
    /// # Event Pipeline
    ///
    /// ```text
    /// CompStage → [Unbounded Channel] → EventBatcher → [Unbounded Channel] → VizStorage
    /// ```
    ///
    /// The batcher runs as an async task, collecting events and forwarding
    /// them in efficient batches to prevent overwhelming the visualization system.
    ///
    /// # Resource Management
    ///
    /// This method allocates async resources (task, channels) that must be
    /// properly cleaned up with `drop_event_receiver()` to prevent resource leaks.
    ///
    /// # Thread Safety
    ///
    /// Safe to call from any thread. Only one event system can be active
    /// per CompStorage instance.
    pub fn get_event_receiver(
        &self,
        max_capacity: usize,
        max_interval: Duration,
    ) -> Result<mpsc::UnboundedReceiver<StageEvent>, EventSystemError> {
        let mut event_system = self.event_system.lock().unwrap();

        // event system cannot be active twice
        if event_system.sender.is_some() {
            return Err(EventSystemError::AlreadyActive);
        }
        // Create channel for CompStage sending events to batcher
        let (comp_sender, comp_receiver) = mpsc::unbounded_channel();
        // Create channel for VizStorage receiving events from batcher
        let (viz_sender, viz_receiver) = mpsc::unbounded_channel();
        // Create the batcher
        let batcher = StageEventBatcher::new(max_capacity, max_interval);
        // Spawn the async task, this also connects both channels to the batcher
        let task_handle = tokio::task::spawn(batcher.run(comp_receiver, viz_sender));
        // Connect the comp channel to the stage
        self.stage.set_change_sender(Some(comp_sender.clone()));
        // Put everything in event system
        event_system.sender = Some(comp_sender);
        event_system.task_handle = Some(task_handle);

        // And finally return the receiver to the caller
        Ok(viz_receiver)
    }

    /// Deactivates the async event system and cleans up resources.
    ///
    /// Properly shuts down the event streaming infrastructure, aborts the
    /// async batching task, and releases all associated resources. This is
    /// essential for preventing resource leaks when switching coordinate systems
    /// or shutting down computation.
    ///
    /// # Returns
    ///
    /// - `Ok(())` - Event system successfully deactivated
    /// - `Err(NotActive)` - No event system was currently active
    ///
    /// # Cleanup Process
    ///
    /// 1. **Disconnect CompStage**: Stop sending new events
    /// 2. **Abort Async Task**: Terminate the event batching task
    /// 3. **Drop Channels**: Release channel endpoints
    /// 4. **Reset State**: Return to inactive state
    ///
    /// # Thread Safety
    ///
    /// Safe to call from any thread. The cleanup process is atomic and
    /// prevents partial teardown states.
    ///
    /// # Usage
    ///
    /// Always call this method before dropping CompStorage or switching
    /// to a different coordinate system to ensure proper resource cleanup.
    pub fn drop_event_receiver(&self) -> Result<(), EventSystemError> {
        let mut event_system = self.event_system.lock().unwrap();

        if event_system.sender.is_none() {
            return Err(EventSystemError::NotActive);
        }

        // Disconnect CompStage from the event system
        self.stage.set_change_sender(None);
        event_system.task_handle.take().unwrap().abort();
        event_system.sender.take().unwrap(); // Dropping the sender automatically closes the channel - according to Claude…

        Ok(())
    }

    /// Creates a new CompStorage with coordinate system shifted by pixel offset.
    ///
    /// Generates a complete new computation storage with the mathematical
    /// coordinate system translated by the specified pixel displacement.
    /// Preserves computed data where possible through intelligent stage cloning.
    ///
    /// # Arguments
    ///
    /// * `offset` - Pixel displacement vector (positive = right/down)
    ///
    /// # Returns
    ///
    /// New `CompStorage` with:
    /// - Translated coordinate systems (both original and rectified)
    /// - Shifted stage data preserving overlapping computed pixels
    /// - Inactive event system (must be reactivated if needed)
    ///
    /// # Data Preservation
    ///
    /// The stage cloning preserves computed fractal data that remains
    /// visible after the coordinate shift, avoiding unnecessary recomputation.
    /// New areas requiring computation are initialized as uncomputed.
    ///
    /// # Use Cases
    ///
    /// - Interactive panning operations
    /// - Coordinate system adjustments
    /// - Progressive computation optimization
    ///
    /// # Performance
    ///
    /// - **Optimal**: Small offsets preserve most computed data
    /// - **Standard**: Large offsets require mostly fresh computation
    /// - **Memory**: Allocates complete new storage structures
    pub fn shifted_clone_by_pixels(&self, offset: Vector2D<i32, StageSpace>) -> Self {
        CompStorage {
            original_properties: self
                .original_properties
                .shifted_clone_by_math(self.properties.pixel_to_math_offset(offset)),
            properties: self.properties.shifted_clone_by_pixels(offset),
            stage: self.stage.shifted_clone(offset),
            event_system: std::sync::Mutex::new(EventSystem::new()),
        }
    }

    /// Creates a new CompStorage with coordinate system zoomed around a pixel.
    ///
    /// Generates a complete new computation storage with the mathematical
    /// coordinate system scaled around the specified origin point. The zoom
    /// transformation updates both coordinate systems appropriately.
    ///
    /// # Arguments
    ///
    /// * `origin` - Pixel coordinate that remains fixed during zoom
    /// * `factor` - Zoom factor (>1.0 = zoom in, <1.0 = zoom out)
    ///
    /// # Returns
    ///
    /// New `CompStorage` with:
    /// - Scaled coordinate systems (origin pixel maintains its mathematical coordinate)
    /// - Fresh stage initialized for new coordinate system
    /// - Inactive event system (must be reactivated if needed)
    ///
    /// # Coordinate Transformation
    ///
    /// The zoom operation maintains the mathematical invariant:
    /// ```text
    /// old_storage.properties.pix_to_math(origin) == new_storage.properties.pix_to_math(origin)
    /// ```
    ///
    /// # Data Handling
    ///
    /// Currently creates a fresh stage (no data preservation). Future
    /// implementations could preserve and interpolate existing computed data.
    ///
    /// # Use Cases
    ///
    /// - Interactive zoom operations
    /// - Mathematical area exploration
    /// - Detail level adjustments
    pub fn zoomed_clone_by_pixels(&self, origin: Point2D<i32, StageSpace>, factor: f32) -> Self {
        let new_properties = self.properties.zoomed_clone_by_pixels(origin, factor);
        CompStorage {
            original_properties: new_properties.clone(),
            properties: new_properties,
            stage: self.stage.zoomed_clone(origin, factor),
            event_system: std::sync::Mutex::new(EventSystem::new()),
        }
    }

    pub fn max_iteration_changed_clone(
        &self,
        old_max_iteration: u32,
        new_max_iteration: u32,
    ) -> Self {
        CompStorage {
            original_properties: self
                .original_properties
                .max_iteration_changed_clone(new_max_iteration),
            properties: self
                .properties
                .max_iteration_changed_clone(new_max_iteration),
            stage: self
                .stage
                .max_iteration_changed_clone(old_max_iteration, new_max_iteration),
            event_system: std::sync::Mutex::new(EventSystem::new()),
        }
    }
}

// end of file
