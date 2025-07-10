use std::time::{Duration, Instant};

use crate::gui::iced::app::AppState;
use crate::gui::iced::message::Message;

pub fn subscription(state: &AppState) -> iced::Subscription<Message> {
    println!("GGG - s - A, called (at {:?})", Instant::now());
    if state.viz.auto_start_computation {
        iced::Subscription::run(|| {
            async_stream::stream! {
                yield Message::ComputeClicked;
            }
        })
    } else if state.runtime.zoom_timer.is_some() {
        println!("GGG - s - B, creating timer subscription");
        iced::Subscription::run(|| {
            async_stream::stream! {
                loop {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    yield Message::ZoomTimerCheck;
                }
            }
        })
    } else {
        iced::Subscription::none()
    }
}

// end of file
