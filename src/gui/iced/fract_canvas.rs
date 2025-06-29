// Code for the central canvas where the interaction with the fractal image happens

use crate::{
    gui::iced::{app::AppState, message::Message},
    storage::{
        coord_spaces::{PixelSpace, StageSpace},
        data_point::DataPoint,
        visualization::{coloring::base::GradientColors, viz_storage::VizStorage},
    },
};
use euclid::Point2D;
use iced::{
    mouse,
    widget::{
        canvas::{self, event, Event},
        image::Handle,
    },
    Point, Size,
};

struct Pixels {
    origin: Point<usize>,
    size: Size<usize>,
    pixels: Vec<u8>,
}

impl Pixels {
    pub fn new(origin: Point<usize>, size: Size<usize>, pixels: Vec<u8>) -> Self {
        Pixels {
            origin,
            size,
            pixels,
        }
    }
    pub fn at_zero_origin(size: Size<usize>, pixels: Vec<u8>) -> Self {
        Self::new(Point::new(0, 0), size, pixels)
    }
    pub fn extract_center(&self, new_aspect_ratio: f32) -> Option<Pixels> {
        let old_aspect_ratio = self.size.width as f32 / self.size.height as f32;
        if (old_aspect_ratio - new_aspect_ratio).abs() < 1e-3 {
            // almost the same: Don't extract anything
            None
        } else if old_aspect_ratio > new_aspect_ratio {
            // get horizontal mid section of all lines
            let new_width = (self.size.height as f32 * new_aspect_ratio) as usize;
            let line_start = (self.size.width - new_width) / 2;
            let mut new_pixels = Vec::with_capacity(new_width * self.size.height * 4);
            for line in 0..self.size.height {
                let firstpix = (line * self.size.width + line_start) * 4;
                new_pixels.extend_from_slice(&self.pixels[firstpix..firstpix + new_width * 4]);
            }
            Some(Pixels::new(
                Point::new(line_start, 0),
                Size::new(new_width, self.size.height),
                new_pixels,
            ))
        } else {
            // get the complete lines in the vertical middle
            let new_height = (self.size.width as f32 / new_aspect_ratio) as usize;
            let first_line = (self.size.height - new_height) / 2;
            let firstpix = self.size.width * first_line * 4;
            let mut new_pixels = Vec::with_capacity(self.size.width * new_height * 4);
            new_pixels.extend_from_slice(
                &self.pixels[firstpix..firstpix + self.size.width * new_height * 4],
            );
            Some(Pixels::new(
                Point::new(0, first_line),
                Size::new(self.size.width, new_height),
                new_pixels,
            ))
        }
    }
    pub fn change_alpha(&mut self, new_alpha: f32) {
        let a = (new_alpha * 255.0) as u8;
        for p in 0..self.size.width * self.size.height {
            self.pixels[(p * 4) + 3] = a;
        }
    }
}

pub struct CanvasState {
    drag_start: Option<Point>,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self { drag_start: None }
    }
}

pub struct FractalCanvas<'a> {
    pub app_state: &'a AppState,
}

