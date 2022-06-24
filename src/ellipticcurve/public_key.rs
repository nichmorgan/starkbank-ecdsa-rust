use std::fmt;

use super::curve::{CurveFp, Oid};
use super::point::Point;

const ECDSA_PUBLIC_KEY_OID: [i32; 6] = [1, 2, 840, 10045, 2, 1];

const TO_PEM_TEMPLATE: &str = "
-----BEGIN PUBLIC KEY-----
{content}
-----END PUBLIC KEY-----
";
const FROM_PEM_TEMPLATE: &str = r"
^\s*-----BEGIN PUBLIC KEY-----
{content}
-----END PUBLIC KEY-----\s*$
";

struct PublicKey {
    curve: CurveFp,
    point: Point,
}

impl PublicKey {
    pub fn new(point: Point, curve: CurveFp) -> PublicKey {
        PublicKey { curve, point }
    }

    pub fn to_string(&self, encoded: bool) -> String {
        let base_len = 2 * self.curve.len();
        let x_hex = format!(
            "{:0fill$}",
            self.point.x.to_owned().to_string(),
            fill = base_len
        );
        let y_hex = format!(
            "{:0fill$}",
            self.point.y.to_owned().to_string(),
            fill = base_len
        );
        let hex = format!("{}{}", x_hex, y_hex);

        match encoded {
            true => format!("0004{}", hex),
            false => hex,
        }
    }

    // pub fn to_der(&self) -> Vec<u8> {}
}
