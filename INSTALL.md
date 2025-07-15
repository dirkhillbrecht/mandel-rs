# How to install mandel.rs

Mandel.rs is so far only available as Rust source code from Github. To "install" it, perform the following steps:

* Install the Rust package manager and compiler using the "rustup" programm as described [on the rustup documentation](https://www.rust-lang.org/tools/install). You need at least Rust 1.88.
* Clone the repository using `git clone` or download and unpack an archive from Github.
* Go into the project's directory and compile it with `cargo build --release`.
* Start the compiled binary in `target/release/mandel-rs`.
* Copy the binary anywhere you like, e.g. `$HOME/bin` or `/usr/local/bin`.

And then you can start exploring the Mandelbrot set!

[Back to README](README.md)
