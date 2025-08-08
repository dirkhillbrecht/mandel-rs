# Release notes for mandel-rs 0.4.0

This is the first public release of mandel-rs.
It features a useful program, yet things are not really settled and rough edges remain.

## What this version can do

* Compute Mandelbrot fractal graphics with a parallel point-by-point computation algorithm. Remarkably fast if you have a capable hardware. Main processor core count rulez!
* Allow increasing the maximum iteration depth with only recomputing the points in question.
* Allow selection of computed area from a number of presets.
* Freely adjustable size of the computed image with uncoupled presentation on the available screen area.
* Interactive panning and zooming of the computation area.
* Several options to adjust iteration depth to color assignments, immediate application without recomputation.
* Save an image as PNG file.
* Export coordinates into the clipboard.

## What this version cannot do

* Work with number represenations more precise than `f64`. This Mandelbrot microscope ends at a resolution of some 10^-14 radius magnitude.
* Use more efficient global image computation schemes like boundary trace.
* Have a nice GUI. This is really messed up, partly due to no proper cleanup since programming start, partly due to shortcomings in the underlying Iced library, partly due to a certain lazyness of the responsible developer (guess who…).
* Store parametes of intersting areas (and color assignments) somehow locally or remotely.
* Have nice presets for usual image resolutions.
* Allow customized coloring schemes.

Basically, this is a nice little program to compute Mandelbrot images.
It is already quite fast and in certain aspects quite convenient to use.
It misses really outstanding features so far, but:

It is released exactly two months after I started development.
And this is the first time I _ever_ program _anything_ in Rust.
So, it's not too bad, I think.

Use it for digging around in the always fascinating Mandelbrot set environment.

## More resources

This program's home base is its [Github project](https://github.com/dirkhillbrecht/mandel-rs).
Have a look there for the source code and further information.
