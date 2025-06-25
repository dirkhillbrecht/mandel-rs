/// Update part of the mandel-rs-Iced-GUI
use crate::comp::mandelbrot_engine::{EngineState, MandelbrotEngine};
use crate::gui::iced::app::AppState;
use crate::gui::iced::message::Message;
use crate::storage::image_comp_properties::{ImageCompProperties, Rect, StageProperties};
use crate::storage::visualization::viz_storage::VizStorage;
use iced::Task;
use std::time::Duration;

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::ToggleSidebar => state.viz.sidebar_visible = !state.viz.sidebar_visible,
        Message::PresetChanged(value) => state.viz.math_preset = value,
        Message::PresetClicked => {
            let data = &state.viz.math_preset.preset();
            state.math.left = data.coordinates().min_x().to_string();
            state.math.right = data.coordinates().max_x().to_string();
            state.math.top = data.coordinates().max_y().to_string();
            state.math.bottom = data.coordinates().min_y().to_string();
            state.math.max_iteration = data.max_iteration().to_string();
            return Task::perform(async {}, |_| Message::ComputeClicked);
        }
        Message::LeftChanged(value) => state.math.left = value,
        Message::RightChanged(value) => state.math.right = value,
        Message::TopChanged(value) => state.math.top = value,
        Message::BottomChanged(value) => state.math.bottom = value,
        Message::WidthChanged(value) => state.math.width = value,
        Message::HeightChanged(value) => state.math.height = value,
        Message::MaxIterationChanged(value) => state.math.max_iteration = value,
        Message::ComputeClicked => {
            state.viz.auto_start_computation = false;
            if let (
                Ok(left),
                Ok(right),
                Ok(bottom),
                Ok(top),
                Ok(width),
                Ok(height),
                Ok(max_iteration),
            ) = (
                state.math.left.parse::<f64>(),
                state.math.right.parse::<f64>(),
                state.math.bottom.parse::<f64>(),
                state.math.top.parse::<f64>(),
                state.math.width.parse::<u32>(),
                state.math.height.parse::<u32>(),
                state.math.max_iteration.parse::<u32>(),
            ) {
                if let Some(_) = state.engine {
                    println!("Engine already initialized");
                } else {
                    state.runtime.computing = true;
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
                    state.runtime.computing = false;
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
                state.runtime.computing = false;
            }
        }
        Message::ColorSchemeChanged(value) => {
            state.viz.gradient_color_preset = value;
        }
        Message::IterationAssignmentChanged(value) => {
            state.viz.iteration_assignment = value;
        }
    }
    Task::none()
}

// end of file
