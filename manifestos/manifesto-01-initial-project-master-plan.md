# Mandel.rs - Learning the Rust language writing a fractal graphics visualizer

_2025-06-09, Dirk Hillbrecht_

This project has two goals:

1. When finished, it shows fractal graphics like the mandelbrot set in the usual, well-known visualisations.
2. It allows me to learn programming in the Rust language.

## Overview over the envisioned program design

I have already developed fractal visualizers in the past. I have a clear idea how the program should be structured. It should consist of three basic building blocks:

1. Storage: An abstract representation of the computed data.
2. Computation: Some code actually generating the data of the fractal, storing them in the storage.
3. Visualization: Code showing the data in the storage, e.g. through iteration-depth-dependent colorized images.

I layout these three building blocks a bit further:

### The storage

The storage is a two-dimensional array of data points. The indexes of these data points reflect certain cartesian coordinates in the x/y-plane. This is the super-charged version of the iteration-depth (or even color) 2-dimensional data storage of early "fractal programs" of the 1980ies.

The data points do not simply store a iteration depth. They store more information. This allows to separate computation from visualization. I.e. computation stores the depth of when the iterated computation of the fractal left the escape distance, the visualization makes a certain color of it. This way, visualization can change the color without recomputation.

But storage could contain more information. It could store the actual position of the last iterated point when it first jumped over the escape distance. Then, visualization could take this information also into account and use it e.g. for shaded colorization of the point.

### Computation

Computation fills the storage with data. The most simple solution is the classic point-by-point computation of a fractal graphics using one or more parallel running iterators.

This realisation can be updated by time-saving optimisations:

* Boundary trace around areas of same iteration depth
* Area splits to prevent unnecessary computation
* Epsilon-distance based computation for extreme iteration depth.

As computation is designed independent of storage, optimisations can be applied later. Implementation will start with the most straight-forward algorithm of independent point-by-point computation.

### Visualization

The visualizer reads the storage and presents the data by some means. I envision two approaches to visualization:

1. Local visualization on the computer's and user's GUI. This should be done in a system-independent way with emphasis on a Linux/Ubuntu environment.
2. Remote visualization through a Web browser. This should use some sensible library for communication with the browser and will also include a browser-based interface to the program in general.

Idea is to implement local visualization first with remote operation in mind but only implemented later.

## Development process for educational purposes

I want to develop mandel.rs to _learn_ programming in Rust. I am a haptic learner: I need to type in things to really embrace and understand them. Therefore it is important that _I_ am actually the one typing all needed stuff in.

That means that whoever helps me in writing this program does _not_ create code on their own. They explain things to me, they check code I've written, they point out mistakes or problematic designs if they spot it - by they _do not_ correct me on their own. They only change stuff in the project on my explicit advise!

My educational goal is to understand all core aspects of the Rust Programming Language. That includes:

* The development infrastructure: How to use libraries, how to setup a project, how to evolve its structure.
* The programming language: How to program in Rust, key concepts of the language, important building blocks, design of a complex codebase, pitfalls - especially for Java programmers
* Building and deploying: How to generate and publish executable programs.

## My current knowledge

As I said, I want to learn Rust with this project. I am a very experienced Java programmer, so explaining things in Java terms is very welcome. Additionally, I have good knowledge in shell programming and programming concepts in general. I have programmed a bit in C and a very small bit in C++ in the past, but I am everything but experienced or fluid in these languages.

I am German native speaker but I prefer communication in English on programming and software development topics.

## Project setup

This project is setup with a git repository to store all important development milestones. I use VS Code for development and I have installed the "Rust Extension Pack".
