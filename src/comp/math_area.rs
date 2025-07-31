use bigdecimal::{BigDecimal, FromPrimitive, One, ToPrimitive};
use euclid::{Point2D, Rect, Size2D};

use crate::storage::coord_spaces::MathSpace;

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

    pub fn rect(&self) -> Rect<BigDecimal, MathSpace> {
        let bradwidth = if self.ratio <= BigDecimal::one() {
            self.radius.clone()
        } else {
            &self.radius * &self.ratio
        };
        let bradheight = if self.ratio >= BigDecimal::one() {
            self.radius.clone()
        } else {
            &self.radius / &self.ratio
        };
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
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_new() {
        let x = BigDecimal::from_str("5.2").unwrap();
        let y = BigDecimal::from_str("3.9").unwrap();
        let radius = BigDecimal::from_str("0.7").unwrap();
        let ratio = BigDecimal::from_str("1.0").unwrap();
        let area = MathArea::new(Point2D::new(x.clone(), y.clone()), radius, ratio);
        assert_eq!(x, area.center.x);
    }

    #[test]
    fn test_rect() {
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

    #[test]
    fn test_rect_ratio_gt_1() {
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

    #[test]
    fn test_rect_ratio_lt_1() {
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

    #[test]
    fn test_rect_f64() {
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
    fn test_from_rect() {
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
    fn test_from_rect_f64() {
        {
            let rect = Rect::new(Point2D::new(1.0, 1.0), Size2D::new(4.0, 4.0));
            let area = MathArea::from_rect_f64(rect).unwrap();
            assert_eq!(area.center.x, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.center.y, BigDecimal::from_str("3").unwrap());
            assert_eq!(area.radius, BigDecimal::from_str("2").unwrap());
            assert_eq!(area.ratio, BigDecimal::from_str("1").unwrap());
        }
    }
}

// end of file
