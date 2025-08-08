# How to install mandel-rs

## The easy way: Use a release (Linux-only)

Mandel-rs features releases.
Just pick one, extract the binary and start it.
No installation needed.
No external dependencies.

Note that this way is only available for Linux on 64-bit x86 platforms.
For any other platform, you have to use the way described below.
That is also your way to go if you want to use a newer version than the latest release
or dig yourself in the source code.

## The less easy way: Use the source, Luke!

Mandel.rs is available as Rust source code from Github. To "install" it, perform the following steps:

* Install the Rust package manager and compiler using the "rustup" programm as described [on the rustup documentation](https://www.rust-lang.org/tools/install). You need at least Rust 1.88.
* Clone the repository using `git clone` or download and unpack an archive from Github.
* Go into the project's directory and compile it with `cargo build --release`.
* Start the compiled binary in `target/release/mandel-rs`.
* Copy the binary anywhere you like, e.g. `$HOME/bin` or `/usr/local/bin`.

And then you can start exploring the Mandelbrot set!

[Back to README](README.md)
