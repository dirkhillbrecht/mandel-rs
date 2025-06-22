/// Update part of the mandel-rs-Iced-GUI
use crate::comp::mandelbrot_engine::{EngineState, MandelbrotEngine};
use crate::gui::iced::app::MandelRSApp;
use crate::gui::iced::message::Message;
use crate::storage::image_comp_properties::{ImageCompProperties, Rect, StageProperties};
use crate::storage::visualization::viz_storage::VizStorage;
use iced::Task;
use std::time::Duration;

pub fn update(state: &mut MandelRSApp, message: Message) -> Task<Message> {
    match message {
        Message::ToggleSidebar => state.sidebar_visible = !state.sidebar_visible,
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

// end of file
