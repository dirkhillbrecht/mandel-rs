//! Visualization storage system for the dual-storage architecture.
//!
//! This module implements the visualization side of the dual-storage pattern,
//! providing sequential access to fractal data optimized for rendering and
//! UI operations. It receives updates from the parallel computation system
//! via async events and maintains a local copy suitable for visualization.
//!
//! # Dual-Storage Architecture
//!
//! ```text
//! ┌───────────────────────────────────────────────────────┐
//! │                 Computation Side                 │    Events    │
//! │  ┌───────────────────────────────────────────┐  │       ────────────────────────────────────────────────────────────┐
//! │  │          CompStorage                    │  │  ┌─────────────────────────────────────────────────────────┐
//! │  │  • Parallel access (RwLocks)          │──▶│  │                VizStorage                  │
//! │  │  • Computation threads              │  │  │  • Sequential access (simple refs)      │
//! │  │  • Thread-safe operations           │  │  │  • Visualization thread               │
//! │  │  • Event broadcasting               │  │  │  • UI-optimized operations           │
//! │  └───────────────────────────────────────────┘  │  └─────────────────────────────────────────────────────────┘
//! └───────────────────────────────────────────────────────┘            └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # Key Features
//!
//! ## Event-Driven Synchronization
//! - Receives updates from CompStorage via async channels
//! - Processes batched events for efficiency
//! - Maintains eventual consistency with computation state
//!
//! ## Visualization Optimization
//! - Sequential access patterns suitable for rendering
//! - No locking overhead for UI thread operations
//! - Immediate data availability for visualization
//!
//! ## Resource Management
//! - Automatic event system lifecycle management
//! - Cleanup on computation completion/abort
//! - Efficient memory usage with shared coordinate systems
//!
//! # Usage Pattern
//!
//! 1. **Creation**: Link to CompStorage and initialize event system
//! 2. **Processing**: Regularly call `process_events()` to receive updates
//! 3. **Visualization**: Use stage data for rendering operations
//! 4. **Cleanup**: Automatic resource cleanup on completion

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc::UnboundedReceiver;

use super::viz_stage::VizStage;
use crate::storage::computation::comp_storage::CompStorage;
use crate::storage::event::stage_event_batcher::StageEvent;
use crate::storage::image_comp_properties::{ImageCompProperties, StageState};

/// Visualization-optimized storage for fractal data.
///
/// Provides the visualization side of the dual-storage architecture,
/// maintaining a local copy of fractal computation data optimized for
/// sequential access by the UI thread. Receives updates from the parallel
/// computation system and ensures the visualization always has current data.
///
/// # Design Goals
///
/// - **Sequential Access**: Optimized for UI thread access patterns
/// - **No Blocking**: Never blocks on computation thread operations
/// - **Event Synchronization**: Stays current via async event streams
/// - **Resource Efficiency**: Minimal overhead for visualization operations
///
/// # Lifecycle
///
/// 1. **Initialization**: Links to CompStorage and captures initial state
/// 2. **Synchronization**: Receives and processes events from computation
/// 3. **Visualization**: Provides data for rendering operations
/// 4. **Cleanup**: Automatically manages event system resources
///
/// # Thread Safety
///
/// Designed for single-threaded access by the visualization system.
/// Thread safety is achieved through the event-driven synchronization
/// pattern rather than locking mechanisms.
pub struct VizStorage {
    /// Coordinate system and computation parameters (shared with CompStorage)
    pub properties: ImageCompProperties,
    /// Sequential-access fractal data storage optimized for visualization
    pub stage: VizStage,
    /// Last known computation state from event stream
    /// Used for tracking computation progress and completion
    #[allow(dead_code)]
    pub seen_state: StageState,
    /// Reference to source computation storage for lifecycle management
    comp_storage: Arc<CompStorage>,
    /// Async event receiver for computation updates, None after completion
    event_receiver: Option<UnboundedReceiver<StageEvent>>,
}

