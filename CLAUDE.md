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
**Storage System**: Dual-storage architecture with CompStorage for parallel computation access and VizStorage for sequential visualization. An event system synchronizes changes between them, keeping VizStorage current while allowing CompStorage exclusive access for computation algorithms.

**Computation Engine**: Simple, direct implementation computing Mandelbrot set points and escape depths with randomized point selection. Currently non-parallel but designed for future optimization.

**Visualization Interface**: Iced 0.13-based GUI providing computation controls and interactive fractal rendering. Features implemented:
- ✅ **Panning**: Interactive dragging with deferred coordinate updates
- 🚧 **Zooming**: Partial implementation with mouse wheel events and `2^(0.1*ticks)` scaling formula

### Interactive Controls Architecture
The canvas uses a state-based operation system:
- `CanvasOperation::Idle` - Default state, ready for new interactions
- `CanvasOperation::Drag` - Active panning with visual feedback
- `CanvasOperation::Zoom` - Accumulating zoom operations (in progress)

During operations, visual transformations are applied to VizStorage data without recomputation. Only when operations complete are new coordinates calculated for CompStorage, triggering fresh fractal computation.

## Technical Context
- **Language**: Rust (educational focus)
- **Development**: VS Code with Rust Extension Pack, auto-format on save with rustfmt
- **GUI Framework**: iced 0.13.1 for cross-platform native applications with function-based API
- **Color Science**: palette 0.7.6 for professional color space operations and gradient generation
- **Coordinate System**: euclid for type-safe mathematical coordinate handling
- **Async Runtime**: tokio for non-blocking operations and enhanced threading
- **Streaming**: async-stream for creating finite event streams
- **Version Control**: Git repository
- **Target Platform**: Linux/Ubuntu (system-independent design)
- **Human Background**: Experienced Java programmer learning Rust

## Current Development Status
Project development is organized through [manifestos](manifestos/). Currently working on **[Manifesto 03](manifestos/manifesto-03-cleanup-and-mvp.md): Issue 7 - Interactive Area Selection**.

**Recent Progress**:
- ✅ Panning implementation complete with deferred coordinate updates
- 🚧 Zoom event handling implemented with wheel scroll detection
- 🚧 Zoom factor calculation using `2^(0.1*wheel_ticks)` formula
- ⏳ Zoom timeout detection for operation completion (architecture decision pending)

**Next Steps**:
- Resolve zoom timeout detection pattern (canvas vs app state management)
- Implement zoom coordinate transformation and CompStorage updates
- Add zoom visual feedback during operation
- Preserve computed data during zoom operations where possible

## Communication Guidelines
- Explain concepts in Java terms when helpful
- Use English for all technical communication
- Focus on educational value over speed
- Encourage hands-on typing and understanding