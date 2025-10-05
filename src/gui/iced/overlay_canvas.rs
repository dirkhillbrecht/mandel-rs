//! Overlay for the fractal canvas showing things like helper lines during shifting the image

use std::f32::consts::PI;

use crate::gui::iced::{app::AppState, message::Message};
use iced::{
    Color, Point, event,
    widget::canvas::{self, Event, Frame, Stroke},
};

pub struct OverlayCanvas<'a> {
    /// Reference to current application state (fractal data, settings, etc.)
    pub app_state: &'a AppState,
}

impl<'a> OverlayCanvas<'a> {
    /// Creates a new fractal canvas with the given application state.
    ///
    /// # Arguments
    ///
    /// * `app_state` - Reference to current application state
    ///
    /// # Returns
    ///
    /// Canvas ready for rendering and interaction
    pub fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }
}

/// Draw the center of the frame in case of a dragging operation is on the run
fn draw_center(frame: &mut Frame) {
    let col1 = Color::from_rgba8(32, 32, 32, 0.5);
    let col2 = Color::from_rgba8(224, 224, 224, 0.5);

    let center = frame.center();
    let radius1 = frame.size().height.min(frame.size().width) * 0.02;
    let radius2 = radius1 * 1.5;
    let init_angle = 0.0; // between 0 and PI/2.0
    (0..=3).for_each(|q| {
        let start_angle = q as f32 * (PI / 2.0) + init_angle;
        let end_angle = (q + 1) as f32 * (PI / 2.0) + init_angle;
        {
            // Draw the inner pie slice
            let mut pb = canvas::path::Builder::new();
            // First, a triangle…
            pb.move_to(center);
            pb.line_to(Point::new(
                center.x + radius1 * start_angle.cos(),
                center.y + radius1 * start_angle.sin(),
            ));
            pb.line_to(Point::new(
                center.x + radius1 * end_angle.cos(),
                center.y + radius1 * end_angle.sin(),
            ));
            pb.close(); // The triangle is closed with this call
            // And then an added to it. For whatever reason, this one is also filled below
            // It does not seem possible to add the line and arc segments to create a "pie slice" in one piece.
            pb.arc(canvas::path::Arc {
                center,
                radius: radius1,
                start_angle: iced::Radians(start_angle),
                end_angle: iced::Radians(end_angle),
            });
            let qpath = pb.build(); // Must be stored in its own variable to be referenced below
            // Fill everything to create the inner pie slice
            frame.fill(&qpath, if q & 1 == 1 { col1 } else { col2 });
        }
        {
            // Draw the outer "pie extension"
            let mut pb = canvas::path::Builder::new();
            // Only create one _unclosed_ circle arc in the middle of the two radii…
            pb.arc(canvas::path::Arc {
                center,
                radius: (radius1 + radius2) / 2.0,
                start_angle: iced::Radians(start_angle),
                end_angle: iced::Radians(end_angle),
            });
            let qpath = pb.build();
            // …and then stroke it with a width of the distance between inner and outer radius.
            frame.stroke(
                &qpath,
                Stroke {
                    style: canvas::Style::Solid(if q & 1 == 1 { col2 } else { col1 }),
                    width: radius2 - radius1,
                    ..Stroke::default()
                },
            );
        }
    });
}

impl<'a> canvas::Program<Message> for OverlayCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        canvas_bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        if self.app_state.runtime.canvas_is_dragging {
            let canvas_size = canvas_bounds.size();
            let circle_geometry = {
                let mut frame = canvas::Frame::new(renderer, canvas_size);
                draw_center(&mut frame);
                frame.into_geometry()
            };
            vec![circle_geometry]
        } else {
            vec![]
        }
    }

    fn update(
        &self,
        _: &mut Self::State,
        _: Event,
        _: iced::Rectangle,
        _: iced::mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        (event::Status::Ignored, None)
    }
}
