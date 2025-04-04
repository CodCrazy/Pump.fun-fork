use std::ops::{Div, Mul};

pub fn convert_to_float(value: u64, decimals: u8) -> f64 {
    (value as f64).div(f64::powf(10.0, decimals as f64))
}

pub fn convert_from_float(value: f64, decimals: u8) -> u64 {
    value.mul(f64::powf(10.0, decimals as f64)) as u64
}

pub fn calculate_price(price: u64, fee: u64) -> u64 {
    price.mul(fee).div(100)
}

pub fn calculate_price_with_fee(price: u64, fee: u64) -> u64 {
    price.mul(fee).div(100)
}


