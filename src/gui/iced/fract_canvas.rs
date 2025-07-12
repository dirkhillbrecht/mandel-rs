// Code for the central canvas where the interaction with the fractal image happens

use crate::{
    gui::iced::{
        app::{AppState, ImageRenderScheme, ZoomState},
        message::Message,
    },
    storage::{
        coord_spaces::StageSpace,
        data_point::DataPoint,
        visualization::{coloring::base::GradientColors, viz_storage::VizStorage},
    },
};
use euclid::Vector2D;
use iced::{
    Point, Rectangle, Size,
    mouse::{self, ScrollDelta},
    widget::{
        canvas::{self, Event, event},
        image::Handle,
    },
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
    pub fn extract_part(&self, image_part: iced::Rectangle) -> Pixels {
        let new_linestart = image_part.x.abs() as usize;
        let new_firstline: usize = image_part.y.abs() as usize;
        let new_size = Size::new(image_part.width as usize, image_part.height as usize);
        let bytecount = new_size.width * new_size.height * 4;
        let mut new_pixels = Vec::with_capacity(bytecount);
        if new_linestart == 0 && new_size.width == self.size.width {
            // Copy one chunk covering the given number of lines
            let firstpix = self.size.width * image_part.y as usize * 4;
            new_pixels.extend_from_slice(&self.pixels[firstpix..firstpix + bytecount]);
        } else {
            // Copy part of each line over the whole height
            for line in new_firstline..new_firstline + new_size.height {
                let firstpix = (line * self.size.width + new_linestart) * 4;
                new_pixels.extend_from_slice(&self.pixels[firstpix..firstpix + new_size.width * 4]);
            }
        }
        Self::at_zero_origin(new_size, new_pixels)
    }
    pub fn extract_part_if_needed(&self, image_part: iced::Rectangle) -> Option<Pixels> {
        if image_part.x.abs() as usize == self.origin.x
            && image_part.y.abs() as usize == self.origin.y
            && image_part.width.abs() as usize == self.size.width
            && image_part.height.abs() as usize == self.size.height
        {
            None
        } else {
            Some(self.extract_part(image_part))
        }
    }
    /// Generate an independent pixels instance where the pixels are shifted by the given offset
    /// The "new" pixels are painted black.
    /// If the offset is 0, nothing is returned
    pub fn shift(&self, offset: Size) -> Option<Pixels> {
        if offset.width.abs() < 1e-2 && offset.height.abs() < 1e-2 {
            None
        } else {
            let ox = offset.width as i32;
            let oy = offset.height as i32;
            let empty_line_start = (ox.max(0) as usize).min(self.size.width);
            let empty_line_end = ((-ox).max(0) as usize).min(self.size.width);
            let empty_start_lines = (oy.max(0) as usize).min(self.size.height);
            let empty_end_lines = ((-oy).max(0) as usize).min(self.size.height);
            let line_width = self.size.width - (empty_line_start.max(empty_line_end));
            let first_line = empty_end_lines;
            let last_line = self.size.height - empty_start_lines;
            let mut new_pixels = Vec::with_capacity(self.size.width * self.size.height * 4);
            let one_pixel: [u8; 4] = [0, 0, 0, 0];
            for _ in 0..empty_start_lines {
                for _ in 0..self.size.width {
                    new_pixels.extend_from_slice(&one_pixel);
                }
            }
            for line in first_line..last_line {
                for _ in 0..empty_line_start {
                    new_pixels.extend_from_slice(&one_pixel);
                }
                let first_idx = (line * self.size.width + empty_line_end) * 4;
                let last_idx = first_idx + line_width as usize * 4;
                new_pixels.extend_from_slice(&self.pixels[first_idx..last_idx]);
                for _ in 0..empty_line_end {
                    new_pixels.extend_from_slice(&one_pixel);
                }
            }
            for _ in 0..empty_end_lines {
                for _ in 0..self.size.width {
                    new_pixels.extend_from_slice(&one_pixel);
                }
            }
            Some(Pixels::at_zero_origin(self.size, new_pixels))
        }
    }
    pub fn zoom(&self, zoom_state: &ZoomState) -> Option<Pixels> {
        if zoom_state.ticks == 0 {
            None
        } else {
            let one_pixel: [u8; 4] = [0, 0, 0, 0];
            let zoom_part = 1.0 - 1.0 / zoom_state.factor;
            let leftpix = zoom_state.origin.x * zoom_part;
            let toppix = zoom_state.origin.y * zoom_part;
            let mut new_pixels = Vec::with_capacity(self.size.width * self.size.height * 4);
            let mut newx = Vec::with_capacity(self.size.width);
            for x in 0..self.size.width {
                newx.push(leftpix + x as f32 / zoom_state.factor);
            }
            for y in 0..self.size.height {
                let newy = (toppix + y as f32 / zoom_state.factor) as i32;
                for x in 0..self.size.width {
                    let newx = (leftpix + x as f32 / zoom_state.factor) as i32;
                    if newx < 0
                        || newx >= self.size.width as i32
                        || newy < 0
                        || newy >= self.size.height as i32
                    {
                        new_pixels.extend_from_slice(&one_pixel);
                    } else {
                        let first_idx = (self.size.width as i32 * newy + newx)
                            .max(0)
                            .min((self.size.width * self.size.height) as i32 - 1)
                            as usize
                            * 4;
                        new_pixels.extend_from_slice(&self.pixels[first_idx..first_idx + 4]);
                    }
                }
            }
            Some(Pixels::at_zero_origin(self.size, new_pixels))
        }
    }
    pub fn change_alpha(&mut self, new_alpha: f32) {
        let a = (new_alpha * 255.0) as u8;
        for p in 0..self.size.width * self.size.height {
            self.pixels[(p * 4) + 3] = a;
        }
    }
}