impl<'a> FractalCanvas<'a> {
    /// Create a new canvas with the current application state
    pub fn new(app_state: &'a AppState) -> Self {
        FractalCanvas { app_state }
    }
    /// Get one pixel from the canvas, return none if no pixel has been computed
    fn get_pixel(&self, storage: &'a VizStorage, x: usize, y: usize) -> Option<&'a DataPoint> {
        storage.stage.get(x, y)
    }
    /// Guess a pixel from already computed values, call only if get_pixel returned None
    fn guess_pixel(&self, storage: &VizStorage, x: usize, y: usize) -> Option<DataPoint> {
        let mut modrest = 2;
        while modrest < x || modrest < y {
            if let Some(guesspix) = storage.stage.get(x - (x % modrest), y - (y % modrest)) {
                return Some(guesspix.as_guessed());
            }
            modrest *= 2;
        }
        None
    }
    fn generate_pixel(
        &self,
        storage: &VizStorage,
        color_scheme: &GradientColors,
        point: &DataPoint,
    ) -> [u8; 4] {
        color_scheme.iteration_to_color(
            point.iteration_count,
            self.app_state
                .viz
                .iteration_assignment
                .assignment_function(),
            storage.properties.max_iteration,
        )
    }
    /// Actually create the pixels needed in the canvas.
    fn create_pixels(&self) -> Option<Pixels> {
        if let Some(storage) = self.app_state.storage.as_ref() {
            let width = storage.stage.width();
            let height = storage.stage.height();

            // TODO: Move color_scheme to the app_state to prevent permanent recomputation
            let color_scheme =
                GradientColors::new(&self.app_state.viz.gradient_color_preset.scheme(), 256);

            let mut pixels = Vec::new();
            for y in 0..height {
                for x in 0..width {
                    if let Some(point) = self.get_pixel(storage, x, y) {
                        // computed points: handled as reference in the storage
                        pixels.extend_from_slice(&self.generate_pixel(
                            storage,
                            &color_scheme,
                            point,
                        ));
                    } else if let Some(point) = self.guess_pixel(storage, x, y) {
                        // guessed points: Have to be generated on the fly
                        pixels.extend_from_slice(&self.generate_pixel(
                            storage,
                            &color_scheme,
                            &point,
                        ));
                    } else {
                        // unknown points: A nice neutral grey…
                        let pix = 128;
                        pixels.extend_from_slice(&[pix, pix, pix, 255]);
                    }
                }
            }
            Some(Pixels::at_zero_origin(Size::new(width, height), pixels))
        } else {
            None
        }
    }
    /// convert some pixel coordinates into coordinates on the stage
    fn pixel_to_stage(
        pixel: &Point,
        bounds: &iced::Rectangle,

        pixels: &Pixels,
    ) -> Option<Point2D<u32, StageSpace>> {
        None
    }
}

impl<'a> canvas::Program<Message> for FractalCanvas<'a> {
    type State = CanvasState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        let canvas_size = bounds.size();
        let geometry = self
            .app_state
            .runtime
            .canvas_cache
            .draw(renderer, canvas_size, |frame| {
                if let Some(pixels) = self.create_pixels() {
                    let render_scheme = self.app_state.viz.render_scheme;
                    if render_scheme.needs_cropped() {
                        if let Some(mut croppixels) =
                            pixels.extract_center(canvas_size.width / canvas_size.height)
                        {
                            if render_scheme.needs_background_cropped() {
                                croppixels.change_alpha(0.4);
                            }
                            let image = canvas::Image::new(Handle::from_rgba(
                                croppixels.size.width as u32,
                                croppixels.size.height as u32,
                                croppixels.pixels,
                            ))
                            .filter_method(iced::widget::image::FilterMethod::Linear);
                            frame.draw_image(iced::Rectangle::with_size(canvas_size), image);
                        }
                    }
                    if render_scheme.needs_filled() {
                        let stage_size =
                            Size::new(pixels.size.width as f32, pixels.size.height as f32);

                        let canvas_by_stage = Size::new(
                            canvas_size.width / stage_size.width,
                            canvas_size.height / stage_size.height,
                        );

                        let mut scale_min = canvas_by_stage.width.min(canvas_by_stage.height);
                        if !render_scheme.needs_upscaled_filled() {
                            scale_min = scale_min.min(1.0);
                        }

                        let draw_size =
                            Size::new(stage_size.width * scale_min, stage_size.height * scale_min);

                        let image = canvas::Image::new(Handle::from_rgba(
                            pixels.size.width as u32,
                            pixels.size.height as u32,
                            pixels.pixels,
                        ))
                        .filter_method(iced::widget::image::FilterMethod::Linear);

                        let draw_rect = iced::Rectangle::new(
                            iced::Point::new(
                                (canvas_size.width - draw_size.width) / 2.0,
                                (canvas_size.height - draw_size.height) / 2.0,
                            ),
                            draw_size,
                        );
                        frame.draw_image(draw_rect, image);
                    }
                }
            });
        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        state.drag_start = cursor.position();
                        if (state.drag_start.is_none()) {
                            (event::Status::Ignored, None)
                        } else {
                            println!("GGG - button pressed at {:?}", state.drag_start);
                            // do something
                            (event::Status::Captured, None)
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        if let Some(drag_start) = state.drag_start {
                            println!(
                                "GGG - handling a cursor moved at position {:?}, bounds are {:?}",
                                position, bounds
                            );
                            // do something
                            (event::Status::Captured, None)
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if let Some(drag_start) = state.drag_start {
                            if let Some(drag_stop) = cursor.position() {
                                println!("GGG - button released at {:?}", drag_stop);
                                // do something
                            }
                            state.drag_start = None; // In any case, dragging is ended.
                            (event::Status::Captured, None)
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    _ => (event::Status::Ignored, None),
                }
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