impl VizStorage {
    /// Creates a new visualization storage linked to a computation storage.
    ///
    /// Establishes the visualization side of the dual-storage architecture by:
    /// 1. Setting up event synchronization with the computation storage
    /// 2. Creating an initial snapshot of the computation state
    /// 3. Initializing the visualization stage with current data
    ///
    /// # Arguments
    ///
    /// * `arc_of_comp_storage` - Shared reference to the computation storage
    ///
    /// # Returns
    ///
    /// New `VizStorage` instance synchronized with the computation storage
    ///
    /// # Synchronization Process
    ///
    /// 1. **Event System Setup**: Establishes async event channel (1000 capacity, 50ms batching)
    /// 2. **State Snapshot**: Captures current computation state
    /// 3. **Data Copy**: Creates visualization-optimized copy of all computed data
    /// 4. **Ready State**: Returns fully synchronized visualization storage
    ///
    /// # Thread Safety
    ///
    /// The initialization process ensures no data is lost during setup by
    /// establishing event synchronization before copying data.
    pub fn new(arc_of_comp_storage: &Arc<CompStorage>) -> VizStorage {
        // Step 1: Establish event synchronization to prevent missing updates
        // Configure batching: up to 1000 events per batch, maximum 50ms delay
        let event_receiver_result = arc_of_comp_storage
            .get_event_receiver(1000, Duration::from_millis(50))
            .ok();

        // Step 2: Capture current computation state for progress tracking
        let seen_state = arc_of_comp_storage.stage.get_state();

        // Step 3: Create visualization stage with initial data snapshot
        // This reads all current computation data into visualization-optimized format
        let stage = VizStage::new(&arc_of_comp_storage.as_ref().stage);
        VizStorage {
            properties: arc_of_comp_storage.properties.clone(),
            stage,
            seen_state,
            comp_storage: arc_of_comp_storage.clone(),
            event_receiver: event_receiver_result,
        }
    }

    /// Processes pending events from the computation storage.
    ///
    /// Reads and applies all available events from the async event stream,
    /// keeping the visualization storage synchronized with computation progress.
    /// This method should be called regularly (e.g., during UI update cycles)
    /// to maintain current visualization data.
    ///
    /// # Returns
    ///
    /// - `true` if any events were processed (visualization data changed)
    /// - `false` if no events were available (no updates needed)
    ///
    /// # Event Types Handled
    ///
    /// - **ContentChange**: Single pixel update from computation
    /// - **ContentMultiChange**: Batch of pixel updates for efficiency
    /// - **StateChange**: Computation state transitions (evolving/completed/stalled)
    ///
    /// # Performance
    ///
    /// - **Non-blocking**: Uses `try_recv()` to avoid blocking UI thread
    /// - **Batch Processing**: Handles multiple events in single call
    /// - **Early Exit**: Returns immediately when no events available
    ///
    /// # Resource Management
    ///
    /// Automatically cleans up the event system when computation completes
    /// or is aborted, releasing async resources.
    ///
    /// # Usage Pattern
    ///
    /// ```rust
    /// // In UI update loop
    /// if viz_storage.process_events() {
    ///     // Redraw visualization with updated data
    ///     invalidate_canvas();
    /// }
    /// ```
    pub fn process_events(&mut self) -> bool {
        let mut events_handled = false;
        if let Some(receiver) = &mut self.event_receiver {
            // Process all available events in batch for efficiency
            while let Ok(event) = receiver.try_recv() {
                events_handled = true;
                match event {
                    // Single pixel update: Apply directly to visualization stage
                    StageEvent::ContentChange(change) => {
                        self.stage.set_from_change(change);
                    }
                    // Multiple pixel updates: Process batch efficiently
                    StageEvent::ContentMultiChange(changes) => {
                        changes
                            .changes()
                            .iter()
                            .for_each(|change| self.stage.set_from_change(*change));
                    }
                    // Computation state change: Handle lifecycle management
                    StageEvent::StateChange(thestate) => {
                        // Clean up event system when computation ends
                        if thestate == StageState::Stalled || thestate == StageState::Completed {
                            let _ = self.comp_storage.drop_event_receiver();
                        }
                    }
                }
            }
        }
        events_handled
    }
}

// end of file
