//! Color mapping system for fractal visualization.
//!
//! This module provides a comprehensive color mapping system that transforms
//! fractal iteration counts into visually appealing colors. The system supports
//! gradient-based color schemes with mathematical transformation functions to
//! create diverse visual representations of fractal data.
//!
//! # Architecture
//!
//! ```text
//! Iteration Count → Assignment Function → Color Index → Gradient Lookup → RGB Color
//!       ↓                  ↓                ↓             ↓              ↓
//!   Raw compute      Mathematical      Palette         Color           Final
//!   result           transformation    position        interpolation   pixel
//! ```
//!
//! # Color Pipeline
//!
//! ## 1. Mathematical Assignment
//! Raw iteration counts are transformed using mathematical functions:
//! - **Linear**: Direct mapping (x → x)
//! - **Power Functions**: Squared (x²), cubic (x³) for emphasis
//! - **Root Functions**: Square root (√x), cube root (∛x) for compression
//! - **Logarithmic**: Natural log for extreme value handling
//!
//! ## 2. Gradient Interpolation
//! Transformed values are mapped to colors using gradient schemes:
//! - **Anchor Colors**: Key colors defining the gradient
//! - **Linear Interpolation**: Smooth transitions between anchors
//! - **Cyclic Gradients**: Wrapping behavior for infinite iteration ranges
//!
//! ## 3. Special Cases
//! - **Body Color**: Special color for points that never escape (iteration = max)
//! - **Modulo Wrapping**: Handles iteration counts exceeding gradient length
//!
//! # Design Benefits
//!
//! - **Mathematical Variety**: Multiple assignment functions reveal different patterns
//! - **Visual Appeal**: Professional gradient schemes optimized for fractal visualization
//! - **Performance**: Pre-computed interpolation tables for fast lookup
//! - **Flexibility**: Easy to add new color schemes and assignment functions
//!
//! # Usage Patterns
//!
//! The coloring system is typically used in a two-stage process:
//! 1. **Setup**: Create `GradientColors` from a `GradientColorScheme`
//! 2. **Rendering**: Call `iteration_to_color()` for each pixel during visualization

/// Core color mapping data structures and algorithms
pub mod base;

/// Pre-defined color schemes and mathematical assignment functions
pub mod presets;
