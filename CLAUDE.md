# Claude Development Context

## Project Overview
This is mandel.rs - a Rust learning project focused on building a fractal graphics visualizer for the Mandelbrot set. The project serves dual purposes: creating functional fractal visualization software and providing hands-on Rust learning experience.

## Development Philosophy
**CRITICAL**: This is an educational project where the human is learning Rust. The human must type all code themselves. Claude should:
- Explain concepts and approaches
- Review and check written code
- Point out mistakes or design issues
- Suggest improvements
- **NEVER write code directly** - only provide guidance and explanations

## Project Architecture
The application follows a three-layer design:

1. **Storage**: ✅ 2D array storing rich fractal data (not just iteration depth)
2. **Computation**: ✅ Fractal calculation algorithms (starting simple, optimizing later)
3. **Visualization**: ✅ Interactive GUI with real-time fractal rendering

### Current Implementation Status

**Storage Layer (Complete)**
- `DataPoint` struct: Stores iteration count, final x/y coordinates
- `DataPlane` struct: 2D array operations with coordinate transformation
- `DataStorage` struct: High-level container with computation metadata
- Coordinate bounds (x_min, x_max, y_min, y_max) and max_iteration configuration
- Immutable design with proper mutability access (plane() / plane_mut())
- Option<DataPoint> for handling uncomputed points

**Computation Layer (Complete)**
- `simple_mandelbrot.rs`: Core Mandelbrot set algorithm
- `data_point_at()`: Single complex number computation with escape detection
- `compute_mandelbrot()`: Full storage computation with coordinate transformation
- Direct complex number math (z² + c) without external dependencies
- Proper handling of max iterations and final escape coordinates
- Verified working algorithm with edge/center point testing

**Visualization Layer (Complete - Migrated to iced)**
- `mandel_iced_app.rs`: Interactive GUI application using iced MVU architecture
- Model-View-Update (MVU) pattern with pure functional UI description
- Event-driven architecture with Message enum for all user interactions
- Real-time fractal computation triggered by button clicks
- Dynamic color mapping from iteration counts to RGB values
- Cross-platform window with native look and feel and responsive image scaling
- High-resolution rendering with configurable dimensions and instant visual feedback
- Proper state management (computing vs idle states)
- Interactive parameter input: coordinate bounds (left, right, top, bottom), image dimensions (width, height), and max iterations
- Real-time parameter validation using Rust's Result pattern matching
- Improved layout design with centered alignment and consistent spacing
- Fixed-width input fields (100px) for uniform appearance
- Automatic initial computation on application startup using Command::perform
- Background computation infrastructure with tokio channels (in progress)
- Arc<DataStorage> for memory-efficient shared ownership of computation results
- ComputeProgress struct for statistical progress reporting without data duplication

**Project Structure**
```
src/
├── main.rs              # Entry point launching GUI application
├── data_point.rs        # DataPoint struct with constructor and getters
├── data_plane.rs        # 2D array operations and indexing
├── data_storage.rs      # High-level storage with computation metadata
├── simple_mandelbrot.rs # Mandelbrot computation algorithm
└── mandel_iced_app.rs   # Interactive GUI with fractal visualization (iced MVU)
```

## Technical Context
- Language: Rust (educational focus)
- Development: VS Code with Rust Extension Pack
- GUI Framework: iced for cross-platform native applications with MVU architecture
- Version Control: Git repository
- Target Platform: Linux/Ubuntu (system-independent design)
- Human Background: Experienced Java programmer learning Rust

## Major Achievement: Complete Functional Application
**Accomplished comprehensive Rust development:**
- Built complete fractal visualizer from scratch
- Mastered Rust's ownership system and advanced concepts
- Created interactive GUI application with real-time graphics
- Successfully migrated from egui to iced, learning MVU architecture
- Implemented mathematical algorithms and coordinate transformations
- Achieved high-resolution (800×800) fractal rendering with custom coloring
- Demonstrated architecture independence by reusing business logic across UI frameworks

## Development Commands
- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Key Rust Concepts Learned
- **Ownership & Borrowing**: `&self` vs `&mut self`, moving vs borrowing data
- **Option<T>**: Null safety with explicit handling of "maybe no value"
- **Derive Macros**: Auto-generating Debug, Clone, PartialEq implementations
- **Vec Memory Management**: Capacity vs length, proper initialization
- **Borrow Checker**: Preventing data races at compile time, separating immutable/mutable borrows
- **Module System**: Declaring (`mod`) vs importing (`use`), cross-module function calls
- **Testing**: Built-in unit testing with `#[test]` and `cargo test`
- **References vs Values**: When to return `&DataPoint` vs `DataPoint`
- **Mathematical Algorithms**: Implementing complex number operations without external libraries
- **Coordinate Transformations**: Mapping between pixel space and mathematical coordinate systems
- **Struct Design**: Separating concerns (DataStorage metadata vs DataPlane operations)
- **GUI Programming**: iced framework, MVU architecture, event handling, state management
- **Graphics Programming**: Color mapping, image rendering with RGBA format, real-time rendering
- **Trait Implementation**: Implementing `iced::Application` for MVU-based application behavior
- **Functional Reactive Programming**: Pure functions for UI description, message-driven updates
- **Architecture Patterns**: Model-View-Update (MVU) pattern, separation of concerns
- **Cargo Dependencies**: Adding external crates and managing project dependencies
- **Advanced Pattern Matching**: Using `if let` with tuple destructuring for multi-field validation
- **Result Type Handling**: Parsing user input with `.parse()` and handling success/error cases
- **String Manipulation**: Converting between String and numeric types with proper error handling
- **Layout Management**: iced alignment system, horizontal spacing, fixed-width components
- **UI Design Patterns**: Center-aligned layouts, consistent spacing, visual balance
- **Command System**: Using Command::perform for initial actions and async message dispatch
- **Async/Concurrent Programming**: Background computation with tokio channels and Arc for shared ownership
- **Progress Reporting**: ComputeProgress struct with completion ratios for real-time updates

## Communication Guidelines
- Explain concepts in Java terms when helpful
- Use English for all technical communication
- Focus on educational value over speed
- Encourage hands-on typing and understanding