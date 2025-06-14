// Main program for the mandel-rs project

mod storage;
mod gui;
mod comp;

use iced::Application;
use gui::mandel_iced_app::MandelIcedApp;

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
