use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive, One, ToPrimitive};
use euclid::{Point2D, Rect, Size2D, Vector2D};

use crate::{
    comp::bd_math,
    storage::coord_spaces::{MathSpace, StageSpace},
};

const RELEVANT_PRECISION: u64 = 8;
const RATIO_PRECISION: u64 = 20;

/// Area of computation, giving as center of the image, radius to conpute and ratio as width/height
#[derive(Debug, Clone)]
pub struct MathArea {
    center: Point2D<BigDecimal, MathSpace>,
    radius: BigDecimal,
    ratio: BigDecimal,
    radius_magnitude: i64,
    precision: u64,
}

impl MathArea {
    /// Create a new a math area with a center point, the core radius, and the edge ratio
    ///
    /// Note that the parameters are moved into the method and cannot be reused afterwards.
    /// This is due to the fact that BigDecimal does not implement the Copy trait.
    /// You may need to pass explicit clones of some values to this constructor.
    pub fn new(
        center: Point2D<BigDecimal, MathSpace>,
        radius: BigDecimal,
        ratio: BigDecimal,
    ) -> Self {
        let o_radius = radius.with_prec(RELEVANT_PRECISION).normalized();
        let radius_magnitude = bd_math::magnitude(&o_radius);
        let precision =
            (-(radius_magnitude - RELEVANT_PRECISION as i64)).max(RELEVANT_PRECISION as i64) as u64;
        let x_magnitude = bd_math::magnitude(&center.x);
        let y_magnitude = bd_math::magnitude(&center.y);
        let o_center = Point2D::new(
            center
                .x
                .with_prec((x_magnitude + precision as i64).max(RELEVANT_PRECISION as i64) as u64)
                .normalized(),
            center
                .y
                .with_prec((y_magnitude + precision as i64).max(RELEVANT_PRECISION as i64) as u64)
                .normalized(),
        );
        let o_ratio = ratio.with_prec(RATIO_PRECISION).normalized();
        MathArea {
            center: o_center,
            radius: o_radius,
            ratio: o_ratio,
            radius_magnitude,
            precision,
        }
    }

    /// Return a new instance of the math area using
    pub fn from_big_decimals(
        center_x: BigDecimal,
        center_y: BigDecimal,
        radius: BigDecimal,
        ratio: BigDecimal,
    ) -> Self {
        Self::new(Point2D::new(center_x, center_y), radius, ratio)
    }

    /// Return a new instance of math area using string representations of BigDecimal values
    ///
    /// If any of the strings cannot be converted into a big decimal, return None
    pub fn from_str(center_x: &str, center_y: &str, radius: &str, ratio: &str) -> Option<Self> {
        if let Ok(center_x) = BigDecimal::from_str(center_x)
            && let Ok(center_y) = BigDecimal::from_str(center_y)
            && let Ok(radius) = BigDecimal::from_str(radius)
            && let Ok(ratio) = BigDecimal::from_str(ratio)
        {
            Some(Self::from_big_decimals(center_x, center_y, radius, ratio))
        } else {
            None
        }
    }

    fn bradwidth(&self) -> BigDecimal {
        if self.ratio <= BigDecimal::one() {
            self.radius.clone()
        } else {
            &self.radius * &self.ratio
        }
    }

    fn bradheight(&self) -> BigDecimal {
        if self.ratio >= BigDecimal::one() {
            self.radius.clone()
        } else {
            &self.radius / &self.ratio
        }
    }

    /// Return the mathematical coordinates at the origin, i.e. the lower left corner of the area.
    #[allow(dead_code)]
    pub fn origin(&self) -> Point2D<BigDecimal, MathSpace> {
        Point2D::new(
            &self.center.x - self.bradwidth(),
            &self.center.y - self.bradheight(),
        )
    }

