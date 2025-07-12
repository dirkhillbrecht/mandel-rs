//! Mandel.rs - Interactive Fractal Visualization in Rust
//!
//! A comprehensive educational fractal visualization application built in Rust,
//! designed to demonstrate advanced systems programming concepts while providing
//! an interactive platform for exploring the mathematical beauty of fractals.
//!
//! # Project Overview
//!
//! Mandel.rs serves dual purposes as both a functional fractal visualization tool
//! and an educational codebase for learning Rust programming concepts. The project
//! showcases real-world application of Rust's key features including memory safety,
//! concurrency, type safety, and performance optimization.
//!
//! ## Educational Mission
//!
//! This project is specifically designed as a **Rust learning experience**, where:
//! - **Hands-on Learning**: The human types all code to build muscle memory
//! - **Concept Exploration**: Each feature demonstrates core Rust programming patterns
//! - **Real-world Application**: Shows how Rust concepts apply to substantial projects
//! - **Progressive Complexity**: Builds from simple concepts to advanced architectures
//!
//! # Architecture Overview
//!
//! ## Three-Layer Design
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────────────┐
//! │                           GUI Layer (Iced Framework)                           │
//! │  ┌─────────────────────┬─────────────────────┬─────────────────────────────────┤
//! │  │   Parameter Control │   Interactive Canvas│        Message System         │
//! │  │   - Presets        │   - Pan/Zoom        │   - Event-driven updates      │
//! │  │   - Coordinates    │   - Real-time render │   - State management          │
//! │  │   - Visual config  │   - Progress display │   - Async coordination        │
//! │  └─────────────────────┴─────────────────────┴─────────────────────────────────┤
//! └─────────────────────────────────────────────────────────────────────────────────┘
//!                                      ↕ Messages & State
//! ┌─────────────────────────────────────────────────────────────────────────────────┐
//! │                        Storage Layer (Dual Architecture)                       │
//! │  ┌─────────────────────────────────┬─────────────────────────────────────────────┤
//! │  │     Computation Storage         │         Visualization Storage              │
//! │  │  ┌─────────────────────────────┬─────────────────────────────────────────────┤
//! │  │  │  • Thread-safe access      │  • Sequential access optimization          │
//! │  │  │  • Per-pixel RwLocks        │  • Event-driven synchronization            │
//! │  │  │  • Parallel algorithms      │  • UI-optimized data structures            │
//! │  │  │  • Real-time progress       │  • Cache-friendly iteration                │
//! │  │  └─────────────────────────────┴─────────────────────────────────────────────┤
//! │  └─────────────────────────────────────────────────────────────────────────────│
//! └─────────────────────────────────────────────────────────────────────────────────┘
//!                                      ↕ Computation Events
//! ┌─────────────────────────────────────────────────────────────────────────────────┐
//! │                       Computation Layer (Parallel Engine)                      │
//! │  ┌─────────────────────┬─────────────────────┬─────────────────────────────────┤
//! │  │  Mandelbrot Engine  │   Mathematical Core │     Color Mapping System       │
//! │  │  - Thread pool      │   - Complex numbers │   - Gradient interpolation     │
//! │  │  - Work distribution│   - Escape algorithms│   - Assignment functions       │
//! │  │  - Progress tracking│   - Coordinate xforms│   - Real-time color lookup     │
//! │  │  - Resource cleanup │   - Precision control│   - Performance optimization   │
//! │  └─────────────────────┴─────────────────────┴─────────────────────────────────┤
//! └─────────────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Key Rust Concepts Demonstrated
//!
//! ## Memory Safety & Ownership
//! - **Zero-copy Architecture**: Efficient data sharing without unnecessary allocations
//! - **RAII Pattern**: Automatic resource cleanup using Drop trait
//! - **Smart Pointers**: Arc<T> for shared ownership in concurrent contexts
//! - **Lifetime Management**: Careful coordination of data lifetime across threads
//!
//! ## Concurrency & Parallelism
//! - **Thread-safe Design**: RwLock<T> for fine-grained concurrent access
//! - **Async Programming**: Tokio-based event handling and time management
//! - **Channel Communication**: Producer-consumer patterns for event streaming
//! - **Lock-free Algorithms**: Efficient coordination without blocking
//!
//! ## Type Safety & Abstractions
//! - **Coordinate Systems**: Type-safe coordinate transformations using euclid
//! - **Error Handling**: Result<T, E> for robust error propagation
//! - **Option Types**: Safe handling of nullable data throughout
//! - **Strong Typing**: Preventing coordinate system confusion and unit errors
//!
//! ## Performance Engineering
//! - **Cache-friendly Design**: Memory layout optimized for access patterns
//! - **Lazy Evaluation**: Computation triggered only when needed
//! - **Efficient Data Structures**: Optimized for specific use cases
//! - **Progressive Rendering**: Immediate feedback with incremental improvement
//!
//! # Application Features
//!
//! ## Mathematical Capabilities
//! - **Mandelbrot Set**: Classic fractal with infinite detail and boundary complexity
//! - **Arbitrary Precision**: High-resolution coordinate system for deep exploration
//! - **Parameter Control**: Real-time adjustment of mathematical parameters
//! - **Preset Regions**: Curated collection of mathematically interesting areas
//!
//! ## Interactive Exploration
//! - **Pan Navigation**: Drag to translate coordinate system
//! - **Zoom Operations**: Mouse wheel with timeout-based commit system
//! - **Real-time Preview**: Immediate visual feedback during operations
//! - **Progressive Quality**: Computation continues while exploring
//!
//! ## Visual Customization
//! - **Color Schemes**: Professional gradient schemes for aesthetic appeal
//! - **Mathematical Mapping**: Various functions for iteration-to-color transformation
//! - **Render Modes**: Different scaling and presentation options
//! - **Real-time Updates**: Instant visual feedback for all parameter changes
//!
//! # Module Organization
//!
//! ## `comp` - Computation Engine
//! Mathematical fractal computation with parallel processing:
//! - **`mandelbrot_engine`**: Thread pool management and work distribution
//! - **`math_data`**: Mathematical region definitions and presets
//!
//! ## `storage` - Data Management
//! Sophisticated dual-storage architecture for performance:
//! - **`computation`**: Thread-safe parallel access storage systems
//! - **`visualization`**: Sequential access optimization for UI operations
//! - **`event`**: Real-time synchronization between storage systems
//!
//! ## `gui` - User Interface
//! Modern GUI built with Iced framework:
//! - **`iced/app`**: Application state and lifecycle management
//! - **`iced/view`**: UI layout and widget composition
//! - **`iced/update`**: Event handling and state transitions
//!
//! # Development Philosophy
//!
//! ## Educational First
//! Every architectural decision prioritizes learning value:
//! - **Clear Separation**: Distinct layers with well-defined responsibilities
//! - **Explicit Design**: Favors clarity over clever abstractions
//! - **Documentation**: Comprehensive explanations of design decisions
//! - **Progressive Complexity**: Builds understanding incrementally
//!
//! ## Production Quality
//! Despite educational focus, maintains professional standards:
//! - **Error Handling**: Robust handling of edge cases and invalid input
//! - **Performance**: Efficient algorithms and data structures
//! - **User Experience**: Responsive interface with immediate feedback
//! - **Code Quality**: Clean, maintainable, and well-tested implementation
//!
//! # Getting Started
//!
//! Run the application with:
//! ```bash
//! cargo run --release
//! ```
//!
//! The `--release` flag is recommended for optimal fractal computation performance.

