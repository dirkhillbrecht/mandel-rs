# Release notes for mandel-rs 0.5.0

This release is mainly polishing stuff, contains an important bugfix and adds some smaller features.

## Presentation on screen is always updated

In certain situations, most prominently with the full Mandelbrot set default image,
the last computed points were not updated correctly on screen.
Computation updates are fed through a two-stage pipeline.
Unfortunately, the presentation updater at the end of the _second_ second stage
stopped operations as soon as the computation engine at the beginning of the _first_ stage was finished.

This update repairs this behaviour fixing the bug and also some other quirks around the computation handling.

## Other visible changes

* **Runtime representation in frontend fixed** - The "Compute" and "Stop" buttons were shown wrongly as the application did not track the marker correctly whether a computation is in process or not.
* **Cleanup of the main menu on the left** - The application's menu on the left is cleaner now. This is an ongoing process.
* **More nice presets** - A number of new visually appealing areas has been added to the prefixes

## Other internal changes

* **Presets contain visualization parameters** - The presets do not only contain the mathematical parameters (center coordinates, radius), but also the parameters defining the visualization (color scheme, iteration depth assignment). This allows to recreate the image completely as intended.
* **Preparation for computation on top of preliminary data** - The math and visualization stage allow storing preliminary data for smoother presentation. Such data is now considered correctly when actually computing the image. Note that so far, no preliminary data is stored in the image, therefore this change has no visual impact so far.
* **Add Rust dependency cache to release Github action** - This speeds up build time of a new release
* **Update to Rust 1.89** - Always use the latest and greatest. The update actually only contains a small number of changes preventing some warnings.

## More resources

This program's home base is its [Github project](https://github.com/dirkhillbrecht/mandel-rs).
Have a look there for the source code and further information.