    /// Return the rectangle this area describes.
    pub fn rect(&self) -> Rect<BigDecimal, MathSpace> {
        let bradwidth = self.bradwidth();
        let bradheight = self.bradheight();
        let bw2: BigDecimal = 2 * &bradwidth;
        let bh2: BigDecimal = 2 * &bradheight;
        Rect::new(
            Point2D::new(
                (&self.center.x - &bradwidth)
                    .with_prec(self.precision)
                    .normalized(),
                (&self.center.y - &bradheight)
                    .with_prec(self.precision)
                    .normalized(),
            ),
            Size2D::new(
                bw2.with_prec(self.precision).normalized(),
                bh2.with_prec(self.precision).normalized(),
            ),
        )
    }

    pub fn shift(&self, shift: Vector2D<BigDecimal, MathSpace>) -> Self {
        Self::new(
            Point2D::new(&self.center.x + shift.x, &self.center.y + shift.y),
            self.radius.clone(),
            self.ratio.clone(),
        )
    }

    /// Return the center coordinates of the math area
    #[allow(dead_code)]
    pub fn center(&self) -> &Point2D<BigDecimal, MathSpace> {
        &self.center
    }
    /// Return the radius of the math area
    #[allow(dead_code)]
    pub fn radius(&self) -> &BigDecimal {
        &self.radius
    }
    /// Return the ratio of width by height of the math area
    #[allow(dead_code)]
    pub fn ratio(&self) -> &BigDecimal {
        &self.ratio
    }
    /// Return the magnitude of the radius
    ///
    /// This is important to select the correct number representation for computing images on this area.
    #[allow(dead_code)]
    pub fn radius_magnitude(&self) -> i64 {
        self.radius_magnitude
    }
    /// Return the relevant precision of the coordinate values of the math area
    ///
    /// The relevant precision is derived from the negative of the radius' magnitude and a constant relevance difference higher.
    /// The relevance difference is currently 8.
    /// If the magnitude of the radius is -4, i.e. 0.000625 then the needed precision of the coordinates is (-(-4))+8=12.
    /// The precision is never smaller than the relevance difference
    #[allow(dead_code)]
    pub fn precision(&self) -> u64 {
        self.precision
    }
}

/// MathArea with a raster overlay allowing to obtain coordinates of points in the raster
///
/// Idea is to have a number of dots and to be able to get the
/// left, hcenter, or right and top, vcenter, or bottom coordinate of each dot.
/// Indexes are _not_ constrained to be within the raster, they can be larger or negative
/// pointing at a dot outside the area expressed by the math area below.
///
/// Note: The raster has the origin at the same position as the math area!
/// For pixels on screens, the origin is on the _top_ left screen corner.
/// There are methods for offset and coordinate computation which perform this correction
/// internally. Be careful to use the right methods!
#[derive(Debug, Clone)]
pub struct RasteredMathArea {
    math_area: MathArea,
    size: Size2D<u32, StageSpace>,
    base: Point2D<BigDecimal, MathSpace>,
    pix_size: Size2D<BigDecimal, MathSpace>,
}

