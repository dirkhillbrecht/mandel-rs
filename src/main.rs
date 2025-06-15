// Main program for the mandel-rs project

mod storage;
mod gui;
mod comp;

use iced::Application;
use gui::mandel_iced_app::MandelIcedApp;

fn main() -> iced::Result {
    MandelIcedApp::run(iced::Settings::default())
}

// end of file
