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

1. **Storage**: 2D array storing rich fractal data (not just iteration depth)
2. **Computation**: Fractal calculation algorithms (starting simple, optimizing later)
3. **Visualization**: Display layer (local GUI first, web interface later)

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

## Communication Guidelines
- Explain concepts in Java terms when helpful
- Use English for all technical communication
- Focus on educational value over speed
- Encourage hands-on typing and understanding