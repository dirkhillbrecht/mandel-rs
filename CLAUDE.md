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
2. **Computation**: 🚧 Fractal calculation algorithms (starting simple, optimizing later)
3. **Visualization**: 📋 Display layer (local GUI first, web interface later)

### Current Implementation Status

**Storage Layer (Complete)**
- `DataPoint` struct: Stores iteration count, final x/y coordinates
- `DataStorage` struct: 2D array using single Vec with coordinate transformation
- Immutable design with private fields and getter methods
- Option<DataPoint> for handling uncomputed points
- Full unit test coverage

**Project Structure**
```
src/
├── main.rs           # Entry point and module declarations
├── data_point.rs     # DataPoint struct with constructor and getters
└── data_storage.rs   # DataStorage with 2D indexing and tests
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
- **Borrow Checker**: Preventing data races at compile time
- **Module System**: Separating code into logical modules (data_point.rs, data_storage.rs)
- **Testing**: Built-in unit testing with `#[test]` and `cargo test`
- **References vs Values**: When to return `&DataPoint` vs `DataPoint`

## Communication Guidelines
- Explain concepts in Java terms when helpful
- Use English for all technical communication
- Focus on educational value over speed
- Encourage hands-on typing and understanding