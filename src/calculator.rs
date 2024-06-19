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