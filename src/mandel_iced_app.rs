use iced::{Application, Command, Element, Theme};
use crate::{data_storage::DataStorage, simple_mandelbrot};

#[derive(Debug, Clone)]
pub enum Message {
    LeftChanged(String),
    RightChanged(String),
    TopChanged(String),
    BottomChanged(String),
    WidthChanged(String),
    HeightChanged(String),
    MaxIterationChanged(String),
    ComputeClicked,
}

pub struct MandelIcedApp {
    storage: Option<DataStorage>,
    computing: bool,
    left: String,
    right: String,
    top: String,
    bottom: String,
    width: String,
    height: String,
    max_iteration: String,
}

impl Default for MandelIcedApp {
    fn default() -> Self {
        MandelIcedApp {
            storage: None,
            computing: false,
            left: "-2.1".to_string(),
            right: "0.75".to_string(),
            top: "1.25".to_string(),
            bottom: "-1.25".to_string(),
            width: "800".to_string(),
            height: "600".to_string(),
            max_iteration: "200".to_string(),
        }
    }
}

impl MandelIcedApp {
    fn iteration_to_color(it: u32, maxit: u32) -> [u8; 3] {
        if it==maxit {
            [0,0,0]
        }
        else {
            // Some simple color gradient
            let ratio=it as f32/maxit as f32;
            let xor=((it%2)*255) as u8;
            let red=((255.0 * ratio * 5.0) % 255.0) as u8 ^ xor;
            let green=((255.0 * (1.0 - ratio) *3.0) % 255.0) as u8 ^ xor;
            let blue=((128.0 + 127.0*ratio*2.0) % 255.0) as u8 ^ xor;
            [red,green,blue]
        }
    }
    fn render_fractal(&self, storage: &DataStorage) -> Element<Message> {
        use iced::widget::image;

        let width = storage.plane().width();
        let height = storage.plane().height();

        let mut pixels = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(point)=storage.plane().get(x, y) {
                    let color=Self::iteration_to_color(point.iteration_count(), storage.max_iteration());
                    pixels.extend_from_slice(&color);
                    pixels.push(255);
                }
                else {
                    pixels.extend_from_slice(&[255,0,255,255]);
                }
            }
        }
        let handle = image::Handle::from_pixels(width as u32,height as u32,pixels);
        image(handle).into()
    }
}

impl Application for MandelIcedApp {

    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Mandelbrot Fractal Visualizer".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LeftChanged(value) => self.left = value,
            Message::RightChanged(value) => self.right = value,
            Message::TopChanged(value) => self.top = value,
            Message::BottomChanged(value) => self.bottom = value,
            Message::WidthChanged(value) => self.width = value,
            Message::HeightChanged(value) => self.height = value,
            Message::MaxIterationChanged(value) => self.max_iteration = value,
            Message::ComputeClicked => {
                if let (Ok(left), Ok(right), Ok(bottom), Ok(top),
                        Ok(width), Ok(height), Ok(max_iteration)) = (
                    self.left.parse::<f64>(),
                    self.right.parse::<f64>(),
                    self.bottom.parse::<f64>(),
                    self.top.parse::<f64>(),
                    self.width.parse::<u32>(),
                    self.height.parse::<u32>(),
                    self.max_iteration.parse::<u32>(),
                ) {
                    println!("Compute started");
                    self.computing=true;
                    let mut storage = DataStorage::new(left,right,bottom,top,
                        width,height,max_iteration);
                    simple_mandelbrot::compute_mandelbrot(&mut storage);
                    self.storage=Some(storage);
                    self.computing=false;
                    println!("Compute ended");
                }
                else {
                    println!("Problem with input data");
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        use iced::widget::{button, column, row, text, text_input};

        column![
            text("Mandelbrot Fractal Visualizer").size(24),
            row![
                text("Left: "),
                text_input("", &self.left).on_input(Message::LeftChanged),
                text("Right: "),
                text_input("", &self.right).on_input(Message::RightChanged),
            ].spacing(10),
            row![
                text("Top: "),
                text_input("", &self.top).on_input(Message::TopChanged),
                text("Bottom: "),
                text_input("", &self.bottom).on_input(Message::BottomChanged),
            ].spacing(10),
            row![
                text("Width: "),
                text_input("", &self.width).on_input(Message::WidthChanged),
                text("Height: "),
                text_input("", &self.height).on_input(Message::HeightChanged),
                text("Max Iter: "),
                text_input("", &self.max_iteration).on_input(Message::MaxIterationChanged),
            ].spacing(10),
            button("Compute Mandelbrot").on_press(Message::ComputeClicked),
            if self.computing {
                column![text("Computing…")].spacing(10)
            }
            else if let Some(storage) = &self.storage {
                column![
                    text(format!("Computed {}*{} fractal", storage.plane().width(), storage.plane().height())),
                    self.render_fractal(storage)
                ].spacing(10)
            }
            else {
                column![text("Ready to compute")].spacing(10)
            }
        ].spacing(20).padding(20).into()
    }

}

// end of file
