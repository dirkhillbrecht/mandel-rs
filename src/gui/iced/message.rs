//! Message-passing system for the Iced GUI application.
//!
//! This module defines the complete set of messages that can be passed through
//! the Iced event system to coordinate between UI components, user interactions,
//! and application state changes. The message system serves as the central
//! communication hub for the entire application.
//!
//! # Architecture
//!
//! ## Message Flow
//!
//! ```text
//! User Interaction → UI Widget → Message → Update Function → State Change
//! ```
//!
//! ## Message Categories
//!
//! - **UI Controls**: Sidebar, settings, and configuration changes
//! - **Mathematical Parameters**: Coordinate area, iteration limits, presets
//! - **Computation Control**: Start, stop, and progress tracking
//! - **Visual Settings**: Color schemes, rendering options
//! - **Interactive Navigation**: Panning, zooming, mouse interactions
//! - **System Events**: Timers, async updates, internal coordination
//!
//! # Event-Driven Architecture
//!
//! The application uses Iced's message-driven architecture where:
//! 1. **Events** trigger message creation
//! 2. **Messages** are processed by update functions
//! 3. **State changes** trigger UI re-rendering
//! 4. **Side effects** (computation, file I/O) are handled via commands
//!
//! This design ensures predictable state management and clean separation
//! between UI presentation and application logic.

use euclid::Vector2D;
use iced::Point;

use crate::{
    comp::math_data::MathPreset,
    gui::iced::app::ImageRenderScheme,
    storage::{
        coord_spaces::StageSpace,
        visualization::coloring::presets::{GradientColorPreset, IterationAssignment},
    },
};

/// Complete message enumeration for the fractal visualization application.
///
/// Represents all possible events and state changes that can occur in the
/// application. Each message variant corresponds to a specific user action,
/// system event, or internal state transition.
///
/// # Design Principles
///
/// - **Exhaustive**: Covers all possible application events
/// - **Typed**: Each message carries appropriately typed data
/// - **Cloneable**: Supports Iced's message-passing requirements
/// - **Debuggable**: Enables easy troubleshooting of event flow
///
/// # Message Processing
///
/// All messages are processed by the central `update()` function which:
/// - Updates application state based on message type
/// - Triggers side effects (computation, rendering)
/// - Returns commands for async operations
///
/// # Thread Safety
///
/// Messages are designed to be sent across thread boundaries, enabling
/// communication between computation threads and the UI thread.
#[derive(Debug, Clone)]
pub enum Message {
    // === UI Control Messages ===
    /// Toggle visibility of the control sidebar
    /// Triggered by: Sidebar toggle button
    ToggleSidebar,

    // === Mathematical Parameter Messages ===
    /// Mathematical preset selection changed
    /// Triggered by: Preset dropdown selection
    /// Data: New preset (MandelbrotFull, JuliaSet, etc.)
    PresetChanged(MathPreset),

    /// Apply current preset to coordinate fields
    /// Triggered by: "Apply Preset" button click
    PresetClicked,

    /// Image width in pixels changed
    /// Triggered by: Width text input
    /// Data: New width value as string
    WidthChanged(String),

    /// Image height in pixels changed
    /// Triggered by: Height text input
    /// Data: New height value as string
    HeightChanged(String),

    /// Maximum iteration count changed
    /// Triggered by: Max iteration text input
    /// Data: New iteration limit as string
    MaxIterationChanged(String),

    // === Computation Control Messages ===
    /// Update maximum iteration of the image
    MaxIterationUpdateClicked,

    /// Start fractal computation
    /// Triggered by: "Compute" button click
    ComputeClicked,

    /// Stop ongoing computation
    /// Triggered by: "Stop" button click
    StopClicked,

    /// Save the content of the current image to the save file
    SaveImageClicked,

    /// Update visualization with new data
    /// Triggered by: Async computation progress events
    UpdateViz,

    // === Visual Configuration Messages ===
    /// Color gradient scheme changed
    /// Triggered by: Color scheme dropdown
    /// Data: New color preset (Sunrise, Ocean, etc.)
    ColorSchemeChanged(GradientColorPreset),

    /// Iteration-to-color mapping function changed
    /// Triggered by: Iteration assignment dropdown
    /// Data: New assignment function (Linear, Logarithmic, etc.)
    IterationAssignmentChanged(IterationAssignment),

    /// Image rendering scheme changed
    /// Triggered by: Render scheme dropdown
    /// Data: New rendering mode (Cropped, Fitted, Centered)
    RenderSchemeChanged(ImageRenderScheme),

    /// Number of stripes to use for rendering changed
    RenderStripesChanged(String),

    /// Offset for stripe selection changed
    RenderOffsetChanged(String),

    // === Interactive Navigation Messages ===
    /// Drag operation in the FractalCanvas started
    /// Needed so that the app state can update itself correctly
    ShiftStageStart,

    /// Coordinate system shift completed
    /// Triggered by: Canvas drag operation completion
    /// Data: Pixel offset vector for coordinate translation
    ShiftStage(Vector2D<i32, StageSpace>),

    /// Zoom operation initiated
    /// Triggered by: First mouse wheel scroll
    /// Data: (zoom origin pixel, initial scroll ticks)
    ZoomStart((Point, i32)),

    /// Additional zoom input received
    /// Triggered by: Subsequent mouse wheel scrolls during zoom
    /// Data: Additional scroll ticks to accumulate
    ZoomTick(i32),

    /// Check if zoom operation should complete
    /// Triggered by: Timer subscription (every ~50ms during zoom)
    ZoomEndCheck,

    // === Mouse Event Messages (Currently Unused) ===
    /// Mouse button pressed on canvas
    /// Status: Implemented in canvas event handling instead
    #[allow(dead_code)]
    MousePressed(iced::Point),

    /// Mouse dragged across canvas
    /// Status: Implemented in canvas event handling instead
    #[allow(dead_code)]
    MouseDragged(iced::Point),

    /// Mouse button released on canvas
    /// Status: Implemented in canvas event handling instead
    #[allow(dead_code)]
    MouseReleased(iced::Point),
}

// end of file
