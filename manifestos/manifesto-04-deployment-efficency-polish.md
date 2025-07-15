# Manifesto 04: Deployment processes, efficency issues, polishing

Finishing manifesto 03 brought the project to a minimum-viable-product state.
The follow-up manifesto - here - will continue the road to a modern, useable, versatile Rust application.
For this, the following developments steps are addressed.


## Visualization and GUI

While not mentioning it here directly, the interface needs some more polish.
This will happen within the other issues named here

### Issue 4.1: More parameters for colorisation algorithm

Colorisation misses two more parameters:

1. Set the number of stripes to be generated from the gradient scheme. Currently, this is fixed
2. Define a start offset for the stripes application. This would allow simple color animations.

The new parameters must be integrated into the interface.

### Issue 4.2: Size stage to canvas

The stage size is not set explicitly but derived from the canvas representing the stage.
The stage can be set either to

* half of the canvas size for faster computation
* exactly to the canvas size for pixel-exact images
* or to double of the canvas size for smoothing by using multiple stage points for one pixel

In each case, the stage is scaled to fill the canvas completely.
This renders the multiple presentation modes with different scaling strategies unnecessary.

### Issue 4.3: Export stage as PNG file

The image in the stage should be exportable as PDF so that it can be shown independently.
The mathematical parameters, especially the mathematical coordinates, should be stored in some data fields in the file.

### Issue 4.4: Export and collect data of interesting areas in description files

Mathematical coordinates and the other parameters needed for reproducing an image are stored in a data structure
which can be stored to a local file or some online storage.

### Issue 4.5: Full screen mode

The canvas showing the fractal image is shown frameless and fullscreen
so that the natural resolution of the graphics hardware is used.

### Issue 4.6: Snap zoom at 2^Z values

Currently, each mouse wheel event changes the zoom factor. A "snappy" mode is implemented which only allows
zoom factors with integer exponent. For visual feedback, this needs some animation of the preview.
Snappy zoom mode can be turned on and off with a checkbox and that default value can be overridden by pressing
"shift" during the zoom operation.


## Computation - data storage

For the computational data storage, the dual stage approach is a very strong base.
However, there is a number of enhancements possible and needed.

### ✅ (2025-07-13) Issue 4.7: Move maximum iteration and stage size into mathematical model

The primary representation of the stage size and the maximum iteration depth are still the String-based
input fields in the interface.
These settings should be moved into the already existing properties storages so that their on-screen representation
is derived from the internal storage, not the other way around.

### Issue 4.8: Move shift control from canvas model to app model

Dragging the canvas' content around with the mouse is the interface to shifting the presented area.
The program-internal control of this process is courtesy of the canvas' model.
For zooming, however, the internal control has been moved into the app's main model.
The reason for this shift was that the timer-based stop of the zoom operation could not be handled by the canvas' model.

It is sensible to move the drag control also into the app's model.
This allows tighter integration with the computation engine, especially when it comes to earlier start of recomputation.

### Issue 4.9: Keep calculated data at 2^Z zooms

This is a leftover from manifesto 03:
If the computed area changes by an integer power of 2, some of the already computed points can be kept.
The whole zoom algorithm is already designed in a way that this works without correctness errors.
However, keeping the data is not yet implemented.

### Issue 4.10: Stage size and presentation parameter changes

While changing the size of the computed area is already quite flexible, changing the stage size isn't.
Also, changing other computation parameters is not handled as efficiently as possible.

#### Issue 4.10.1: Fill stage with guessed values when zooming

Currently, computation of a zoomes image starts with an empty stage.
Even in "2^R with R not in Z" cases, (parts of) the stage could be preset with guessed values from the former image.
These guessed values are then overwritten by the actual computation but they make the whole presentation smoother.

#### Issue 4.10.2: Keep calculated data at 2^Z size changes

If the stage's size is changed by some integer exponents of 2, points can be kept and only points in the gaps
need to be recomputed.
Algorithmically, this is quite similar to the handling of 2^Z zooms mentioned above.

#### ✅ (2025-07-14) Issue 4.10.3: Keep calculated data when maximum iteration changes

If the maximum iteration changes exclusively in the image data, recomputation can be kept to _some_ points of the image:

* If the maximum iteration depth _decreases_, points with higher computed maximum iteration simply have to be changed to the new maximum iteration. No recomputation at all is needed.
* If the maximum iteration depth _increases_, data for all points with the former maximum interation depth have to removed. Only those points have to be recomputed then.

### Issue 4.11: Change mathematical representation into data triple of center point, angle, radius, and aspect ratio

Currently, the mathematical representation of an area to be shown is represented by two adjacent points of its enclosing rectangle.
It makes sense to switch to a more "polar" primary representation of the area:

* Origin and radius define the center and the extent of the area, independent of any aspect ratio.
* An angle allows to rotate the presentation at the very base of the mathematical data storage.
* Finally, the aspect ratio can adopt to non-square target representations.

