use rust_decimal::Decimal;

pub struct Calculator {
    mulend1: Decimal,
    mulend2: Decimal,
}

impl Calculator {
    pub fn new(mulend1: Decimal, mulend2: Decimal) -> Self {
        Self {
            mulend1,
            mulend2,
        }
    }

    pub fn calc(&self) -> Decimal {
        self.mulend1 * self.mulend2
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::FromPrimitive;
    use super::*;

    #[test]
    fn multiply_two_integer() {
        let calculator = Calculator::new(Decimal::new(5, 0), Decimal::new(10, 0));
        assert_eq!(calculator.calc(), Decimal::new(50, 0));
    }

    #[test]
    fn multiply_two_float() {
        let calculator = Calculator::new(Decimal::from_f64(2.5).unwrap(), Decimal::from_f64(2.5).unwrap());
        assert_eq!(calculator.calc(), Decimal::from_f64(6.25).unwrap());
    }
}