impl RasteredMathArea {
    /// Create a new rastered math area from a (non-rastered) math area and a size in pixels
    pub fn new(math_area: MathArea, size: Size2D<u32, StageSpace>) -> Self {
        let rect = math_area.rect();
        Self {
            math_area,
            size,
            base: rect.origin,
            pix_size: Size2D::new(rect.size.width / size.width, rect.size.height / size.height),
        }
    }
    /// Return a reference to the internally stored math area
    pub fn math_area(&self) -> &MathArea {
        &self.math_area
    }
    /// Return a reference to the internally stored pixel grid size
    pub fn size(&self) -> &Size2D<u32, StageSpace> {
        &self.size
    }
    /// Return the horizontal offset a raster point has from the origin in the math space
    pub fn offset_x(&self, x: i32) -> BigDecimal {
        x * &self.pix_size.width
    }
    /// Return the vertical offset a raster point has from the origin in the math space
    pub fn offset_y(&self, y: i32) -> BigDecimal {
        y * &self.pix_size.height
    }
    /// Return the offset a raster point has from the origin in the math space
    #[allow(dead_code)]
    pub fn offset(&self, coo: Point2D<i32, StageSpace>) -> Vector2D<BigDecimal, MathSpace> {
        Vector2D::new(self.offset_x(coo.x), self.offset_y(coo.y))
    }
    /// Return the horizontal offset a pixel has from the origin in the math space
    /// Pixels have another origin (top left) than the raster points (bottom left).
    /// Note that this does not make a difference for the horizontal axis.
    /// This method only exists for a clear API
    pub fn offset_pix_x(&self, x: i32) -> BigDecimal {
        self.offset_x(x)
    }
    /// Return the vertical offset a pixel has from the origin in the math space
    /// Pixels have another origin (top left) than the raster points (bottom left).
    pub fn offset_pix_y(&self, pix_y: i32) -> BigDecimal {
        self.offset_y(self.size.height as i32 - pix_y)
    }
    /// Return the offset a pixel has from the origin in the math space
    /// Pixels have another origin (top left) than the raster points (bottom left).
    #[allow(dead_code)]
    pub fn offset_pix(&self, coo: Point2D<i32, StageSpace>) -> Vector2D<BigDecimal, MathSpace> {
        Vector2D::new(self.offset_pix_x(coo.x), self.offset_pix_y(coo.y))
    }
    /// Return the mathematical x value of the given raster coordinate value
    pub fn coo_x(&self, x: i32) -> BigDecimal {
        &self.base.x + self.offset_x(x)
    }
    /// Return the mathematical y value of the given raster coordinate value
    pub fn coo_y(&self, y: i32) -> BigDecimal {
        &self.base.y + self.offset_y(y)
    }
    /// Return the mathematical value of the given raster coordinate value
    #[allow(dead_code)]
    pub fn coo(&self, coo: Point2D<i32, StageSpace>) -> Point2D<BigDecimal, MathSpace> {
        Point2D::new(self.coo_x(coo.x), self.coo_y(coo.y))
    }
    /// Return the mathematical x value of the given pixel coordinate value
    /// Pixels have another origin (top left) than the raster points (bottom left).
    pub fn coo_pix_x(&self, x: i32) -> BigDecimal {
        &self.base.x + self.offset_pix_x(x)
    }
    /// Return the mathematical y value of the given pixel coordinate value
    /// Pixels have another origin (top left) than the raster points (bottom left).
    pub fn coo_pix_y(&self, y: i32) -> BigDecimal {
        &self.base.y + self.offset_pix_y(y)
    }
    /// Return the mathematical value of the given pixel coordinate value
    /// Pixels have another origin (top left) than the raster points (bottom left).
    pub fn coo_pix(&self, coo: Point2D<i32, StageSpace>) -> Point2D<BigDecimal, MathSpace> {
        Point2D::new(self.coo_pix_x(coo.x), self.coo_pix_y(coo.y))
    }
    /// Return whether the given coordinate is a valid raster or pixel coordinate
    pub fn is_valid_pix(&self, p: &Point2D<i32, StageSpace>) -> bool {
        p.x >= 0 && p.x < self.size.width as i32 && p.y >= 0 && p.y < self.size.height as i32
    }
    /// Return a reference onto the size of a pixel in mathematical coordinates
    #[allow(dead_code)]
    pub fn pix_size(&self) -> &Size2D<BigDecimal, MathSpace> {
        &self.pix_size
    }

    /// Return the pixel the given math coordinate is located in
    pub fn math_to_pix(&self, math: Point2D<BigDecimal, MathSpace>) -> Point2D<i32, StageSpace> {
        let origin = self.coo_pix(Point2D::new(0, 0));
        let x = ((math.x - origin.x) / &self.pix_size.width)
            .to_f64()
            .unwrap()
            .floor() as i32;
        let y = ((origin.y - math.y) / &self.pix_size.height)
            .to_f64()
            .unwrap()
            .floor() as i32;
        Point2D::new(x, y)
    }

