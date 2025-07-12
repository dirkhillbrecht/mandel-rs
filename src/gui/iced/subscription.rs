use std::time::Duration;

use crate::gui::iced::app::AppState;
use crate::gui::iced::message::Message;

pub fn subscription(state: &AppState) -> iced::Subscription<Message> {
    if state.viz.auto_start_computation {
        iced::Subscription::run(|| {
            async_stream::stream! {
                yield Message::ComputeClicked;
            }
        })
    } else if state.runtime.zoom.is_some() {
        iced::Subscription::run(|| {
            async_stream::stream! {
                loop {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    yield Message::ZoomEndCheck;
                }
            }
        })
    } else {
        iced::Subscription::none()
    }
}

// end of file
