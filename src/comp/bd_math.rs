use bigdecimal::BigDecimal;

/// Module to contain functions performing mathematical operations on BigDecimals

/// Return the magnitude of the given BigDecimal which is the exponent of the scientific notation
///
/// This should be a method of BigDecimal itself.
/// And it should be implemented in a more genuine way.
pub fn magnitude(n: &BigDecimal) -> i64 {
    n.to_scientific_notation()
        .rsplit_once('e')
        .map(|(_, mag)| mag.parse().unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_magnitude() {
        assert_eq!(4, magnitude(&BigDecimal::from_str("12345.6789").unwrap()));
        assert_eq!(2, magnitude(&BigDecimal::from_str("100").unwrap()));
        assert_eq!(0, magnitude(&BigDecimal::from_str("1").unwrap()));
        assert_eq!(-2, magnitude(&BigDecimal::from_str("0.01").unwrap()));
        assert_eq!(-3, magnitude(&BigDecimal::from_str("0.00785637").unwrap()));
    }
}
