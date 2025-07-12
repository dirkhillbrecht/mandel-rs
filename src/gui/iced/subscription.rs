//! Async subscription system for time-based and background operations.
//!
//! This module implements Iced's subscription system to handle asynchronous
//! operations that need to run in the background and periodically send messages
//! to the application. Subscriptions enable reactive programming patterns
//! and time-based behaviors.
//!
//! # Subscription Types
//!
//! ## Auto-Computation Trigger
//! When `auto_start_computation` is enabled, immediately triggers computation
//! startup. This provides seamless user experience by automatically beginning
//! fractal calculation when parameters change.
//!
//! ## Zoom Timeout Detection
//! During zoom operations, runs a periodic timer to detect when zoom input
//! has stopped. This enables the "zoom with timeout" interaction pattern
//! where accumulated scroll events are committed after a pause.
//!
//! # Architecture
//!
//! ```text
//! Application State → subscription() → Async Stream → Messages
//!       ↓                    ↓              ↓         ↓
//!   Conditions →         Selection →     Timer/Event → Update Loop
//! ```
//!
//! # Design Patterns
//!
//! - **Conditional Subscriptions**: Only active when needed
//! - **Single Responsibility**: Each subscription handles one concern
//! - **Resource Efficiency**: Inactive subscriptions consume no resources
//! - **Reactive Updates**: State changes automatically adjust subscriptions
//!
//! # Performance
//!
//! Subscriptions are lightweight async streams that only exist when required.
//! The zoom timer runs at 20Hz (50ms intervals) for responsive interaction
//! detection without excessive CPU usage.

use std::time::Duration;

use crate::gui::iced::app::AppState;
use crate::gui::iced::message::Message;

/// Creates the appropriate subscription based on current application state.
///
/// Analyzes the application state to determine which async operations are
/// needed and returns the corresponding subscription. This function is called
/// by Iced on every state change to update the active subscription set.
///
/// # State-Driven Selection
///
/// The subscription type is determined by application state priority:
/// 1. **Auto-computation**: Highest priority for immediate startup
/// 2. **Zoom timeout**: Active during zoom operations
/// 3. **None**: Default state with no background operations
///
/// # Arguments
///
/// * `state` - Current application state to analyze
///
/// # Returns
///
/// - **Auto-computation subscription**: Single `ComputeClicked` message
/// - **Zoom timer subscription**: Periodic `ZoomEndCheck` messages (20Hz)
/// - **No subscription**: When no background operations are needed
///
/// # Subscription Lifecycle
///
/// Subscriptions are automatically created/destroyed as state changes:
/// - Starting auto-computation disables other subscriptions
/// - Beginning zoom creates timer subscription
/// - Completing operations returns to no subscription
///
/// # Performance Impact
///
/// - **Auto-computation**: Single message, immediate termination
/// - **Zoom timer**: Minimal CPU (50ms sleep cycles)
/// - **None**: Zero overhead
pub fn subscription(state: &AppState) -> iced::Subscription<Message> {
    if state.viz.auto_start_computation {
        // Auto-computation: Trigger immediate computation startup
        // This subscription sends a single message and then terminates
        iced::Subscription::run(|| {
            async_stream::stream! {
                yield Message::ComputeClicked;
            }
        })
    } else if state.runtime.zoom.is_some() {
        // Zoom timeout detection: Periodic timer during zoom operations
        // Runs at 20Hz (50ms intervals) to detect when zoom input stops
        iced::Subscription::run(|| {
            async_stream::stream! {
                loop {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    yield Message::ZoomEndCheck;
                }
            }
        })
    } else {
        // No active subscription: Default state with no background operations
        iced::Subscription::none()
    }
}

// end of file
