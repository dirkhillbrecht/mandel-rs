// Main program for the mandel-rs project

mod data_point;
mod data_plane;
mod data_storage;
mod simple_mandelbrot;
//mod mandelbrot_app;
mod mandel_iced_app;

use crate::data_point::DataPoint;
use crate::data_storage::DataStorage;
use iced::Application;
use mandel_iced_app::MandelIcedApp;

/*
fn main() {
    let app=mandelbrot_app::MandelbrotApp::new();
    eframe::run_native("Mandelbrot Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(app))))
        .unwrap();
}
        */

fn main() -> iced::Result {
    MandelIcedApp::run(iced::Settings::default())
}

// end of file