/// Application modules organized by architectural layer
mod comp;     // Computation engine and mathematical algorithms
mod gui;      // User interface and event handling
mod storage;  // Data storage and synchronization systems

/// Application entry point - launches the Iced GUI application.
///
/// Initializes the complete application stack including GUI framework,
/// computation engine, and storage systems. The application uses Iced's
/// built-in event loop and window management for cross-platform compatibility.
///
/// # Returns
///
/// `iced::Result` indicating success or failure of application initialization
///
/// # Application Lifecycle
///
/// 1. **Iced Initialization**: Window creation and event loop setup
/// 2. **Application State**: Initial state construction with default parameters  
/// 3. **GUI Rendering**: Initial UI layout and widget tree creation
/// 4. **Event Loop**: Message-driven state updates and re-rendering
/// 5. **Graceful Shutdown**: Resource cleanup and window closure
///
/// # Error Handling
///
/// Returns `iced::Result` which handles:
/// - **Window Creation Failures**: Graphics context or windowing system issues
/// - **Resource Initialization**: Memory allocation or system resource problems
/// - **Platform Compatibility**: OS-specific initialization requirements
///
/// # Performance Notes
///
/// - Built with `--release` for optimal computation performance
/// - Uses native GUI rendering for responsive user interface
/// - Leverages all available CPU cores for fractal computation
/// - Implements efficient memory management for large datasets
fn main() -> iced::Result {
    gui::iced::app::launch()
}

// end of file
