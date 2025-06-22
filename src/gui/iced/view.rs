/// View part of the mandel-rs-Iced-GUI
use crate::gui::iced::app::MandelRSApp;
use crate::gui::iced::message::Message;
use crate::storage::visualization::coloring::base::GradientColors;
use crate::storage::visualization::coloring::presets::GradientColorPresets;
use crate::storage::visualization::viz_storage::VizStorage;
use iced::widget::{button, column, container, progress_bar, row, text, text_input};
use iced::{Element, Length};

fn render_fractal<'a>(_state: &'a MandelRSApp, storage: &'a VizStorage) -> Element<'a, Message> {
    use iced::widget::image;

    let width = storage.stage.width();
    let height = storage.stage.height();

    let color_scheme = GradientColors::new(&GradientColorPresets::Sunrise.scheme(), 256);

    let mut pixels = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if let Some(point) = storage.stage.get(x, y) {
                let color = color_scheme
                    .iteration_to_color(point.iteration_count(), storage.properties.max_iteration);
                pixels.extend_from_slice(&color);
                pixels.push(255);
            } else {
                pixels.extend_from_slice(&[255, 0, 255, 255]);
            }
        }
    }
    let handle = image::Handle::from_rgba(width as u32, height as u32, pixels);
    image(handle).content_fit(iced::ContentFit::Contain).into()
}

fn open_sidebar(state: &MandelRSApp) -> Element<Message> {
    container(
        column![
            button("<").on_press(Message::ToggleSidebar),
            text("Computed size:"),
            row![
                text_input("", &state.width)
                    .width(50)
                    .on_input(Message::WidthChanged),
                text("*"),
                text_input("", &state.height)
                    .width(50)
                    .on_input(Message::HeightChanged),
                text("px")
            ]
            .spacing(6)
            .align_y(iced::Alignment::Center),
            text("Max. iterations:"),
            text_input("", &state.max_iteration)
                .width(100)
                .on_input(Message::MaxIterationChanged),
            text("Right:"),
            text_input("", &state.right)
                .width(100)
                .on_input(Message::RightChanged),
            text("Top:"),
            text_input("", &state.top)
                .width(100)
                .on_input(Message::TopChanged),
            text("Left:"),
            text_input("", &state.left)
                .width(100)
                .on_input(Message::LeftChanged),
            text("Bottom:"),
            text_input("", &state.bottom)
                .width(100)
                .on_input(Message::BottomChanged),
            if state.computing {
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
            }
        ]
        .spacing(6)
        .align_x(iced::Alignment::Start),
    )
    .width(Length::Shrink)
    .into()
}

fn collapsed_sidebar(_state: &MandelRSApp) -> Element<Message> {
    container(button(">").on_press(Message::ToggleSidebar))
        .width(Length::Shrink)
        .into()
}

fn fractal(state: &MandelRSApp) -> Element<Message> {
    container(if let Some(storage) = &state.storage {
        column![render_fractal(state, storage)].spacing(10)
    } else {
        column![text("")]
    })
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

pub fn view(state: &MandelRSApp) -> Element<Message> {
    row![
        if state.sidebar_visible {
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
