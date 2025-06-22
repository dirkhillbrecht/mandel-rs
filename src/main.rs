// Main program for the mandel-rs project

mod comp;
mod gui;
mod storage;

fn main() -> iced::Result {
    iced::application(
        "Mandelbrot Fractal Visualizer",
        gui::mandel_iced_app::update,
        gui::mandel_iced_app::view,
    )
    .subscription(gui::mandel_iced_app::subscription)
    .run()
}

// end of file
