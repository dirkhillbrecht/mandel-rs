# Manifesto 3: Cleanup work and building a minimum viable product

With finishing manifesto 2, the computation and visualization framework has reached a technical milestone. It can be assumed feature-complete for the moment. The next development stage, which layed out in this manifesto, adds stuff around it to

a) make development better to handle
b) build a program that can really be used by someone else

The following issues have to be addressed, not in any particular order:

## Issue 1: Removal of source code warnings in Visual Code

The source code is full of warnings, mainly for unused functions. We need to remove these warnings so that only important stuff pops up here.

## Issue 2: Update to newer iced version

Iced 0.12 is a bit old-fashined. We should move to the most current sensibly useable version to have the richest feature set (hopefully…).

## Issue 3: Update GUI layout

Currently, the interface loses much space due to frames, padding, and clumsy GUI design. This needs to be cleaner. Let the image fill up the space in the interface in the best possible way.

## Issue 4: Better coloring

The coloring of the produced image is a quick hack which delivers a recognizable result. But that's anything but visually pleasing. We need a nicer scheme:

* Some visually pleasing color gradients can be selected (and defined, but that's a different step)
* The assignment between iteration depth and color can be selected from a number of functions (e.g. square, linear, log, log log).

Changing the color scheme should _not_ lead to any recomputation as color assignment is completely separated from the mathematical operations.

## Issue 5: Better visualization of unfinished images

Currently, non-computed points are drawn in a special color. They should be estimated by near-by already computed points. This would make the image more pleasing while not increasing drawing complexity mentionable.

## Issue 6: Progress bar during computation

Especially with the besser visualization, it is difficult to see how much of the computation has already been done. A progress bar solves the issue.

## Issue 7: Interactive area selection

Images to be rendered should be selectable by

* Panning the rendered area around
* Zooming in and out

These operations should preserve all already computed data which can be preserved. E.g. when multiplying the zoom factor by 2, already computed values can be spread around accordingly in the new CompStorage so that they do not have to be recomputed.

## Issue 8: Re-evaluating the structure of modules and export definitions

The source code is a bit randomly scattered through modules, submodules, files. So far there is no re-export of public stuff, everything is declared to be used exactly from where it is defined. This should be cleaned up. Main questions:

* Is the breakup of the `storage` module into submodules with only two source code files in each of them really a sensible design?
* Should the exportable classes explicitly be mentioned further up in the module tree and be exported independently of the module-internal structure?

## Issue 9: Source code documentation

Current state is "few and far between". Source code documentation must become _much_ better so that the general source code structure becomes understandable for other people interested in the project.

## Implementation Plan

The issues above will be addressed in the following phases to ensure a logical progression from foundation to advanced features:

**Phase A: Foundation (Issues 1, 2, 8)**
1. Fix warnings first - gives clean baseline for development
2. Update iced version - establishes modern foundation  
3. Clean up module structure - makes subsequent work cleaner

**Phase B: Visual Polish (Issues 3, 4, 6)**
4. Improve GUI layout - immediate visual impact
5. Add progress bar - better user feedback
6. Implement better coloring - dramatic visual improvement

**Phase C: Advanced Features (Issues 5, 7)**
7. Unfinished point interpolation - enhanced progressive rendering
8. Interactive area selection - most complex, highest user value

**Phase D: Release Preparation (Issue 9)**
9. Comprehensive documentation - final step before release

## Target: Building the minimum viable product

Having implemented the improvements described here, mandel.rs should evolve into something which can really be used to create nice images - independently from this project's important educational objective. Having reached this state makes it sensible to produce a executable binary and make a "real" release - probably with the help of current Github processes.
