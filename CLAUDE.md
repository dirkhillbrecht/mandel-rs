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
Storage is a two-sided system: A CompStorage allows highly parallel access to the computed data
with higher general access costs. It is for the computation algorithm. The VizStorage allows
only sequential access to the data. It is used for the visualization part. An event system
passes changes from CompStorage to VizStorage. This keeps VizStorage up to date while CompStorage
is left alone for (almost) exclusive access by the computation algorithm.

Computation is kept rather simple at the moment: It computes points of the Mandelbrot set and the escape depth
of the points surrounding it. Currently, it is a non-parallel simple and straight implementation
using some randomisation of the computed points.

Visualization is an interface implemented using Iced 0.13. It allows to enter all data for computation
and controls the computation engine. Currently it allows to drag the computed image and update the computation.

## Technical Context
- Language: Rust (educational focus)
- Development: VS Code with Rust Extension Pack, auto-format on save with rustfmt
- GUI Framework: iced 0.13.1 for cross-platform native applications with function-based API
- Color Science: palette 0.7.6 for professional color space operations and gradient generation
- Coordinate System: euclid for type-safe mathematical coordinate handling
- Async Runtime: tokio for non-blocking operations and enhanced threading
- Streaming: async-stream for creating finite event streams
- Version Control: Git repository
- Target Platform: Linux/Ubuntu (system-independent design)
- Human Background: Experienced Java programmer learning Rust

## Project organisation
Project development is organized through so-called "manifestos". They describe certain development
targets. We are currently working in [manifesto 03](manifestos/manifesto-03-cleanup-and-mvp.md) on issue 7.
While panning is already implemented, zooming is waiting for implementation.

## Communication Guidelines
- Explain concepts in Java terms when helpful
- Use English for all technical communication
- Focus on educational value over speed
- Encourage hands-on typing and understanding