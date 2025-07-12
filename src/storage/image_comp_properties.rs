// Collective storage for the computational image parameters.

use euclid::{Point2D, Rect, Size2D, Vector2D};

use crate::storage::coord_spaces::{MathSpace, StageSpace};

/// Properties of the computation stage, not only mathematical coordinates, but also pixel raster size
#[derive(Debug, Clone, Copy)]
pub struct StageProperties {
    /// The mathematical coordinates of the stage
    pub coo: Rect<f64, MathSpace>,
    pub pixels: Size2D<u32, StageSpace>,

    pub dotsize: Size2D<f64, MathSpace>,
    pub coo_base: Point2D<f64, MathSpace>,
    pub coo_correction: Vector2D<f64, MathSpace>,
}

impl StageProperties {
    /// Create a new stage properties instance
    pub fn new(coo: Rect<f64, MathSpace>, pixels: Size2D<u32, StageSpace>) -> StageProperties {
        let dotsize = Size2D::new(
            coo.width() / pixels.width as f64,
            coo.height() / pixels.height as f64,
        );
        let coo_correction = Vector2D::new(dotsize.width / 2.0, -dotsize.height / 2.0);
        let coo_base = Point2D::new(coo.min_x(), coo.max_y()) + coo_correction;
        StageProperties {
            coo,
            pixels,
            dotsize,
            coo_base,
            coo_correction,
        }
    }

    /// Convert a pixel offset into a math offset based on the local pixel sizes
    pub fn pixel_to_math_offset(
        &self,
        offset: Vector2D<i32, StageSpace>,
    ) -> Vector2D<f64, MathSpace> {
        Vector2D::new(
            offset.x as f64 * -self.dotsize.width,
            offset.y as f64 * self.dotsize.height,
        )
    }

    /// Shift the stage properties by the given amount of pixels while retaining size
    /// and aspect ratio of everything.
    pub fn shifted_clone_by_math(&self, offset: Vector2D<f64, MathSpace>) -> StageProperties {
        let new_coo = self.coo.translate(offset);
        let coo_base = Point2D::new(
            new_coo.min_x() + (self.dotsize.width / 2.0),
            new_coo.max_y() - (self.dotsize.height / 2.0),
        );
        StageProperties {
            coo: new_coo,
            pixels: self.pixels,
            dotsize: self.dotsize,
            coo_base,
            coo_correction: self.coo_correction,
        }
    }

    /// Shift the stage properties by the given amount of pixels while retaining size
    /// and aspect ratio of everything.
    pub fn shifted_clone_by_pixels(&self, offset: Vector2D<i32, StageSpace>) -> StageProperties {
        self.shifted_clone_by_math(self.pixel_to_math_offset(offset))
    }

    pub fn zoomed_clone_by_pixels(&self, origin: Point2D<i32, StageSpace>, factor: f64) -> Self {
        let math_origin = self.pix_to_math(origin);
        let new_dotsize = self.dotsize / factor;
        let new_coo_correction = Vector2D::new(new_dotsize.width / 2.0, -new_dotsize.height / 2.0);
        let new_coo_base = Point2D::new(
            math_origin.x - (origin.x as f64 * new_dotsize.width),
            math_origin.y + (origin.y as f64 * new_dotsize.height),
        );
        let new_top_left = new_coo_base - new_coo_correction;
        let new_bottom_right = new_top_left
            + Vector2D::new(
                new_dotsize.width * self.pixels.width as f64,
                -new_dotsize.height * self.pixels.height as f64,
            );
        let new_coo = Rect::from_points([new_top_left, new_bottom_right]);
        StageProperties {
            coo: new_coo,
            pixels: self.pixels,
            dotsize: new_dotsize,
            coo_base: new_coo_base,
            coo_correction: new_coo_correction,
        }
    }

    /// Return the mathematical x coordinate for the given pixel x coordinate
    pub fn x(&self, x_pix: i32) -> f64 {
        self.coo_base.x + x_pix as f64 * self.dotsize.width
    }
    /// Return the mathematical y coordinate for the given pixel y coordinate
    pub fn y(&self, y_pix: i32) -> f64 {
        self.coo_base.y - y_pix as f64 * self.dotsize.height
    }

    /// Return whether the given point is within the stage's bounds
    #[allow(dead_code)]
    pub fn is_valid_pix(&self, p: &Point2D<i32, StageSpace>) -> bool {
        p.x >= 0 && p.x < self.pixels.width as i32 && p.y >= 0 && p.y < self.pixels.height as i32
    }

    /// Return the mathematical coordiates of a pixel
    #[allow(dead_code)]
    pub fn pix_to_math(&self, pix: Point2D<i32, StageSpace>) -> Point2D<f64, MathSpace> {
        Point2D::new(self.x(pix.x), self.y(pix.y))
    }

    /// Return the mathematical coordinates of a pixel if it is within the image's bounds
    #[allow(dead_code)]
    pub fn pix_to_math_if_valid(
        &self,
        pix: Point2D<i32, StageSpace>,
    ) -> Option<Point2D<f64, MathSpace>> {
        Some(pix)
            .filter(|p| self.is_valid_pix(p))
            .map(|p| self.pix_to_math(p))
    }

