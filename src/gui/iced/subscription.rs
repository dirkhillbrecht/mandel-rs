use crate::gui::iced::app::AppState;
use crate::gui::iced::message::Message;

pub fn subscription(state: &AppState) -> iced::Subscription<Message> {
    if state.viz.auto_start_computation {
        iced::Subscription::run(|| {
            async_stream::stream! {
                yield Message::ComputeClicked;
            }
        })
    } else {
        iced::Subscription::none()
    }
}

// end of file
