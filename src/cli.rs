use std::str::FromStr;
use clap::{error, Parser};
use rust_decimal::Decimal;
use crate::calculator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    mulend1: String,

    #[arg()]
    mulend2: String,
}

pub fn start() -> Result<(), error::Error> {
    let args = Args::parse();

    let mulend1_decimal = Decimal::from_str(&args.mulend1).map_err(|_| error::Error::new(error::ErrorKind::InvalidValue))?;
    let mulend2_decimal = Decimal::from_str(&args.mulend2).map_err(|_| error::Error::new(error::ErrorKind::InvalidValue))?;

    let calculator = calculator::Calculator::new(mulend1_decimal, mulend2_decimal);
    println!("{}", calculator.calc());

    Ok(())
}