    /// Return the pixel of the given math coordinates
    /// Returns out of bounds values if the given math coordinates are not within the image bounds
    #[allow(dead_code)]
    pub fn math_to_pix(&self, math: Point2D<f64, MathSpace>) -> Point2D<i32, StageSpace> {
        let x = ((math.x - self.coo_base.x) / self.dotsize.width).floor() as i32;
        let y = ((self.coo_base.y - math.y) / self.dotsize.height).floor() as i32;
        Point2D::new(x, y)
    }

    /// Return the pixel coordinates of some mathematical coordinates, if they are in bounds
    #[allow(dead_code)]
    pub fn math_to_pix_if_valid(
        &self,
        math: Point2D<f64, MathSpace>,
    ) -> Option<Point2D<i32, StageSpace>> {
        Some(self.math_to_pix(math)).filter(|p| self.is_valid_pix(p))
    }

    /// Return a rectified version of the stage, i.e. guarantee that the pixels of the image cover a square area.
    ///
    /// Width and height are unchanged, mathematical coordinates are shrunk or enlarged to keep the ratio.
    /// Here, the new coordinate rectangle is spun around the _center_ of the original one.
    ///
    /// # Arguments
    /// * `inner` - Flag whether the new coordinates should describe a rectangle _within_ or _outside_ the original one
    pub fn rectified(&self, inner: bool) -> StageProperties {
        let dotsize_min = self.dotsize.width.min(self.dotsize.height);
        let dotsize_max = self.dotsize.width.max(self.dotsize.height);
        if (1.0 - (dotsize_min / dotsize_max)) < 1e-5 {
            self.clone()
        } else {
            let dotsize = if inner { dotsize_min } else { dotsize_max };
            let center = Point2D::new(
                self.coo.min_x() + (self.coo.width() / 2.0),
                self.coo.min_y() + (self.coo.height() / 2.0),
            );
            let dist = Size2D::new(
                dotsize * ((self.pixels.width as f64) / 2.0),
                dotsize * ((self.pixels.height as f64) / 2.0),
            );
            StageProperties::new(
                Rect::from_points([center - dist, center + dist]),
                self.pixels,
            )
        }
    }
}

/// Mathematical properties of an image.
/// The settings here allow to reconstruct the mathematical base of an image.
#[derive(Debug, Clone, Copy)]
pub struct ImageCompProperties {
    pub stage_properties: StageProperties,
    pub max_iteration: u32,
}

impl ImageCompProperties {
    /// # Returns
    /// a new instance of image computation properties
    pub fn new(stage_properties: StageProperties, max_iteration: u32) -> Self {
        ImageCompProperties {
            stage_properties,
            max_iteration,
        }
    }
    /// # Returns
    /// some new ImageCompProperties which are rectified for pixels with a square area
    pub fn rectified(&self, inner: bool) -> Self {
        ImageCompProperties {
            stage_properties: self.stage_properties.rectified(inner),
            max_iteration: self.max_iteration,
        }
    }

    /// Shift the stage properties by the given amount of pixels while retaining size
    /// and aspect ratio of everything.
    pub fn shifted_clone_by_pixels(&self, offset: Vector2D<i32, StageSpace>) -> Self {
        ImageCompProperties {
            stage_properties: self.stage_properties.shifted_clone_by_pixels(offset),
            max_iteration: self.max_iteration,
        }
    }

    /// Zoom the stage properties around the given origin pixel by the given factor
    /// while retaining size of the stage and aspect ratio
    pub fn zoomed_clone_by_pixels(&self, origin: Point2D<i32, StageSpace>, factor: f32) -> Self {
        ImageCompProperties {
            stage_properties: self
                .stage_properties
                .zoomed_clone_by_pixels(origin, factor as f64),
            max_iteration: self.max_iteration,
        }
    }

    /// Convert a pixel offset into a math offset based on the local pixel sizes
    pub fn pixel_to_math_offset(
        &self,
        offset: Vector2D<i32, StageSpace>,
    ) -> Vector2D<f64, MathSpace> {
        self.stage_properties.pixel_to_math_offset(offset)
    }

    /// Shift the stage properties by the given amount of pixels while retaining size
    /// and aspect ratio of everything.
    pub fn shifted_clone_by_math(&self, offset: Vector2D<f64, MathSpace>) -> Self {
        ImageCompProperties {
            stage_properties: self.stage_properties.shifted_clone_by_math(offset),
            max_iteration: self.max_iteration,
        }
    }
}

/// Current state of a stage, set from the outside
///
/// State graph is: Initialized → Evolving ←→ Stalled
///                                  ↓
///                               Finished
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StageState {
    /// The stage has been initialized but no data has been written.
    Initialized,
    /// The stage is currently evolving, i.e. computation takes place and the content changes continously
    Evolving,
    /// The stage is not finished, but work is stalled, so no changes to the content are to be expected
    Stalled,
    /// The stage's content is complete, it matches the stage's properties, no more changes will happen
    Completed,
}

// end of file