/// Parts of canvas and image actually used for presentation
/// Define which part of the image is drawn into which part of the canvas
/// It is up to the creator of this struct to make sure that aspect ratio is correct.
#[derive(Debug)]
struct UsedParts {
    /// Used image part: upper left point, width and height in pixels
    /// Defines: Which part of the image is actually drawn
    /// Cannot be larger then the actual image.
    pub used_image_part: iced::Rectangle,
    /// Used canvas part: upper left point relative to the actual canvas area, width and height
    /// Defines: Into which part of the canvas is the used part of the image drawn
    /// Cannot be larger than the actual canvas.
    pub used_canvas_part: iced::Rectangle,
}

/// Data for relating image and canvas so that coordinates can be transformed back and forth
#[derive(Debug)]
struct ImageInCanvas {
    /// Original canvas bounds including the canvas base point (for mouse coordinate translation)
    pub canvas_bounds: iced::Rectangle,
    /// The original image size (top-left is always (0,0))
    pub image_size: Size<f32>,
    /// Actually used parts of image and canvas
    pub used_parts: UsedParts,
}

impl ImageInCanvas {
    pub fn init(
        canvas_bounds: iced::Rectangle,
        image_size: Size<f32>,
        render_scheme: ImageRenderScheme,
    ) -> Self {
        let canvas_size = canvas_bounds.size();
        ImageInCanvas {
            canvas_bounds,
            image_size,
            used_parts: match render_scheme {
                ImageRenderScheme::Cropped => UsedParts::cropped_bounds(canvas_size, image_size),
                ImageRenderScheme::FilledWithBackground | ImageRenderScheme::Filled => {
                    UsedParts::filled_bounds(canvas_size, image_size, true)
                }
                ImageRenderScheme::ShrunkWithBackground | ImageRenderScheme::Shrunk => {
                    UsedParts::filled_bounds(canvas_size, image_size, false)
                }
                ImageRenderScheme::CenteredWithBackground | ImageRenderScheme::Centered => {
                    UsedParts::centered_bounds(canvas_size, image_size)
                }
            },
        }
    }

