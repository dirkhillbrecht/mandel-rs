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
3. **Visualization**: 📋 Display layer (local GUI first, web interface later)

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

**Project Structure**
```
src/
├── main.rs              # Entry point with working Mandelbrot demonstration
├── data_point.rs        # DataPoint struct with constructor and getters
├── data_plane.rs        # 2D array operations and indexing
├── data_storage.rs      # High-level storage with computation metadata
└── simple_mandelbrot.rs # Mandelbrot computation algorithm
```

## Technical Context
- Language: Rust (educational focus)
- Development: VS Code with Rust Extension Pack
- Version Control: Git repository
- Target Platform: Linux/Ubuntu (system-independent design)
- Human Background: Experienced Java programmer learning Rust

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

## Communication Guidelines
- Explain concepts in Java terms when helpful
- Use English for all technical communication
- Focus on educational value over speed
- Encourage hands-on typing and understanding