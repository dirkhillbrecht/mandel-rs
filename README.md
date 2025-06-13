# Mandel.rs - Learning the Rust language writing a fractal graphics visualizer with the help of AI

This project implements a fractal graphics program from scratch in Rust. I use this to

1. Learn Rust
2. Write a fractal graphics program I always wanted to have but could not find so far
3. Use AI help for this endeavour.

The AI I start with in "Claude Code". I record the sessions with the AI and store them in the `session-protocols` folder. Together with the commits of the projects these allow to follow how communication with the AI worked.

It is important to inform the AI about what the plan is and how implementation should take place. I do this by writing "manifestos" before starting an actual implementation. This formalizes a step every programmer should do before coding start: Thinking about what is to be developed and what the goals are. You find these manifestos in the [manifestos subdirectory](manifestos/). Note that the [first manifesto](manifestos/manifesto-01-initial-project-master-plan.md) has been moved there retroactively; in the beginning it has been the project's README.

As co-working with an AI is also an important part of this project, I record the AI session and [store these records](session-protocols/) within the project. Hopefully, this gives some insights on how working with an AI on a software project is mid-2025.

## Features of this program

While this is in the first place a Rust learning project, I have a clear vision about what this program should do once it is finished:

* Compute fractal graphics with efficient algorithms
* Visualize them _independently_ of computation, i.e. colors and color assignments to iteration depths can be changed _without_ recomputation
* Continuous update of visualization during computation
* Local GUI in the beginning, probably abandoned for remote control.
* Remote control through a client-server structure with a browser-based interface
* Batch computation of (very) high resolution images

This is a hobby project (won't be big and professional like GNU). Don't hold your breath.