    // Create an image in canvas instance for a certain app state and some canvas bounds
    pub fn for_app_state_and_bounds(
        app_state: &AppState,
        canvas_bounds: Rectangle,
    ) -> Option<Self> {
        if let Some(storage) = &app_state.storage {
            Some(ImageInCanvas::init(
                canvas_bounds,
                Size::new(storage.stage.width() as f32, storage.stage.height() as f32),
                app_state.viz.render_scheme,
            ))
        } else {
            None
        }
    }

    /// Convert a mouse coordinate into the point on the image it points on
    /// Due to image positioning and scaling, these have to be converted.
    pub fn mouse_to_image(&self, mouse_on_screen: Point) -> Point {
        // Convert app-global coordinates to canvas-origin
        let mouse_on_canvas = Point::new(
            mouse_on_screen.x - self.canvas_bounds.x,
            mouse_on_screen.y - self.canvas_bounds.y,
        );
        // Convert from canvas to actually used canvas
        let mouse_on_used_canvas = Point::new(
            mouse_on_canvas.x - self.used_parts.used_canvas_part.x,
            mouse_on_canvas.y - self.used_parts.used_canvas_part.y,
        );
        // Convert from on-screen pixels to the pixels of the - potentially scaled - image
        Point::new(
            self.used_parts.used_image_part.x
                + (mouse_on_used_canvas.x * self.used_parts.used_image_part.width
                    / self.used_parts.used_canvas_part.width),
            self.used_parts.used_image_part.y
                + (mouse_on_used_canvas.y * self.used_parts.used_image_part.height
                    / self.used_parts.used_canvas_part.height),
        )
    }

    /// Convert the mouse coordinate into image point if the mouse pointer is inside the image
    pub fn mouse_to_image_if_valid(&self, mouse_on_screen: Point) -> Option<Point> {
        Some(self.mouse_to_image(mouse_on_screen)).filter(|p| {
            p.x >= 0.0
                && p.x <= self.image_size.width
                && p.y >= 0.0
                && p.y <= self.image_size.height
        })
    }
}

