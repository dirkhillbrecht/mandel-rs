//! State management and event handling for the Iced GUI application.
//!
//! This module implements the core state management logic of the fractal visualization
//! application using Iced's message-driven architecture. It processes all user interactions,
//! mathematical parameter changes, and system events to maintain application state and
//! coordinate between GUI components and the computation engine.
//!
//! # Architecture
//!
//! ## Message-Driven Updates
//! Follows Iced's functional update pattern:
//! ```text
//! User Action → Message → update() → State Change → UI Re-render
//! ```
//!
//! ## State Categories
//! The update system manages several categories of application state:
//!
//! ### Mathematical Parameters
//! - **Coordinate Area**: Complex plane region for computation
//! - **Resolution**: Image dimensions (width × height)
//! - **Iteration Limits**: Maximum escape-time iterations
//! - **Presets**: Pre-defined mathematical regions
//!
//! ### Computation Lifecycle
//! - **Engine Management**: Start/stop/monitor computation threads
//! - **Storage Coordination**: Sync between computation and visualization
//! - **Progress Tracking**: Real-time computation progress updates
//! - **Resource Cleanup**: Proper disposal of computation resources
//!
//! ### Visual Configuration
//! - **Color Schemes**: Gradient and assignment function selection
//! - **Render Settings**: Image scaling and presentation options
//! - **UI State**: Sidebar visibility and layout preferences
//!
//! ### Interactive Navigation
//! - **Panning**: Coordinate system translation via drag operations
//! - **Zooming**: Scale transformation with timeout-based completion
//! - **Visual Feedback**: Real-time preview during operations
//!
//! # Key Design Patterns
//!
//! ## Async Task Management
//! Uses Iced's `Task` system for:
//! - **Computation Triggers**: Auto-starting computation after parameter changes
//! - **Progress Updates**: Scheduled visualization refresh cycles
//! - **Interactive Delays**: Zoom timeout detection and completion
//!
//! ## Resource Lifecycle
//! Careful management of computation resources:
//! - **Engine Coordination**: Stop existing computation before starting new
//! - **Storage Synchronization**: Maintain dual-storage consistency
//! - **Cache Invalidation**: Clear visual caches when parameters change
//!
//! ## Error Handling
//! Robust handling of user input and system state:
//! - **Input Validation**: Parse and validate user-entered parameters
//! - **State Consistency**: Ensure valid state transitions
//! - **Fallback Behavior**: Graceful handling of invalid operations

use crate::comp::mandelbrot_engine::{EngineState, MandelbrotEngine};
use crate::gui::iced::app::{AppState, ZoomState};
use crate::gui::iced::message::Message;
use crate::storage::computation::comp_storage::CompStorage;
use crate::storage::image_comp_properties::{ImageCompProperties, StageProperties};
use crate::storage::visualization::viz_storage::VizStorage;
use euclid::{Point2D, Rect, Size2D};
use iced::Task;
use std::sync::Arc;
use std::time::Duration;

