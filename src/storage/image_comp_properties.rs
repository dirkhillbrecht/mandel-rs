// Collective storage for the computational image parameters.

use euclid::{Point2D, Rect, Size2D};

use crate::storage::coord_spaces::{MathSpace, StageSpace};

/// Properties of the computation stage, not only mathematical coordinates, but also pixel raster size
#[derive(Debug, Clone, Copy)]
pub struct StageProperties {
    /// The mathematical coordinates of the stage
    pub coo: Rect<f64, MathSpace>,
    pub pixels: Size2D<u32, StageSpace>,

    pub dotsize: Size2D<f64, MathSpace>,
    pub coo_base: Point2D<f64, MathSpace>,
}

impl StageProperties {
    /// Create a new stage properties instance
    pub fn new(coo: Rect<f64, MathSpace>, pixels: Size2D<u32, StageSpace>) -> StageProperties {
        let dotsize = Size2D::new(
            coo.width() / pixels.width as f64,
            coo.height() / pixels.height as f64,
        );
        let coo_base = Point2D::new(
            coo.min_x() + (dotsize.width / 2.0),
            coo.max_y() - (dotsize.height / 2.0),
        );
        StageProperties {
            coo,
            pixels,
            dotsize,
            coo_base,
        }
    }

    /// Return the mathematical x coordinate for the given pixel x coordinate
    pub fn x(&self, x_pix: u32) -> f64 {
        self.coo_base.x + x_pix as f64 * self.dotsize.width
    }
    /// Return the mathematical y coordinate for the given pixel y coordinate
    pub fn y(&self, y_pix: u32) -> f64 {
        self.coo_base.y - y_pix as f64 * self.dotsize.height
    }

    /// Return the mathematical coordiates of a pixel
    #[allow(dead_code)]
    pub fn pix_to_math(&self, pix: Point2D<u32, StageSpace>) -> Point2D<f64, MathSpace> {
        Point2D::new(self.x(pix.x), self.y(pix.y))
    }

    /// Return the pixel coordinates of some mathematical coordinates, if they are in bounds
    #[allow(dead_code)]
    pub fn math_to_pix(&self, math: Point2D<f64, MathSpace>) -> Option<Point2D<u32, StageSpace>> {
        if math.x < self.coo_base.x || math.y > self.coo_base.y {
            None
        } else {
            let x = ((math.x - self.coo_base.x) / self.dotsize.width).floor() as u32;
            let y = ((self.coo_base.y - math.y) / self.dotsize.height).floor() as u32;
            if x >= self.pixels.width || y >= self.pixels.height {
                None
            } else {
                Some(Point2D::new(x, y))
            }
        }
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
    pub fn new(stage_properties: StageProperties, max_iteration: u32) -> ImageCompProperties {
        ImageCompProperties {
            stage_properties,
            max_iteration,
        }
    }
    /// # Returns
    /// some new ImageCompProperties which are rectified for pixels with a square area
    pub fn rectified(&self, inner: bool) -> ImageCompProperties {
        ImageCompProperties {
            stage_properties: self.stage_properties.rectified(inner),
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
    /// The stage 's content is complete, it matches the stage's properties, no more changes will happen
    Completed,
}

// end of file
