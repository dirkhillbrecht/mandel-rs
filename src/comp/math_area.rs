use bigdecimal::{BigDecimal, FromPrimitive, One, ToPrimitive};
use euclid::{Point2D, Rect, Size2D, Vector2D};

use crate::storage::coord_spaces::{MathSpace, StageSpace};

/// Area of computation, giving as center of the image, radius to conpute and ratio as width/height
#[derive(Debug, Clone)]
pub struct MathArea {
    center: Point2D<BigDecimal, MathSpace>,
    radius: BigDecimal,
    ratio: BigDecimal,
}

impl MathArea {
    /// Create a new a math area with a center point, the core radius, and the edge ratio
    pub fn new(
        center: Point2D<BigDecimal, MathSpace>,
        radius: BigDecimal,
        ratio: BigDecimal,
    ) -> Self {
        MathArea {
            center,
            radius,
            ratio,
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

    pub fn origin(&self) -> Point2D<BigDecimal, MathSpace> {
        Point2D::new(
            &self.center.x - self.bradwidth(),
            &self.center.y - self.bradheight(),
        )
    }

    pub fn rect(&self) -> Rect<BigDecimal, MathSpace> {
        let bradwidth = self.bradwidth();
        let bradheight = self.bradheight();
        Rect::new(
            Point2D::new(&self.center.x - &bradwidth, &self.center.y - &bradheight),
            Size2D::new(2 * bradwidth, 2 * bradheight),
        )
    }

    pub fn rect_f64(&self) -> Option<Rect<f64, MathSpace>> {
        let r = self.rect();
        if let Some(x) = r.origin.x.to_f64()
            && let Some(y) = r.origin.y.to_f64()
            && let Some(width) = r.size.width.to_f64()
            && let Some(height) = r.size.height.to_f64()
        {
            Some(Rect::new(Point2D::new(x, y), Size2D::new(width, height)))
        } else {
            None
        }
    }

    pub fn from_rect(rect: Rect<BigDecimal, MathSpace>) -> Self {
        let halfwidth: BigDecimal = rect.size.width / 2;
        let halfheight = rect.size.height / 2;
        let ratio = &halfwidth / &halfheight;
        let radius = halfwidth.clone().min(halfheight.clone());
        MathArea::new(
            Point2D::new(rect.origin.x + halfwidth, rect.origin.y + halfheight),
            radius,
            ratio,
        )
    }

    pub fn from_rect_f64(rect: Rect<f64, MathSpace>) -> Option<Self> {
        if let Some(x) = BigDecimal::from_f64(rect.origin.x)
            && let Some(y) = BigDecimal::from_f64(rect.origin.y)
            && let Some(width) = BigDecimal::from_f64(rect.size.width)
            && let Some(height) = BigDecimal::from_f64(rect.size.height)
        {
            Some(Self::from_rect(Rect::new(
                Point2D::new(x, y),
                Size2D::new(width, height),
            )))
        } else {
            None
        }
    }

    pub fn shift(&self, shift: Vector2D<BigDecimal, MathSpace>) -> Self {
        Self::new(
            Point2D::new(&self.center.x + shift.x, &self.center.y + shift.y),
            self.radius.clone(),
            self.ratio.clone(),
        )
    }

    pub fn center(&self) -> &Point2D<BigDecimal, MathSpace> {
        &self.center
    }
    pub fn radius(&self) -> &BigDecimal {
        &self.radius
    }
    pub fn ratio(&self) -> &BigDecimal {
        &self.ratio
    }
}

/// MathArea with a raster overlay allowing to obtain coordinates of points in the raster
///
/// Idea is to have a number of dots and to be able to get the
/// left, hcenter, or right and top, vcenter, or bottom coordinate of each dot.
/// Indexes are _not_ constrained to be within the raster, they can be larger or negative
/// pointing at a dot outside the area expressed by the math area below.
pub struct RasteredMathArea {
    math_area: MathArea,
    size: Size2D<u32, StageSpace>,
    base: Point2D<BigDecimal, MathSpace>,
    pix_size: Size2D<BigDecimal, MathSpace>,
}

impl RasteredMathArea {
    pub fn new(math_area: MathArea, size: Size2D<u32, StageSpace>) -> Self {
        let rect = math_area.rect();
        Self {
            math_area,
            size,
            base: rect.origin,
            pix_size: Size2D::new(rect.size.width / size.width, rect.size.height / size.height),
        }
    }
    pub fn math_area(&self) -> &MathArea {
        &self.math_area
    }
    pub fn size(&self) -> &Size2D<u32, StageSpace> {
        &self.size
    }
    pub fn coo_x(&self, x: i32) -> BigDecimal {
        &self.base.x + (x * &self.pix_size.width)
    }
    pub fn coo_y(&self, y: i32) -> BigDecimal {
        &self.base.y + (y * &self.pix_size.height)
    }
    pub fn coo(&self, coo: Point2D<i32, StageSpace>) -> Point2D<BigDecimal, MathSpace> {
        Point2D::new(self.coo_x(coo.x), self.coo_y(coo.y))
    }
    pub fn shift_by_pixel(&self, shift: Vector2D<BigDecimal, StageSpace>) -> Self {
        let math_shift = Vector2D::new(
            shift.x * &self.pix_size.width,
            shift.y * &self.pix_size.height,
        );
        Self::new(self.math_area.shift(math_shift), self.size.clone())
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
    fn area_rect_f64() {
        let x = BigDecimal::from_str("5.2").unwrap();
        let y = BigDecimal::from_str("3.9").unwrap();
        let radius = BigDecimal::from_str("0.7").unwrap();
        let ratio = BigDecimal::from_str("1.0").unwrap();
        let area = MathArea::new(Point2D::new(x.clone(), y.clone()), radius.clone(), ratio);
        let rect = area.rect_f64().unwrap();
        debug_assert_eq!(rect.origin.x, (x - radius.clone()).to_f64().unwrap());
        debug_assert_eq!(rect.origin.y, (y - radius.clone()).to_f64().unwrap());
        debug_assert_eq!(rect.size.width, 2.0 * radius.to_f64().unwrap());
        debug_assert_eq!(rect.size.height, 2.0 * radius.to_f64().unwrap());
    }

    #[test]
    fn area_from_rect() {
        {
            let rect = Rect::new(
                Point2D::new(
                    BigDecimal::from_str("1").unwrap(),
                    BigDecimal::from_str("1").unwrap(),
                ),
                Size2D::new(
                    BigDecimal::from_str("4").unwrap(),
                    BigDecimal::from_str("4").unwrap(),
                ),
            );
            let area = MathArea::from_rect(rect);
            assert_eq!(area.center.x, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.center.y, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.radius, BigDecimal::from_str("2").unwrap());
            assert_eq!(area.ratio, BigDecimal::from_str("1").unwrap());
        }
        {
            let rect = Rect::new(
                Point2D::new(
                    BigDecimal::from_str("1").unwrap(),
                    BigDecimal::from_str("1").unwrap(),
                ),
                Size2D::new(
                    BigDecimal::from_str("6").unwrap(),
                    BigDecimal::from_str("4").unwrap(),
                ),
            );
            let area = MathArea::from_rect(rect);
            assert_eq!(area.center.x, BigDecimal::from_str("4").unwrap());
            assert_eq!(area.center.y, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.radius, BigDecimal::from_str("2").unwrap());
            assert_eq!(
                area.ratio,
                BigDecimal::from_str("3").unwrap() / BigDecimal::from_str("2").unwrap()
            );
        }
        {
            let rect = Rect::new(
                Point2D::new(
                    BigDecimal::from_str("1").unwrap(),
                    BigDecimal::from_str("1").unwrap(),
                ),
                Size2D::new(
                    BigDecimal::from_str("4").unwrap(),
                    BigDecimal::from_str("6").unwrap(),
                ),
            );
            let area = MathArea::from_rect(rect);
            assert_eq!(area.center.x, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.center.y, BigDecimal::from_str("4").unwrap());
            assert_eq!(area.radius, BigDecimal::from_str("2").unwrap());
            assert_eq!(
                area.ratio,
                BigDecimal::from_str("2").unwrap() / BigDecimal::from_str("3").unwrap()
            );
        }
    }

    #[test]
    fn area_from_rect_f64() {
        {
            let rect = Rect::new(Point2D::new(1.0, 1.0), Size2D::new(4.0, 4.0));
            let area = MathArea::from_rect_f64(rect).unwrap();
            assert_eq!(area.center.x, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.center.y, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.radius, BigDecimal::from_str("2").unwrap());
            assert_eq!(area.ratio, BigDecimal::from_str("1").unwrap());
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
        assert_eq!(BigDecimal::from_str("4").unwrap(), raster_area.coo_y(0));
        assert_eq!(
            BigDecimal::from_str("6").unwrap(),
            raster_area.coo_y(height as i32)
        );
        assert_eq!(BigDecimal::from_str("1.04").unwrap(), raster_area.coo_x(1));
        assert_eq!(BigDecimal::from_str("0.96").unwrap(), raster_area.coo_x(-1));
        assert_eq!(BigDecimal::from_str("4.01").unwrap(), raster_area.coo_y(1));
        assert_eq!(BigDecimal::from_str("3.99").unwrap(), raster_area.coo_y(-1));
        assert_eq!(
            Point2D::new(
                BigDecimal::from_str("1.04").unwrap(),
                BigDecimal::from_str("3.99").unwrap()
            ),
            raster_area.coo(Point2D::new(1, -1))
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
            let shifted_raster_area = raster_area.shift_by_pixel(Vector2D::new(
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
            let shifted_raster_area = raster_area.shift_by_pixel(Vector2D::new(
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
}

// end of file