This representation is convertible to and from the current edge-point-based representation if the angle is 0°.
It makes the representation more flexible and allows the introduction of the angle value.

### Issue 4.12: Check that the event mechanism is properly shut down when changing the stage

Shifting, zooming, rescaling and other operations replace the comp/viz stage pair with a new one.
This can happen during still ongoing computation.
It must be guaranteed that the event mechanism between computation and visualization stage is properly shut down.


## Computation - algorithmic enhancements

Apart from the storage section of the computation schemes, there are several optimisations on the algorithmic realm.

### Issue 4.13: Automatic adjustment of maximum iteration depth

Zooming in often needs an increased maximum iteration depth for nice images.
Such iteration depth adjustments happen automatically depending on the current iteration depth and the zoom factor.

### Issue 4.14: More efficient single dot computation

The basic iteration formula for computing the escape speed of a single point in the complex plane can be optimized
to use less operations.
This is a trivial change just using basic mathematics.
As we do not use actual complex number types for the computation but "real" numbers with explicit transformation rules,
this should be no big deal.

### Issue 4.15: Parallel dot computation

The whole storage system is laid out for parallel computation of dots.
However, currently the computation is strictly single-threaded.
The implementation is changed in a way that multiple points are computed in parallel.
Interestingly, this should be not that big of a deal as the architecture should be completely prepared for this change.

### Issue 4.16: Boundary trace algorithm for dot computation

Computation efficiency can be increased - and vastly increased for certain images - by using boundary trace around areas
with the same escape speed - and therefore the same raw value for color selection in the output.
I do already have a complete implementation of parallel boundary trace in an abandonned predecessor project written in Java.
That algorithm is reimplemented in Rust.


## Program organisation and deployment

Finally, there are some tasks targetting the program as a whole.
Note that some of these tasks might be decided to be not yet ready and moved forward into a later manifesto.

### ✅ (2025-07-15) Issue 4.17: Update tokio to 1.46

We currently use version 1.0 of the tokio library.
This version seems to be vastly behind the current version which is 1.46.
The tokio library dependency is updated and brought to the most current version available.

### Issue 4.18: Move storage and engine into an independent library

The whole project is split into the actual _program_ with the GUI
and a _library_ with all parts independent of the user interface and presentation.
These independent parts are mainly computation and stage representation.
The library part could then be released as a "real" Rust library crate
with semantic version and all bells and whistles possible.

### Issue 4.19: I18N'd user interface

The user interface - which is quite sparsely populated with written text in natural language anyway - gets some internationalization capabilties.
There should be a library for doing this in Rust.

### Issue 4.20: Generate documentation on push

Every time the master branch of the project is pushed to Github,
some automatic process regenerates the online documentation of the classes.

### Issue 4.21: Generate executables on push

Every time a release commit is pushed to Github,
some automatic process produces an actual released version of the program


## Summary

The issues in this manifesto enhance the program in different spots.
Altogether, they increase the whole program considerably.
The target could be a 1.0 version of the program.


## Claude Review

Based on analysis of Manifesto 04, these are sensible development targets that show good strategic thinking. Here's the assessment:

### Strong Points
- **Well-structured progression** from GUI polish → storage optimization → algorithmic improvements → deployment
- **Smart architectural improvements** like moving to polar coordinates and centralizing state management
- **Performance-focused** with practical optimizations (2^Z data retention, parallel computation)

### Difficulty Estimates

**Easy (1-2 weeks)**:
- 4.1: Colorization parameters
- 4.7: Move settings to model
- 4.17: Tokio update

**Medium (2-4 weeks)**:
- 4.2: Stage sizing modes
- 4.3: PNG export
- 4.5: Fullscreen mode
- 4.8: Move drag to app model
- 4.13: Auto iteration adjustment

**Hard (4-8 weeks)**:
- 4.6: Snap zoom with animation
- 4.9-4.10: Data retention algorithms
- 4.11: Polar coordinate system
- 4.15: Parallel computation
- 4.18: Library extraction

**Very Hard (8+ weeks)**:
- 4.16: Boundary trace algorithm
- 4.19: Internationalization
- 4.20-4.21: CI/CD automation

### Suggested Ordering
1. **Foundation** (4.7, 4.8, 4.17) - Clean up architecture first
2. **Core Features** (4.1, 4.2, 4.3, 4.5) - User-visible improvements
3. **Performance** (4.9, 4.15, 4.13) - Computational enhancements
4. **Advanced** (4.11, 4.6, 4.16) - Complex algorithmic work
5. **Deployment** (4.18, 4.20, 4.21) - Production readiness

### Minor Gaps
- **Error handling** for file operations (PNG export, data files)
- **Configuration persistence** for user preferences
- **Undo/redo** for navigation operations

The scope is ambitious but well-reasoned for advancing from MVP to production-quality software.
