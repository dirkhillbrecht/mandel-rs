use crate::comp::math_data::MathPreset;
/// View part of the mandel-rs-Iced-GUI
use crate::gui::iced::app::{AppState, ImageRenderScheme};
use crate::gui::iced::fract_canvas::FractalCanvas;
use crate::gui::iced::message::Message;
use crate::storage::visualization::coloring::presets::{GradientColorPreset, IterationAssignment};
use iced::widget::{
    button, canvas, column, container, pick_list, progress_bar, row, text, text_input,
};
use iced::{Element, Length};

fn render_fractal(app_state: &AppState) -> Element<Message> {
    let fract_canvas = FractalCanvas::new(app_state);
    canvas(fract_canvas)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn open_sidebar(state: &AppState) -> Element<Message> {
    container(
        column![
            row![
                button("<").on_press(Message::ToggleSidebar),
                text("Computed size:")
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
            row![
                text_input("", &state.math.width)
                    .width(50)
                    .on_input(Message::WidthChanged),
                text("*"),
                text_input("", &state.math.height)
                    .width(50)
                    .on_input(Message::HeightChanged),
                text("px")
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
            text("Preset"),
            pick_list(
                MathPreset::all(),
                Some(state.viz.math_preset),
                Message::PresetChanged,
            )
            .width(200),
            button("Apply").on_press_maybe(if state.runtime.computing {
                None
            } else {
                Some(Message::PresetClicked)
            }),
            text("Max. iterations:"),
            text_input("", &state.math.max_iteration)
                .width(100)
                .on_input(Message::MaxIterationChanged),
            text("Right/Top:"),
            row![
                text_input("", &state.math.right)
                    .width(100)
                    .on_input(Message::RightChanged),
                text("/"),
                text_input("", &state.math.top)
                    .width(100)
                    .on_input(Message::TopChanged),
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
            text("Left/Bottom:"),
            row![
                text_input("", &state.math.left)
                    .width(100)
                    .on_input(Message::LeftChanged),
                text("/"),
                text_input("", &state.math.bottom)
                    .width(100)
                    .on_input(Message::BottomChanged),
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
            if state.runtime.computing {
                button("Stop").on_press(Message::StopClicked)
            } else {
                button("Compute").on_press(Message::ComputeClicked)
            },
            if let Some(storage) = &state.storage {
                if storage.stage.is_fully_computed() {
                    Element::from(text("✓ Complete"))
                } else {
                    Element::from(
                        progress_bar(0.0..=1.0, storage.stage.computed_ratio()).width(100),
                    )
                }
            } else {
                Element::from(text("Waiting…"))
            },
            text("Color scheme:"),
            pick_list(
                GradientColorPreset::all(),
                Some(state.viz.gradient_color_preset),
                Message::ColorSchemeChanged,
            )
            .width(150),
            text("Iteration Mapping:"),
            pick_list(
                IterationAssignment::all(),
                Some(state.viz.iteration_assignment),
                Message::IterationAssignmentChanged,
            )
            .width(150),
            text("Render scheme:"),
            pick_list(
                ImageRenderScheme::all(),
                Some(state.viz.render_scheme),
                Message::RenderSchemeChanged,
            )
            .width(150),
        ]
        .spacing(6)
        .align_x(iced::Alignment::Start),
    )
    .width(Length::Shrink)
    .into()
}

fn collapsed_sidebar(_state: &AppState) -> Element<Message> {
    container(button(">").on_press(Message::ToggleSidebar))
        .width(Length::Shrink)
        .into()
}

fn fractal(state: &AppState) -> Element<Message> {
    container(if let Some(_) = &state.storage {
        column![render_fractal(state)].spacing(10)
    } else {
        column![text("")]
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

pub fn view(state: &AppState) -> Element<Message> {
    row![
        if state.viz.sidebar_visible {
            open_sidebar(state)
        } else {
            collapsed_sidebar(state)
        },
        fractal(state)
    ]
    .spacing(10)
    .padding(10)
    .into()
}

// end of file
