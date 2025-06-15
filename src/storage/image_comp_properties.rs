// Collective storage for the computational image parameters.

/// Simple data structure storing the mathematical coordinates of a rectangle
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    /// left edge value
    pub x_min: f64,
    /// right edge value
    pub x_max: f64,
    /// bottom edge value
    pub y_min: f64,
    /// top edge value
    pub y_max: f64,
}

impl Rect {
    /// Create a new Rect instance, min and max values are sliently swapped if in wrong order
    pub fn new(x_min:f64, x_max:f64, y_min:f64, y_max:f64) -> Rect {
        Rect { x_min: x_min.min(x_max), x_max: x_max.max(x_min),
            y_min: y_min.min(y_max), y_max: y_max.max(y_min), }
    }
    /// return the distance between right and left edge, always positive
    pub fn x_dist(&self) -> f64 {
        self.x_max-self.x_min
    }
    /// return the distance between top and bottom edge, always positive
    pub fn y_dist(&self) -> f64 {
        self.y_max-self.y_min
    }
}

/// Properties of the computation stage, not only mathematical coordinates, but also pixel raster size
#[derive(Debug, Clone, Copy)]
pub struct StageProperties {
    /// The mathematical coordinates of the stage
    pub coo: Rect,
    /// Number of pixels in width
    pub width: u32,
    /// Number of pixels in height
    pub height: u32,

    x_dotsize: f64,
    y_dotsize: f64,
    x_coo_base: f64,
    y_coo_base: f64,
}

impl StageProperties {
    /// Create a new stage properties instance
    pub fn new(coo: Rect, width: u32, height: u32) -> StageProperties {
        let x_dotsize=coo.x_dist()/width as f64;
        let y_dotsize=coo.y_dist()/height as f64;
        let x_coo_base=coo.x_min+(x_dotsize/2.0);
        let y_coo_base=coo.y_max-(y_dotsize/2.0);
        StageProperties { coo, width, height,
            x_dotsize,
            y_dotsize,
            x_coo_base,
            y_coo_base,
        }
    }

    /// Return the mathematical x coordinate for the given pixel x coordinate
    pub fn x(&self,x_pix:u32) -> f64 { self.x_coo_base+x_pix as f64*self.x_dotsize }
    /// Return the mathematical y coordinate for the given pixel y coordinate
    pub fn y(&self,y_pix:u32) -> f64 { self.y_coo_base-y_pix as f64*self.y_dotsize }

    /// Return a rectified version of the stage, i.e. guarantee that the pixels of the image cover a square area.
    ///
    /// Width and height are unchanged, mathematical coordinates are shrunk or enlarged to keep the ratio.
    /// Here, the new coordinate rectangle is spun around the _center_ of the original one.
    ///
    /// # Arguments
    /// * `inner` - Flag whether the new coordinates should describe a rectangle _within_ or _outside_ the original one
    pub fn rectified(&self, inner: bool) -> StageProperties {
        let x_dotsize=self.coo.x_dist()/self.width as f64;
        let y_dotsize=self.coo.y_dist()/self.height as f64;
        if (1.0-(x_dotsize/y_dotsize)).abs()<1e-5 {
            self.clone()
        }
        else {
            let dotsize=if inner { x_dotsize.min(y_dotsize) } else { x_dotsize.max(y_dotsize) };
            let x_center=self.coo.x_min+(self.coo.x_dist()/2.0);
            let y_center=self.coo.y_min+(self.coo.y_dist()/2.0);
            let x_dist=dotsize*((self.width as f64)/2.0);
            let y_dist=dotsize*((self.height as f64)/2.0);
            StageProperties::new(Rect { x_min: x_center-x_dist, x_max: x_center+x_dist, y_min: y_center-y_dist, y_max: y_center+y_dist, },
                self.width,self.height)
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
        ImageCompProperties { stage_properties, max_iteration }
    }
    /// # Returns
    /// some new ImageCompProperties which are rectified for pixels with a square area
    pub fn rectified(&self, inner: bool) -> ImageCompProperties {
        ImageCompProperties { stage_properties: self.stage_properties.rectified(inner), max_iteration: self.max_iteration }
    }
}

// end of file