impl UsedParts {
    /// Generate parts which crop the picture in the canvas
    /// In this case, always the complete canvas is used and the image is cropped
    pub fn cropped_bounds(canvas_size: Size<f32>, image_size: Size<f32>) -> Self {
        let used_canvas_part = Rectangle::new(Point::new(0.0, 0.0), canvas_size);
        let canvas_aspect_ratio = canvas_size.width / canvas_size.height;
        let image_aspect_ratio = image_size.width / image_size.height;
        if image_aspect_ratio < canvas_aspect_ratio {
            // image narrower than canvas, takes all image width, mid of image height
            let new_image_height = image_size.width / canvas_aspect_ratio;
            let new_image_top = (image_size.height - new_image_height).max(0.0) / 2.0;
            UsedParts {
                used_image_part: Rectangle::new(
                    Point::new(0.0, new_image_top),
                    Size::new(image_size.width, new_image_height),
                ),
                used_canvas_part,
            }
        } else {
            // image wider than canvas, takes all image height, mid of image width
            let new_image_width = image_size.height * canvas_aspect_ratio;
            let new_image_left = (image_size.width - new_image_width).max(0.0) / 2.0;
            UsedParts {
                used_image_part: Rectangle::new(
                    Point::new(new_image_left, 0.0),
                    Size::new(new_image_width, image_size.height),
                ),
                used_canvas_part,
            }
        }
    }
    /// Generate bounds which show the complete image in the canvas
    /// In this case, always the complete image is used but potentially only parts of the canvas
    fn filled_bounds(canvas_size: Size<f32>, image_size: Size<f32>, upscale: bool) -> Self {
        let used_image_part = Rectangle::new(Point::new(0.0, 0.0), image_size);

        let canvas_by_stage = Size::new(
            canvas_size.width / image_size.width,
            canvas_size.height / image_size.height,
        );

        let mut scale_min = canvas_by_stage.width.min(canvas_by_stage.height);
        if !upscale {
            scale_min = scale_min.min(1.0);
        }

        let used_canvas_size =
            Size::new(image_size.width * scale_min, image_size.height * scale_min);

        UsedParts {
            used_image_part,
            used_canvas_part: Rectangle::new(
                Point::new(
                    ((canvas_size.width - used_canvas_size.width) / 2.0).max(0.0),
                    ((canvas_size.height - used_canvas_size.height) / 2.0).max(0.0),
                ),
                used_canvas_size,
            ),
        }
    }
    /// Generate bounds which show the unscaled (center of the) image in the center of the canvas
    /// In this case, the image is always unscaled and fills either a part of the canvas
    /// or is not shown fully
    fn centered_bounds(canvas_size: Size<f32>, image_size: Size<f32>) -> Self {
        let (image_left, image_width, canvas_left, canvas_width) =
            if image_size.width <= canvas_size.width {
                (
                    0.0,
                    image_size.width,
                    (canvas_size.width - image_size.width) / 2.0,
                    image_size.width,
                )
            } else {
                (
                    (image_size.width - canvas_size.width) / 2.0,
                    canvas_size.width,
                    0.0,
                    canvas_size.width,
                )
            };
        let (image_top, image_height, canvas_top, canvas_height) =
            if image_size.height <= canvas_size.height {
                (
                    0.0,
                    image_size.height,
                    (canvas_size.height - image_size.height) / 2.0,
                    image_size.height,
                )
            } else {
                (
                    (image_size.height - canvas_size.height) / 2.0,
                    canvas_size.height,
                    0.0,
                    canvas_size.height,
                )
            };
        UsedParts {
            used_image_part: Rectangle::new(
                Point::new(image_left, image_top),
                Size::new(image_width, image_height),
            ),
            used_canvas_part: Rectangle::new(
                Point::new(canvas_left, canvas_top),
                Size::new(canvas_width, canvas_height),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanvasOperation {
    Idle,
    Drag,
}

pub struct CanvasState {
    operation: CanvasOperation,
    start_pixel: Option<Point>, // Rename (operation_start?)
    drag_shift: Option<Size>,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            operation: CanvasOperation::Idle,
            start_pixel: None,
            drag_shift: None,
        }
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

            let mut pixels = Vec::with_capacity(width * height * 4);
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
    /// Helper method to derive a zoom tick out of a mouse wheel movement
    fn mouse_wheel_to_zoom_tick(delta: ScrollDelta) -> i32 {
        match delta {
            mouse::ScrollDelta::Lines { y, .. } => {
                if y.abs() > 1e-5 {
                    y.signum() as i32
                } else {
                    0
                }
            }
            mouse::ScrollDelta::Pixels { y, .. } => {
                if y.abs() > 1e-5 {
                    y.signum() as i32
                } else {
                    0
                }
            }
        }
    }
}

impl<'a> canvas::Program<Message> for FractalCanvas<'a> {
    type State = CanvasState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        canvas_bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        let canvas_size = canvas_bounds.size();
        let geometry = self
            .app_state
            .runtime
            .canvas_cache
            .draw(renderer, canvas_size, |frame| {
                if let Some(rawpixels) = self.create_pixels() {
                    let pixels = if let Some(drag_shift) = state.drag_shift {
                        rawpixels.shift(drag_shift).unwrap_or(rawpixels)
                    } else if let Some(zoom) = &self.app_state.runtime.zoom
                        && zoom.ticks != 0
                    {
                        rawpixels.zoom(zoom).unwrap_or(rawpixels)
                    } else {
                        rawpixels
                    };
                    let render_scheme = self.app_state.viz.render_scheme;
                    let image_size = Size::new(pixels.size.width as f32, pixels.size.height as f32);
                    if render_scheme.needs_background_cropped()
                        && let None = state.start_pixel
                        && let None = self.app_state.runtime.zoom
                    {
                        let background_mgr = ImageInCanvas::init(
                            canvas_bounds,
                            image_size,
                            ImageRenderScheme::Cropped,
                        );
                        if let Some(mut background_pixels) =
                            pixels.extract_part_if_needed(background_mgr.used_parts.used_image_part)
                        {
                            background_pixels.change_alpha(0.4);
                            let image = canvas::Image::new(Handle::from_rgba(
                                background_pixels.size.width as u32,
                                background_pixels.size.height as u32,
                                background_pixels.pixels,
                            ))
                            .filter_method(iced::widget::image::FilterMethod::Linear);
                            frame.draw_image(background_mgr.used_parts.used_canvas_part, image);
                        }
                    }
                    let foreground_mgr =
                        ImageInCanvas::init(canvas_bounds, image_size, render_scheme);
                    let foreground_pixels = pixels
                        .extract_part_if_needed(foreground_mgr.used_parts.used_image_part)
                        .unwrap_or(pixels);
                    let image = canvas::Image::new(Handle::from_rgba(
                        foreground_pixels.size.width as u32,
                        foreground_pixels.size.height as u32,
                        foreground_pixels.pixels,
                    ))
                    .filter_method(iced::widget::image::FilterMethod::Linear);
                    frame.draw_image(foreground_mgr.used_parts.used_canvas_part, image);
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
                        if state.operation == CanvasOperation::Idle
                            && let Some(position) = cursor.position()
                            && let Some(point) =
                                ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                                    .and_then(|iic| iic.mouse_to_image_if_valid(position))
                        {
                            state.operation = CanvasOperation::Drag;
                            state.start_pixel = Some(point);
                            state.drag_shift = None;
                            (event::Status::Captured, None)
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    mouse::Event::CursorMoved { position } => {
                        if state.operation == CanvasOperation::Drag
                            && let Some(drag_start) = state.start_pixel
                            && let Some(image_in_canvas) =
                                ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                        {
                            let point = image_in_canvas.mouse_to_image(position);
                            state.drag_shift =
                                Some(Size::new(point.x - drag_start.x, point.y - drag_start.y))
                                    .filter(|p| p.width.abs() >= 1e-2 || p.height.abs() >= 1e-2);
                            self.app_state.runtime.canvas_cache.clear();
                            (event::Status::Captured, None)
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if state.operation == CanvasOperation::Drag
                            && let Some(drag_start) = state.start_pixel
                        {
                            state.operation = CanvasOperation::Idle;
                            state.start_pixel = None; // In any case, dragging is ended.
                            state.drag_shift = None;
                            if let Some(position) = cursor.position()
                                && let Some(image_in_canvas) =
                                    ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                            {
                                let drag_stop = image_in_canvas.mouse_to_image(position);
                                let pixel_offset: Vector2D<i32, StageSpace> = Vector2D::new(
                                    (drag_stop.x - drag_start.x) as i32,
                                    (drag_stop.y - drag_start.y) as i32,
                                );
                                self.app_state.runtime.canvas_cache.clear();
                                (
                                    event::Status::Captured,
                                    Some(Message::ShiftStage(pixel_offset)),
                                )
                            } else {
                                (event::Status::Ignored, None)
                            }
                        } else {
                            (event::Status::Ignored, None)
                        }
                    }
                    mouse::Event::WheelScrolled { delta } => {
                        if self.app_state.runtime.zoom.is_none()
                            && let Some(position) = cursor.position()
                            && let Some(point) =
                                ImageInCanvas::for_app_state_and_bounds(&self.app_state, bounds)
                                    .and_then(|iic| iic.mouse_to_image_if_valid(position))
                        {
                            let zoom_tick_sum = Self::mouse_wheel_to_zoom_tick(delta);
                            if zoom_tick_sum != 0 {
                                (
                                    event::Status::Captured,
                                    Some(Message::ZoomStart((point, zoom_tick_sum))),
                                )
                            } else {
                                (event::Status::Ignored, None)
                            }
                        } else {
                            let this_tick = Self::mouse_wheel_to_zoom_tick(delta);
                            if this_tick != 0 {
                                (event::Status::Captured, Some(Message::ZoomTick(this_tick)))
                            } else {
                                (event::Status::Ignored, None)
                            }
                        }
                    }
                    _ => (event::Status::Ignored, None),
                }
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
