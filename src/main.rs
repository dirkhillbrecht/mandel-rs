// Main program for the mandel-rs project

mod comp;
mod gui;
mod storage;

fn main() -> iced::Result {
    gui::iced::app::launch()
}

// end of file
