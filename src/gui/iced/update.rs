/// Update part of the mandel-rs-Iced-GUI
use crate::comp::mandelbrot_engine::{EngineState, MandelbrotEngine};
use crate::gui::iced::app::{AppState, ZoomState};
use crate::gui::iced::message::Message;
use crate::storage::computation::comp_storage::CompStorage;
use crate::storage::image_comp_properties::{ImageCompProperties, StageProperties};
use crate::storage::visualization::viz_storage::VizStorage;
use euclid::{Point2D, Rect, Size2D};
use iced::Task;
use std::sync::Arc;
use std::time::Duration;

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::ToggleSidebar => state.viz.sidebar_visible = !state.viz.sidebar_visible,
        Message::PresetChanged(value) => state.viz.math_preset = value,
        Message::PresetClicked => {
            let data = &state.viz.math_preset.preset();
            state.math.area = data.coordinates();
            state.math.max_iteration = data.max_iteration().to_string();
            return Task::perform(async {}, |_| Message::ComputeClicked);
        }
        Message::LeftChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    Point2D::new(value, state.math.area.min_y()),
                    state.math.area.max(),
                ]);
            }
        }
        Message::RightChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    state.math.area.min(),
                    Point2D::new(value, state.math.area.max_y()),
                ]);
            }
        }
        Message::TopChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    state.math.area.min(),
                    Point2D::new(state.math.area.max_x(), value),
                ]);
            }
        }
        Message::BottomChanged(value) => {
            if let Ok(value) = value.parse::<f64>() {
                state.math.area = Rect::from_points([
                    Point2D::new(state.math.area.min_x(), value),
                    state.math.area.max(),
                ]);
            }
        }
        Message::WidthChanged(value) => state.math.width = value,
        Message::HeightChanged(value) => state.math.height = value,
        Message::MaxIterationChanged(value) => state.math.max_iteration = value,
        Message::ComputeClicked => {
            state.viz.auto_start_computation = false;
            if let (Ok(width), Ok(height), Ok(max_iteration)) = (
                state.math.width.parse::<u32>(),
                state.math.height.parse::<u32>(),
                state.math.max_iteration.parse::<u32>(),
            ) {
                if let Some(engine) = &state.engine {
                    engine.stop();
                }
                state.runtime.computing = false;
                //                    state.runtime.computing = true;
                let comp_props = ImageCompProperties::new(
                    StageProperties::new(state.math.area, Size2D::new(width, height)),
                    max_iteration,
                );
                state.comp_storage = Some(Arc::new(CompStorage::new(comp_props)));
                state.engine = Some(MandelbrotEngine::new(&state.comp_storage.as_ref().unwrap()));
                state.storage = Some(VizStorage::new(&state.comp_storage.as_ref().unwrap()));
                state.engine.as_ref().unwrap().start();
                state.runtime.canvas_cache.clear();

                // Schedule first update
                return Task::perform(async {}, |_| Message::UpdateViz);
            } else {
                println!("Problem with input data");
            }
        }
        Message::UpdateViz => {
            if let Some(ref mut vizstorage) = state.storage {
                if vizstorage.process_events() {
                    state.runtime.canvas_cache.clear();
                }
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
            state.runtime.canvas_cache.clear();
        }
        Message::IterationAssignmentChanged(value) => {
            state.viz.iteration_assignment = value;
            state.runtime.canvas_cache.clear();
        }
        Message::RenderSchemeChanged(value) => {
            state.viz.render_scheme = value;
            state.runtime.canvas_cache.clear();
        }
        Message::ShiftStage(offset) => {
            if let Some(engine) = &state.engine {
                engine.stop();
            }
            state.runtime.computing = false;
            let new_storage = state
                .comp_storage
                .as_ref()
                .unwrap()
                .as_ref()
                .shifted_clone_by_pixels(offset);
            state.math.area = new_storage.original_properties.stage_properties.coo;
            state.comp_storage = Some(Arc::new(new_storage));
            state.engine = Some(MandelbrotEngine::new(&state.comp_storage.as_ref().unwrap()));
            state.storage = Some(VizStorage::new(state.comp_storage.as_ref().unwrap()));
            state.engine.as_ref().unwrap().start();
            state.runtime.canvas_cache.clear();

            // Schedule first update
            return Task::perform(async {}, |_| Message::UpdateViz);
        }
        Message::ZoomStart((origin, ticks)) => {
            state.runtime.zoom = Some(ZoomState::start(origin, ticks));
            state.runtime.canvas_cache.clear();
        }
        Message::ZoomTick(ticks_offset) => {
            //state.runtime.zoom.update_ticks(ticks_offset);
            if ticks_offset != 0
                && let Some(zoom) = &mut state.runtime.zoom
            {
                zoom.update_ticks(ticks_offset);
                state.runtime.canvas_cache.clear();
            }
        }
        Message::ZoomEndCheck => {
            if let Some(zoom) = &state.runtime.zoom
                && zoom.is_timeout(Duration::from_millis(500))
            {
                if zoom.ticks != 0 {
                    if let Some(engine) = &state.engine {
                        engine.stop();
                    }
                    state.runtime.computing = false;
                    let new_storage = state
                        .comp_storage
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .zoomed_clone_by_pixels(
                            Point2D::new(zoom.origin.x as i32, zoom.origin.y as i32),
                            zoom.factor,
                        );
                    state.math.area = new_storage.original_properties.stage_properties.coo;
                    state.comp_storage = Some(Arc::new(new_storage));
                    state.engine =
                        Some(MandelbrotEngine::new(&state.comp_storage.as_ref().unwrap()));
                    state.storage = Some(VizStorage::new(state.comp_storage.as_ref().unwrap()));
                    state.engine.as_ref().unwrap().start();
                    state.runtime.canvas_cache.clear();
                    state.runtime.zoom = None;
                    // Schedule first update
                    return Task::perform(async {}, |_| Message::UpdateViz);
                }
                state.runtime.zoom = None;
            }
        }
        Message::MousePressed(_point) => {}
        Message::MouseDragged(_point) => {}
        Message::MouseReleased(_point) => {}
    }
    Task::none()
}

// end of file