    /// Shift the whole math area by a certain amount of raster points
    pub fn shift_by_raster_points(&self, shift: Vector2D<BigDecimal, StageSpace>) -> Self {
        let math_shift = Vector2D::new(
            shift.x * &self.pix_size.width,
            shift.y * &self.pix_size.height,
        );
        Self::new(self.math_area.shift(math_shift), self.size.clone())
    }
    /// Shifts the whole area by a half raster point so that the actual coordinate is in the middle of the raster point
    pub fn shift_to_raster_point_center(&self) -> Self {
        self.shift_by_raster_points(Vector2D::new(
            BigDecimal::from_str("0.5").unwrap(),
            BigDecimal::from_str("0.5").unwrap(),
        ))
    }
    pub fn pixel_to_math_shift(
        &self,
        shift: Vector2D<BigDecimal, StageSpace>,
    ) -> Vector2D<BigDecimal, MathSpace> {
        Vector2D::new(
            -shift.x * &self.pix_size.width,
            shift.y * &self.pix_size.height,
        )
    }
    /// Shift the whole math area by a certain amount of pixels
    pub fn shift_by_pixels(&self, shift: Vector2D<BigDecimal, StageSpace>) -> Self {
        Self::new(
            self.math_area.shift(self.pixel_to_math_shift(shift)),
            self.size.clone(),
        )
    }
    /// Shifts the whole area by a half pixel so that the actual coordinate is in the middle of the pixel
    #[allow(dead_code)]
    pub fn shift_to_pixel_center(&self) -> Self {
        self.shift_by_pixels(Vector2D::new(
            BigDecimal::from_str("0.5").unwrap(),
            BigDecimal::from_str("0.5").unwrap(),
        ))
    }

    /// Shift this rastered area by some vector in the mathematical coordinate space
    /// Raster is unchanged by this operation
    pub fn shift_by_math(&self, shift: Vector2D<BigDecimal, MathSpace>) -> Self {
        Self::new(self.math_area.shift(shift), self.size)
    }

    /// Return a zoomed version with a certain factor at a certain pixel
    ///
    /// The method works in a way that the origin pixel relatively stays at the same position in the raster.
    /// Actually, the parameters of the underlying math area have to be recalculated.
    /// As the area coordinates are center/radius-based, this means some clever distance application.
    ///
    /// Note that some schemes from the Euclid library, namely to_vector() and friends, do not work here
    /// as BigDecimal does not implement the Copy trait. Too bad…
    pub fn zoom_at_pixel(&self, origin: Point2D<i32, StageSpace>, factor: BigDecimal) -> Self {
        let new_origin = self.coo_pix(origin);
        let old_center = &self.math_area.center;
        let orig_to_old_center: Vector2D<BigDecimal, MathSpace> =
            Vector2D::new(&new_origin.x - &old_center.x, &new_origin.y - &old_center.y);
        let orig_to_new_center = Vector2D::new(
            orig_to_old_center.x / &factor,
            orig_to_old_center.y / &factor,
        );
        let new_center = new_origin - orig_to_new_center;
        let new_radius = self.math_area().radius() / &factor;
        let new_math_area = MathArea::new(new_center, new_radius, self.math_area.ratio.clone());
        Self::new(new_math_area, self.size().clone())
    }

