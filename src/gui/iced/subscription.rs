use crate::gui::iced::app::MandelRSApp;
use crate::gui::iced::message::Message;

pub fn subscription(state: &MandelRSApp) -> iced::Subscription<Message> {
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

// end of file
