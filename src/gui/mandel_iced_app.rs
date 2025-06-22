use crate::comp::mandelbrot_engine::{EngineState, MandelbrotEngine};
use crate::storage::image_comp_properties::{ImageCompProperties, Rect, StageProperties};
use crate::storage::visualization::viz_storage::VizStorage;
use iced::{Element, Task};
use std::time::Duration;

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
    auto_start_computation: bool,
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
            auto_start_computation: true,
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
        if it == maxit {
            [0, 0, 0]
        } else {
            // Some simple color gradient
            let ratio = it as f32 / maxit as f32;
            let xor = ((it % 2) * 255) as u8;
            let red = ((255.0 * ratio * 5.0) % 255.0) as u8 ^ xor;
            let green = ((255.0 * (1.0 - ratio) * 3.0) % 255.0) as u8 ^ xor;
            let blue = ((128.0 + 127.0 * ratio * 2.0) % 255.0) as u8 ^ xor;
            [red, green, blue]
        }
    }
    fn render_fractal(&self, storage: &VizStorage) -> Element<Message> {
        use iced::widget::image;

        let width = storage.stage.width();
        let height = storage.stage.height();

        let mut pixels = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(point) = storage.stage.get(x, y) {
                    let color = Self::iteration_to_color(
                        point.iteration_count(),
                        storage.properties.max_iteration,
                    );
                    pixels.extend_from_slice(&color);
                    pixels.push(255);
                } else {
                    pixels.extend_from_slice(&[255, 0, 255, 255]);
                }
            }
        }
        let handle = image::Handle::from_rgba(width as u32, height as u32, pixels);
        image(handle).into()
    }
}

/*
impl Program for MandelIcedApp {

    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();
*/

//    fn new(_flags: ()) -> (Self, Task<Message>) {
//        (Self::default(), Task::perform(async{}, |_| Message::ComputeClicked))
//    }

/*
    fn title(&self) -> String {
        "Mandelbrot Fractal Visualizer".to_string()
    }
*/

pub fn subscription(state: &MandelIcedApp) -> iced::Subscription<Message> {
    if state.auto_start_computation {
        iced::Subscription::run(|| {
            async_stream::stream! {
                yield Message::ComputeClicked;
            }
        })
    } else {
        iced::Subscription::none()
    }
}

pub fn update(state: &mut MandelIcedApp, message: Message) -> Task<Message> {
    match message {
        Message::LeftChanged(value) => state.left = value,
        Message::RightChanged(value) => state.right = value,
        Message::TopChanged(value) => state.top = value,
        Message::BottomChanged(value) => state.bottom = value,
        Message::WidthChanged(value) => state.width = value,
        Message::HeightChanged(value) => state.height = value,
        Message::MaxIterationChanged(value) => state.max_iteration = value,
        Message::ComputeClicked => {
            state.auto_start_computation = false;
            if let (
                Ok(left),
                Ok(right),
                Ok(bottom),
                Ok(top),
                Ok(width),
                Ok(height),
                Ok(max_iteration),
            ) = (
                state.left.parse::<f64>(),
                state.right.parse::<f64>(),
                state.bottom.parse::<f64>(),
                state.top.parse::<f64>(),
                state.width.parse::<u32>(),
                state.height.parse::<u32>(),
                state.max_iteration.parse::<u32>(),
            ) {
                if let Some(_) = state.engine {
                    println!("Engine already initialized");
                } else {
                    state.computing = true;
                    let comp_props = ImageCompProperties::new(
                        StageProperties::new(Rect::new(left, right, bottom, top), width, height),
                        max_iteration,
                    );
                    state.engine = Some(MandelbrotEngine::new(comp_props));
                    state.storage = Some(VizStorage::new(state.engine.as_ref().unwrap().storage()));
                    state.engine.as_ref().unwrap().start();

                    // Schedule first update
                    return Task::perform(async {}, |_| Message::UpdateViz);
                }
            } else {
                println!("Problem with input data");
            }
        }
        Message::UpdateViz => {
            if let Some(ref mut vizstorage) = state.storage {
                vizstorage.process_events();
            }
            if let Some(engine) = &state.engine {
                let engine_state = engine.state();
                if engine_state == EngineState::Aborted || engine_state == EngineState::Finished {
                    state.engine = None;
                    state.computing = false;
                    return Task::none(); // Stop updates
                } else {
                    // Schedule next update
                    return Task::perform(
                        async {
                            tokio::time::sleep(Duration::from_millis(20)).await;
                        },
                        |_| Message::UpdateViz,
                    );
                }
            }
        }
        Message::StopClicked => {
            if let Some(_) = state.engine {
                state.engine.as_ref().unwrap().stop();
                state.engine = None;
                state.computing = false;
            }
        }
    }
    Task::none()
}

pub fn view(state: &MandelIcedApp) -> Element<Message> {
    use iced::widget::{button, column, row, text, text_input};

    column![
        text("Mandelbrot Fractal Visualizer").size(24),
        row![
            iced::widget::horizontal_space(),
            text("Computed size:"),
            text_input("", &state.width)
                .width(100)
                .on_input(Message::WidthChanged),
            text("*"),
            text_input("", &state.height)
                .width(100)
                .on_input(Message::HeightChanged),
            text("pixels, maximum depth:"),
            text_input("", &state.max_iteration)
                .width(100)
                .on_input(Message::MaxIterationChanged),
            text("iterations"),
            iced::widget::horizontal_space(),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            iced::widget::horizontal_space(),
            if state.computing {
                button("Stop Computation").on_press(Message::StopClicked)
            } else {
                button("Compute Mandelbrot").on_press(Message::ComputeClicked)
            },
        ]
        .align_y(iced::Alignment::Center),
        row![
            iced::widget::horizontal_space(),
            text("Coordinate (x:"),
            text_input("", &state.right)
                .width(100)
                .on_input(Message::RightChanged),
            text(",y:"),
            text_input("", &state.top)
                .width(100)
                .on_input(Message::TopChanged),
            text(")"),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
        row![
            iced::widget::horizontal_space(),
            if let Some(storage) = &state.storage {
                column![if state.computing {
                    text(format!(
                        "Computing {}*{} fractal",
                        storage.properties.stage_properties.width,
                        storage.properties.stage_properties.height
                    ))
                } else {
                    text(format!(
                        "Computed {}*{} fractal",
                        storage.properties.stage_properties.width,
                        storage.properties.stage_properties.height
                    ))
                }]
                .spacing(10)
            } else {
                column![text(if state.computing {
                    "Computing…"
                } else {
                    "Ready to compute"
                })]
                .spacing(10)
            },
            iced::widget::horizontal_space(),
        ],
        row![
            iced::widget::horizontal_space(),
            if let Some(storage) = &state.storage {
                column![state.render_fractal(storage)].spacing(10)
            } else {
                column![text("")]
            },
            iced::widget::horizontal_space(),
        ],
        row![
            text("Coordinate (x:"),
            text_input("", &state.left)
                .width(100)
                .on_input(Message::LeftChanged),
            text(",y:"),
            text_input("", &state.bottom)
                .width(100)
                .on_input(Message::BottomChanged),
            text(")"),
            iced::widget::horizontal_space(),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(20)
    .padding(20)
    .into()
}

//}

// end of file