    /// Return a rectified version of this math area, i.e. a version where pixels are squares
    ///
    /// The rectified version has the same pixel resolution.
    /// The mathematical area might be changed.
    pub fn rectified(&self) -> Self {
        let raster_ratio = self.size.width as f64 / self.size.height as f64;
        let math_ratio = self.math_area.ratio.to_f64().unwrap();
        if (1.0 - (raster_ratio / math_ratio)).abs() < 1e-5 {
            self.clone()
        } else {
            Self::new(
                MathArea::new(
                    self.math_area.center.clone(),
                    self.math_area.radius.clone(),
                    BigDecimal::from_f64(raster_ratio).unwrap(),
                ),
                self.size.clone(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn new_area() {
        let x = BigDecimal::from_str("5.2").unwrap();
        let y = BigDecimal::from_str("3.9").unwrap();
        let radius = BigDecimal::from_str("0.7").unwrap();
        let ratio = BigDecimal::from_str("1.0").unwrap();
        let area = MathArea::new(
            Point2D::new(x.clone(), y.clone()),
            radius.clone(),
            ratio.clone(),
        );
        assert_eq!(x, area.center.x);
        assert_eq!(y, area.center.y);
        assert_eq!(&radius, area.radius());
        assert_eq!(ratio, area.ratio)
    }

    #[test]
    fn area_precision() {
        let x = BigDecimal::from_str("0.12345678901234567890").unwrap();
        let y = BigDecimal::from_str("-0.012345678901234567890").unwrap();
        let ratio = BigDecimal::from_str("1.0").unwrap();
        {
            let radius = BigDecimal::from_str("0.6").unwrap();
            let area1 = MathArea::new(
                Point2D::new(x.clone(), y.clone()),
                radius.clone(),
                ratio.clone(),
            );
            assert_eq!(
                BigDecimal::from_str("0.12345679").unwrap(),
                area1.center().x
            );
            assert_eq!(
                BigDecimal::from_str("-0.012345678").unwrap(),
                area1.center().y
            );
        }
        {
            let radius = BigDecimal::from_str("0.000006").unwrap();
            let area1 = MathArea::new(
                Point2D::new(x.clone(), y.clone()),
                radius.clone(),
                ratio.clone(),
            );
            assert_eq!(
                BigDecimal::from_str("0.1234567890123").unwrap(),
                area1.center().x
            );
            assert_eq!(
                BigDecimal::from_str("-0.0123456789012").unwrap(),
                area1.center().y
            );
        }
    }

    #[test]
    fn area_rect() {
        {
            let x = BigDecimal::from_str("5.2").unwrap();
            let y = BigDecimal::from_str("3.9").unwrap();
            let radius = BigDecimal::from_str("0.7").unwrap();
            let ratio = BigDecimal::from_str("1.0").unwrap();
            let area = MathArea::new(Point2D::new(x.clone(), y.clone()), radius.clone(), ratio);
            let rect = area.rect();
            debug_assert_eq!(rect.origin.x, x - radius.clone());
            debug_assert_eq!(rect.origin.y, y - radius.clone());
            debug_assert_eq!(rect.size.width, 2 * radius.clone());
            debug_assert_eq!(rect.size.height, 2 * radius.clone());
        }
        {
            let x = BigDecimal::from_str("6.0").unwrap();
            let y = BigDecimal::from_str("8.0").unwrap();
            let radius = BigDecimal::from_str("2.0").unwrap();
            let ratio = BigDecimal::from_str("3.0").unwrap() / BigDecimal::from_str("2.0").unwrap();
            let area = MathArea::new(
                Point2D::new(x.clone(), y.clone()),
                radius.clone(),
                ratio.clone(),
            );
            let rect = area.rect();
            assert_eq!(rect.origin.x, x - &radius * &ratio);
            assert_eq!(rect.origin.y, y - &radius);
            assert_eq!(rect.size.width, 2 * &radius * &ratio);
            assert_eq!(rect.size.height, 2 * &radius);
        }
        {
            let x = BigDecimal::from_str("6.0").unwrap();
            let y = BigDecimal::from_str("8.0").unwrap();
            let radius = BigDecimal::from_str("2.0").unwrap();
            let ratio = BigDecimal::from_str("2.0").unwrap() / BigDecimal::from_str("3.0").unwrap();
            let area = MathArea::new(
                Point2D::new(x.clone(), y.clone()),
                radius.clone(),
                ratio.clone(),
            );
            let rect = area.rect();
            assert_eq!(rect.origin.x, x - &radius);
            assert_eq!(rect.origin.y, y - &radius / &ratio);
            assert_eq!(rect.size.width, 2 * &radius);
            assert_eq!(rect.size.height, 2 * (&radius / &ratio));
        }
    }

    #[test]
    fn area_shift() {
        let x = BigDecimal::from_str("5").unwrap();
        let y = BigDecimal::from_str("1").unwrap();
        let radius = BigDecimal::from_str("8").unwrap();
        let ratio = BigDecimal::from_str("9").unwrap();
        let area = MathArea::new(
            Point2D::new(x.clone(), y.clone()),
            radius.clone(),
            ratio.clone(),
        );
        let new_area = area.shift(Vector2D::new(
            BigDecimal::from_str("3").unwrap(),
            BigDecimal::from_str("4").unwrap(),
        ));
        assert_eq!(BigDecimal::from_str("8").unwrap(), new_area.center.x);
        assert_eq!(BigDecimal::from_str("5").unwrap(), new_area.center.y);
        assert_eq!(&radius, new_area.radius());
        assert_eq!(ratio, new_area.ratio);
    }

    #[test]
    fn raster_area_new() {
        let x = BigDecimal::from_str("5").unwrap();
        let y = BigDecimal::from_str("1").unwrap();
        let radius = BigDecimal::from_str("8").unwrap();
        let ratio = BigDecimal::from_str("9").unwrap();
        let area = MathArea::new(
            Point2D::new(x.clone(), y.clone()),
            radius.clone(),
            ratio.clone(),
        );
        let width = 800;
        let height = 600;
        let size = Size2D::new(width, height);
        let raster_area = RasteredMathArea::new(area, size);
        assert_eq!(x, raster_area.math_area().center().x);
        assert_eq!(y, raster_area.math_area().center().y);
        assert_eq!(&radius, raster_area.math_area().radius());
        assert_eq!(&ratio, raster_area.math_area().ratio());
        assert_eq!(width, raster_area.size().width);
        assert_eq!(height, raster_area.size().height);
    }

    #[test]
    fn raster_area_coo() {
        let x = BigDecimal::from_str("3").unwrap();
        let y = BigDecimal::from_str("5").unwrap();
        let radius = BigDecimal::from_str("1").unwrap();
        let ratio = BigDecimal::from_str("2").unwrap();
        let area = MathArea::new(
            Point2D::new(x.clone(), y.clone()),
            radius.clone(),
            ratio.clone(),
        );
        let width = 100;
        let height = 200;
        let size = Size2D::new(width, height);
        let raster_area = RasteredMathArea::new(area, size);
        assert_eq!(BigDecimal::from_str("1").unwrap(), raster_area.coo_x(0));
        assert_eq!(
            BigDecimal::from_str("5").unwrap(),
            raster_area.coo_x(width as i32)
        );
        assert_eq!(BigDecimal::from_str("1").unwrap(), raster_area.coo_pix_x(0));
        assert_eq!(
            BigDecimal::from_str("5").unwrap(),
            raster_area.coo_pix_x(width as i32)
        );
        assert_eq!(BigDecimal::from_str("4").unwrap(), raster_area.coo_y(0));
        assert_eq!(
            BigDecimal::from_str("6").unwrap(),
            raster_area.coo_y(height as i32)
        );
        assert_eq!(BigDecimal::from_str("6").unwrap(), raster_area.coo_pix_y(0));
        assert_eq!(
            BigDecimal::from_str("4").unwrap(),
            raster_area.coo_pix_y(height as i32)
        );
        assert_eq!(BigDecimal::from_str("1.04").unwrap(), raster_area.coo_x(1));
        assert_eq!(BigDecimal::from_str("0.96").unwrap(), raster_area.coo_x(-1));
        assert_eq!(BigDecimal::from_str("4.01").unwrap(), raster_area.coo_y(1));
        assert_eq!(BigDecimal::from_str("3.99").unwrap(), raster_area.coo_y(-1));
        assert_eq!(
            BigDecimal::from_str("5.99").unwrap(),
            raster_area.coo_y(height as i32 - 1)
        );
        assert_eq!(
            BigDecimal::from_str("6.01").unwrap(),
            raster_area.coo_y(height as i32 + 1)
        );
        assert_eq!(
            Point2D::new(
                BigDecimal::from_str("1.04").unwrap(),
                BigDecimal::from_str("3.99").unwrap()
            ),
            raster_area.coo(Point2D::new(1, -1))
        );
        assert_eq!(
            Point2D::new(
                BigDecimal::from_str("1.04").unwrap(),
                BigDecimal::from_str("3.99").unwrap()
            ),
            raster_area.coo_pix(Point2D::new(1, height as i32 + 1))
        );
    }

    #[test]
    fn raster_area_shift() {
        let x = BigDecimal::from_str("3").unwrap();
        let y = BigDecimal::from_str("5").unwrap();
        let radius = BigDecimal::from_str("1").unwrap();
        let ratio = BigDecimal::from_str("2").unwrap();
        let area = MathArea::new(
            Point2D::new(x.clone(), y.clone()),
            radius.clone(),
            ratio.clone(),
        );
        let width = 100;
        let height = 200;
        let size = Size2D::new(width, height);
        let raster_area = RasteredMathArea::new(area, size);
        {
            let shifted_raster_area = raster_area.shift_by_raster_points(Vector2D::new(
                BigDecimal::from_str("1").unwrap(),
                BigDecimal::from_str("-1").unwrap(),
            ));
            assert_eq!(
                Point2D::new(
                    BigDecimal::from_str("1.04").unwrap(),
                    BigDecimal::from_str("3.99").unwrap()
                ),
                shifted_raster_area.coo(Point2D::new(0, 0))
            );
        }
        {
            let shifted_raster_area = raster_area.shift_by_raster_points(Vector2D::new(
                BigDecimal::from_str("-0.5").unwrap(),
                BigDecimal::from_str("0.5").unwrap(),
            ));
            assert_eq!(
                Point2D::new(
                    BigDecimal::from_str("0.98").unwrap(),
                    BigDecimal::from_str("4.005").unwrap()
                ),
                shifted_raster_area.coo(Point2D::new(0, 0))
            );
            assert_eq!(
                Point2D::new(
                    BigDecimal::from_str("1.06").unwrap(),
                    BigDecimal::from_str("4.025").unwrap()
                ),
                shifted_raster_area.coo(Point2D::new(2, 2))
            );
        }
    }

    #[test]
    fn raster_area_math_shift() {
        let x = BigDecimal::from_str("5").unwrap();
        let y = BigDecimal::from_str("1").unwrap();
        let radius = BigDecimal::from_str("8").unwrap();
        let ratio = BigDecimal::from_str("9").unwrap();
        let area = RasteredMathArea::new(
            MathArea::new(
                Point2D::new(x.clone(), y.clone()),
                radius.clone(),
                ratio.clone(),
            ),
            Size2D::new(800, 600),
        );
        let new_area = area.shift_by_math(Vector2D::new(
            BigDecimal::from_str("3").unwrap(),
            BigDecimal::from_str("4").unwrap(),
        ));
        assert_eq!(
            BigDecimal::from_str("8").unwrap(),
            new_area.math_area.center.x
        );
        assert_eq!(
            BigDecimal::from_str("5").unwrap(),
            new_area.math_area.center.y
        );
        assert_eq!(&radius, new_area.math_area.radius());
        assert_eq!(ratio, new_area.math_area().ratio);
        assert_eq!(800, new_area.size().width);
        assert_eq!(600, new_area.size().height);
    }

    #[test]
    fn raster_math_to_pix() {
        let x = BigDecimal::from_str("5").unwrap();
        let y = BigDecimal::from_str("8").unwrap();
        let radius = BigDecimal::from_str("2").unwrap();
        let ratio = BigDecimal::from_str("3").unwrap();
        let area = RasteredMathArea::new(
            MathArea::new(
                Point2D::new(x.clone(), y.clone()),
                radius.clone(),
                ratio.clone(),
            ),
            Size2D::new(900, 100),
        );
        assert_eq!(
            Point2D::new(0, 0), // top left corner
            area.math_to_pix(Point2D::new(
                BigDecimal::from_str("-1").unwrap(),
                BigDecimal::from_str("10").unwrap()
            ))
        );
        assert_eq!(
            Point2D::new(450, 50), // center
            area.math_to_pix(Point2D::new(
                BigDecimal::from_str("5").unwrap(),
                BigDecimal::from_str("8").unwrap()
            ))
        );
        assert_eq!(
            Point2D::new(900, 100), // center
            area.math_to_pix(Point2D::new(
                BigDecimal::from_str("11").unwrap(),
                BigDecimal::from_str("6").unwrap()
            ))
        );
    }
}

// end of file
