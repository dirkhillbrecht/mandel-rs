use std::time::Duration;
use iced::{Application, Command, Element, Theme};
use crate::storage::visualization::viz_storage::VizStorage;
use crate::comp::mandelbrot_engine::{EngineState, MandelbrotEngine};
use crate::storage::image_comp_properties::{ImageCompProperties, Rect, StageProperties};

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
    StopClicked,
    UpdateViz,
}

pub struct MandelIcedApp {
    storage: Option<VizStorage>,
    engine: Option<MandelbrotEngine>,
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
            engine: None,
            computing: false,

/*            // Full mandelbrot set
            left: "-2.1".to_string(),
            right: "0.75".to_string(),
            top: "1.25".to_string(),
            bottom: "-1.25".to_string(),
            width: "800".to_string(),
            height: "600".to_string(),
            max_iteration: "200".to_string(),
*/
/*            // Deep zoom elephant valley
            left: "-0.7512".to_string(),
            right: "-0.7502".to_string(),
            top: "0.1103".to_string(),
            bottom: "0.1093".to_string(),
            width: "800".to_string(),
            height: "600".to_string(),
            max_iteration: "2000".to_string(),
*/
/*            // Spiral region
            left: "-0.7269".to_string(),
            right: "-0.7259".to_string(),
            top: "0.1889".to_string(),
            bottom: "0.1879".to_string(),
            width: "2800".to_string(),
            height: "1800".to_string(),
            max_iteration: "2000".to_string(),
*/
            // Seahorse valley
            left: "-0.7463".to_string(),
            right: "-0.7453".to_string(),
            top: "0.1102".to_string(),
            bottom: "0.1092".to_string(),
            width: "1200".to_string(),
            height: "800".to_string(),
            max_iteration: "2000".to_string(),

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
    fn render_fractal(&self, storage: &VizStorage) -> Element<Message> {
        use iced::widget::image;

        let width = storage.stage.width();
        let height = storage.stage.height();

        let mut pixels = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(point)=storage.stage.get(x, y) {
                    let color=Self::iteration_to_color(point.iteration_count(), storage.properties.max_iteration);
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
        (Self::default(), Command::perform(async{}, |_| Message::ComputeClicked))
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
                    if let Some(_) = self.engine {
                        println!("Engine already initialized");
                    }
                    else {
                        self.computing=true;
                        let comp_props=ImageCompProperties::new(StageProperties::new(
                            Rect::new(left,right,bottom,top), width, height),max_iteration);
                        self.engine=Some(MandelbrotEngine::new(comp_props));
                        self.storage=Some(VizStorage::new(self.engine.as_ref().unwrap().storage()));
                        self.engine.as_ref().unwrap().start();

                        // Schedule first update
                        return Command::perform(async{}, |_| Message::UpdateViz);
                    }
                }
                else {
                    println!("Problem with input data");
                }
            },
            Message::UpdateViz => {
                if let Some(ref mut vizstorage) = self.storage {
                    vizstorage.process_events();
                }
                if let Some(engine) = &self.engine {
                    let state=engine.state();
                    if state == EngineState::Aborted || state == EngineState::Finished {
                        self.engine=None;
                        self.computing=false;
                        return Command::none(); // Stop updates
                    } else {
                        // Schedule next update
                        return Command::perform(
                            async { tokio::time::sleep(Duration::from_millis(20)).await; },
                            |_| Message::UpdateViz
                        );
                    }
                }
            },
            Message::StopClicked => {
                if let Some(_) = self.engine {
                    self.engine.as_ref().unwrap().stop();
                    self.engine=None;
                    self.computing=false;
                }
            },
        }
        Command::none()
    }


    fn view(&self) -> Element<Message> {
        use iced::widget::{button, column, row, text, text_input};

        column![
            text("Mandelbrot Fractal Visualizer").size(24),
            row![
                iced::widget::horizontal_space(),
                text("Computed size:"),
                text_input("", &self.width).width(100).on_input(Message::WidthChanged),
                text("*"),
                text_input("", &self.height).width(100).on_input(Message::HeightChanged),
                text("pixels, maximum depth:"),
                text_input("", &self.max_iteration).width(100).on_input(Message::MaxIterationChanged),
                text("iterations"),
                iced::widget::horizontal_space(),
            ].spacing(10).align_items(iced::Alignment::Center),
            row![
                iced::widget::horizontal_space(),
                if self.computing {
                    button("Stop Computation").on_press(Message::StopClicked)
                } else {
                    button("Compute Mandelbrot").on_press(Message::ComputeClicked)
                },
            ].align_items(iced::Alignment::Center),
            row![
                iced::widget::horizontal_space(),
                text("Coordinate (x:"),
                text_input("", &self.right).width(100).on_input(Message::RightChanged),
                text(",y:"),
                text_input("", &self.top).width(100).on_input(Message::TopChanged),
                text(")"),
            ].spacing(10).align_items(iced::Alignment::Center),
            row![
                iced::widget::horizontal_space(),
                if let Some(storage) = &self.storage {
                    column![
                        if self.computing {
                            text(format!("Computing {}*{} fractal", storage.properties.stage_properties.width, storage.properties.stage_properties.height))
                        }
                        else {
                            text(format!("Computed {}*{} fractal", storage.properties.stage_properties.width, storage.properties.stage_properties.height))
                        }
                    ].spacing(10)
                }
                else {
                  column![text(if self.computing { "Computing…" } else { "Ready to compute" })].spacing(10)
                },
                iced::widget::horizontal_space(),
            ],
            row![
                iced::widget::horizontal_space(),
                if let Some(storage) = &self.storage {
                    column![
                        self.render_fractal(storage)
                    ].spacing(10)
                }
                else {
                    column![ text("") ]
                },
                iced::widget::horizontal_space(),
            ],
            row![
                text("Coordinate (x:"),
                text_input("", &self.left).width(100).on_input(Message::LeftChanged),
                text(",y:"),
                text_input("", &self.bottom).width(100).on_input(Message::BottomChanged),
                text(")"),
                iced::widget::horizontal_space(),
            ].spacing(10).align_items(iced::Alignment::Center),
        ].spacing(20).padding(20).into()
    }

}

// end of file