/// Core state update function implementing Iced's message-driven architecture.
///
/// Processes all application messages and updates the corresponding state components.
/// This is the central coordination point for the entire application, handling everything
/// from UI interactions to computation lifecycle management to interactive navigation.
///
/// # Arguments
///
/// * `state` - Mutable reference to the complete application state
/// * `message` - Event message to process (from user actions, timers, or system events)
///
/// # Returns
///
/// - `Task<Message>` for async operations (computation triggers, timers, progress updates)
/// - `Task::none()` for immediate state changes with no follow-up actions
///
/// # Message Categories
///
/// The function handles several categories of messages:
///
/// ## UI Control Messages
/// - **ToggleSidebar**: Show/hide the parameter control sidebar
/// - **Preset Management**: Mathematical region preset selection and application
///
/// ## Mathematical Parameter Messages
/// - **Coordinate Changes**: Real-time updates to complex plane boundaries
/// - **Resolution Changes**: Image width/height parameter updates
/// - **Iteration Limits**: Maximum computation depth configuration
///
/// ## Computation Lifecycle Messages
/// - **ComputeClicked**: Initialize and start new fractal computation
/// - **UpdateViz**: Periodic visualization refresh during computation
/// - **StopClicked**: Abort ongoing computation and cleanup resources
///
/// ## Visual Configuration Messages
/// - **Color Scheme Changes**: Gradient and iteration assignment updates
/// - **Render Settings**: Image scaling and presentation configuration
///
/// ## Interactive Navigation Messages
/// - **ShiftStage**: Apply panning translation to coordinate system
/// - **Zoom Operations**: Multi-stage zoom with timeout-based completion
///
/// # State Management Patterns
///
/// ## Resource Coordination
/// - Stops existing computation before starting new operations
/// - Maintains synchronization between computation and visualization storage
/// - Clears visual caches when parameters change
///
/// ## Async Task Scheduling
/// - Auto-triggers computation after parameter changes
/// - Schedules periodic visualization updates (20ms intervals)
/// - Implements zoom timeout detection (500ms delay)
///
/// ## Error Resilience
/// - Validates user input before applying parameter changes
/// - Gracefully handles invalid state transitions
/// - Provides fallback behavior for edge cases
pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::ToggleSidebar => state.viz.sidebar_visible = !state.viz.sidebar_visible,
        Message::PresetChanged(value) => state.viz.math_preset = value,
        Message::PresetClicked => {
            let data = &state.viz.math_preset.preset();
            state.math.area = data.coordinates();
            state.math.max_iteration = data.max_iteration();
            // Auto-trigger computation with preset parameters
            return Task::perform(async {}, |_| Message::ComputeClicked);
        }
        Message::LeftChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    Point2D::new(value, state.math.area.min_y()),
                    state.math.area.max(),
                ]);
            }
        }
        Message::RightChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    state.math.area.min(),
                    Point2D::new(value, state.math.area.max_y()),
                ]);
            }
        }
        Message::TopChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    state.math.area.min(),
                    Point2D::new(state.math.area.max_x(), value),
                ]);
            }
        }
        Message::BottomChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    Point2D::new(state.math.area.min_x(), value),
                    state.math.area.max(),
                ]);
            }
        }
        Message::WidthChanged(value) => {
            if let Ok(value) = value.parse::<u32>() {
                state.math.stage_size = Size2D::new(value, state.math.stage_size.height);
            }
        }
        Message::HeightChanged(value) => {
            if let Ok(value) = value.parse::<u32>() {
                state.math.stage_size = Size2D::new(state.math.stage_size.width, value);
            }
        }
        Message::MaxIterationChanged(value) => {
            if let Ok(value) = value.parse::<u32>() {
                state.math.max_iteration = value;
            }
        }
        Message::MaxIterationUpdateClicked => {
            if let Some(comp_storage) = state.comp_storage.as_ref() {
                // Stop existing computation before coordinate change
                if let Some(engine) = &state.engine {
                    engine.stop();
                }
                state.runtime.computing = false;

                // Create new storage with translated coordinates
                // This preserves any computed data that's still valid after translation
                let new_storage = comp_storage.as_ref().max_iteration_changed_clone(
                    comp_storage.as_ref().properties.max_iteration,
                    state.math.max_iteration,
                );

                // Rebuild complete computation pipeline with new coordinates
                state.comp_storage = Some(Arc::new(new_storage));
                state.engine = Some(MandelbrotEngine::new(&state.comp_storage.as_ref().unwrap()));
                state.storage = Some(VizStorage::new(state.comp_storage.as_ref().unwrap()));

                // Start computation and schedule visualization updates
                state.engine.as_ref().unwrap().start();
                state.runtime.canvas_cache.clear();
                return Task::perform(async {}, |_| Message::UpdateViz);
            }
        }
        Message::SaveImageClicked => {
            if let Some(savename) = super::file_save::show_save_file_dialog()
                && let Some(rawpixels) = super::pixels::create_pixels_from_app_state(&state)
            {
                super::file_save::write_image_png(savename, rawpixels);
            }
        }
        Message::ComputeClicked => {
            // Disable auto-computation to prevent loops
            state.viz.auto_start_computation = false;

            // Stop any existing computation to prevent resource conflicts
            if let Some(engine) = &state.engine {
                engine.stop();
            }
            state.runtime.computing = false;

            // Create new computation properties from validated parameters
            let comp_props = ImageCompProperties::new(
                StageProperties::new(state.math.area, state.math.stage_size),
                state.math.max_iteration,
            );

            // Initialize complete computation pipeline:
            // 1. CompStorage: Parallel-access computation data
            state.comp_storage = Some(Arc::new(CompStorage::new(comp_props)));
            // 2. MandelbrotEngine: Computation thread management
            state.engine = Some(MandelbrotEngine::new(&state.comp_storage.as_ref().unwrap()));
            // 3. VizStorage: Sequential-access visualization data
            state.storage = Some(VizStorage::new(&state.comp_storage.as_ref().unwrap()));

            // Start computation and reset visual state
            state.engine.as_ref().unwrap().start();
            state.runtime.canvas_cache.clear();

            // Schedule first visualization update
            return Task::perform(async {}, |_| Message::UpdateViz);
        }
        Message::UpdateViz => {
            // Process any pending computation events and update visualization
            if let Some(ref mut vizstorage) = state.storage {
                if vizstorage.process_events() {
                    // Clear canvas cache when new data arrives
                    state.runtime.canvas_cache.clear();
                }
            }

            // Check computation engine state and manage update cycle
            if let Some(engine) = &state.engine {
                let engine_state = engine.state();
                if engine_state == EngineState::Aborted || engine_state == EngineState::Finished {
                    // Computation completed - cleanup resources and stop updates
                    state.engine = None;
                    state.runtime.computing = false;
                    return Task::none(); // Stop update cycle
                } else {
                    // Computation still running - schedule next update in 20ms
                    return Task::perform(
                        async {
                            tokio::time::sleep(Duration::from_millis(20)).await;
                        },
                        |_| Message::UpdateViz,
                    );
                }
            }
        }
        Message::StopClicked => {
            if let Some(_) = state.engine {
                state.engine.as_ref().unwrap().stop();
                state.engine = None;
                state.runtime.computing = false;
            }
        }
        Message::ColorSchemeChanged(value) => {
            state.viz.gradient_color_preset = value;
            state.runtime.canvas_cache.clear();
        }
        Message::IterationAssignmentChanged(value) => {
            state.viz.iteration_assignment = value;
            state.runtime.canvas_cache.clear();
        }
        Message::RenderSchemeChanged(value) => {
            state.viz.render_scheme = value;
            state.runtime.canvas_cache.clear();
        }
        Message::RenderStripesChanged(value) => {
            if let Ok(value) = value.parse::<u32>() {
                state.viz.gradient_color_stripes = value;
                state.runtime.canvas_cache.clear();
            }
        }
        Message::RenderOffsetChanged(value) => {
            if let Ok(value) = value.parse::<u32>() {
                state.viz.gradient_color_offset = value;
                state.runtime.canvas_cache.clear();
            }
        }
        Message::ShiftStage(offset) => {
            // Stop existing computation before coordinate change
            if let Some(engine) = &state.engine {
                engine.stop();
            }
            state.runtime.computing = false;

            // Create new storage with translated coordinates
            // This preserves any computed data that's still valid after translation
            let new_storage = state
                .comp_storage
                .as_ref()
                .unwrap()
                .as_ref()
                .shifted_clone_by_pixels(offset);

            // Update UI coordinate display to reflect new mathematical region
            state.math.area = new_storage.original_properties.stage_properties.coo;

            // Rebuild complete computation pipeline with new coordinates
            state.comp_storage = Some(Arc::new(new_storage));
            state.engine = Some(MandelbrotEngine::new(&state.comp_storage.as_ref().unwrap()));
            state.storage = Some(VizStorage::new(state.comp_storage.as_ref().unwrap()));

            // Start computation and schedule visualization updates
            state.engine.as_ref().unwrap().start();
            state.runtime.canvas_cache.clear();
            return Task::perform(async {}, |_| Message::UpdateViz);
        }
        Message::ZoomStart((origin, ticks)) => {
            state.runtime.zoom = Some(ZoomState::start(origin, ticks));
            state.runtime.canvas_cache.clear();
        }
        Message::ZoomTick(ticks_offset) => {
            if ticks_offset != 0
                && let Some(zoom) = &mut state.runtime.zoom
            {
                zoom.update_ticks(ticks_offset);
                state.runtime.canvas_cache.clear();
            }
        }
        Message::ZoomEndCheck => {
            if let Some(zoom) = &state.runtime.zoom
                && zoom.is_timeout(Duration::from_millis(500))
            {
                // Zoom timeout reached - apply accumulated changes
                if zoom.ticks != 0 {
                    // Stop existing computation before coordinate transformation
                    if let Some(engine) = &state.engine {
                        engine.stop();
                    }
                    state.runtime.computing = false;

                    // Create new storage with zoomed coordinates
                    // Preserves computed data that remains valid after zoom
                    let new_storage = state
                        .comp_storage
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .zoomed_clone_by_pixels(
                            Point2D::new(zoom.origin.x as i32, zoom.origin.y as i32),
                            zoom.factor,
                        );

                    // Update UI coordinate display for new mathematical region
                    state.math.area = new_storage.original_properties.stage_properties.coo;

                    // Rebuild computation pipeline with new coordinates
                    state.comp_storage = Some(Arc::new(new_storage));
                    state.engine =
                        Some(MandelbrotEngine::new(&state.comp_storage.as_ref().unwrap()));
                    state.storage = Some(VizStorage::new(state.comp_storage.as_ref().unwrap()));

                    // Start computation and schedule updates
                    state.engine.as_ref().unwrap().start();
                    state.runtime.canvas_cache.clear();
                    state.runtime.zoom = None;
                    return Task::perform(async {}, |_| Message::UpdateViz);
                }
                // No zoom changes - just clear zoom state
                state.runtime.zoom = None;
            }
        }
        Message::MousePressed(_point) => {}
        Message::MouseDragged(_point) => {}
        Message::MouseReleased(_point) => {}
    }
    Task::none()
}

// end of file
