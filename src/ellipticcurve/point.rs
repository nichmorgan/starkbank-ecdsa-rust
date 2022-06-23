extern crate num_bigint;

use num_bigint::{BigInt, ToBigInt};
use std::{fmt, str::FromStr};

pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
    pub z: BigInt,
}

impl Point {
    pub fn new(x_hex: String, y_hex: String, z_hex: String) -> Point {
        let x = BigInt::from_str(&x_hex).unwrap();
        let y = BigInt::from_str(&y_hex).unwrap();
        let z = BigInt::from_str(&z_hex).unwrap();
        Point { x, y, z }
    }

    pub fn is_at_infinity(&self) -> bool {
        self.y == 0_i32.to_bigint().unwrap()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}
