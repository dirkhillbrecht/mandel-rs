// Main program for the mandel-rs project

mod data_point;
mod data_plane;
mod data_storage;
mod simple_mandelbrot;
mod mandelbrot_app;

use crate::data_point::DataPoint;
use crate::data_storage::DataStorage;

fn main() {
    let app=mandelbrot_app::MandelbrotApp::new();
    eframe::run_native("Mandelbrot Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))))
        .unwrap();
}

// end of file
