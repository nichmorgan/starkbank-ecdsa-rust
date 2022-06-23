extern crate num_bigint;

use super::point::Point;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::{collections::HashMap, str::FromStr};

pub struct CurveFp {
    a: BigInt,
    b: BigInt,
    p: BigInt,
    n: BigInt,
    g: Point,
    name: String,
    oid: Vec<i64>,
    nist_name: Option<String>,
}

impl CurveFp {
    fn new(
        a_hex: String,
        b_hex: String,
        p_hex: String,
        n_hex: String,
        gx_hex: String,
        gy_hex: String,
        name: String,
        oid: Vec<i64>,
        nist_name: Option<String>,
    ) -> CurveFp {
        let f0: BigInt = Zero::zero();

        let a = BigInt::from_str(&a_hex).unwrap();
        let b = BigInt::from_str(&b_hex).unwrap();
        let p = BigInt::from_str(&p_hex).unwrap();
        let n = BigInt::from_str(&n_hex).unwrap();
        let g = Point::new(gx_hex, gy_hex, f0.to_string());

        CurveFp {
            a,
            b,
            p,
            n,
            g,
            name,
            oid,
            nist_name,
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        let f0: BigInt = Zero::zero();
        let f1: BigInt = One::one();

        if !(&f0 <= &p.x) && &p.x <= &(&self.p - &f1) {
            false
        } else if !(&f0 <= &p.y) && &p.y <= &(&self.p - &f1) {
            false
        } else if (&p.y ^ 2 - (&p.x ^ 3 + &self.a * &p.x + &self.b)) % &self.p != f0 {
            false
        } else {
            true
        }
    }

    pub fn len(&self) -> usize {
        let n_len = format!("{:#x}", self.n).len();
        (1 + n_len).rem_euclid(2)
    }
}

fn secp256k1() -> CurveFp {
    let a_hex = String::from("0x0000000000000000000000000000000000000000000000000000000000000000");
    let b_hex = String::from("0x0000000000000000000000000000000000000000000000000000000000000007");
    let p_hex = String::from("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");
    let n_hex = String::from("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
    let gx_hex = String::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    let gy_hex = String::from("0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");

    let name = String::from("secp256k1");
    let oid = vec![1, 3, 132, 0, 10];
    let nist_name = None;

    CurveFp::new(
        a_hex, b_hex, p_hex, n_hex, gx_hex, gy_hex, name, oid, nist_name,
    )
}

fn prime256v1() -> CurveFp {
    let a_hex = String::from("0xffffffff00000001000000000000000000000000fffffffffffffffffffffffc");
    let b_hex = String::from("0x5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b");
    let p_hex = String::from("0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff");
    let n_hex = String::from("0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
    let gx_hex = String::from("0x6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296");
    let gy_hex = String::from("0x4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5");

    let name = String::from("prime256v1");
    let oid = vec![1, 2, 840, 10045, 3, 1, 7];
    let nist_name = Some(String::from("P-256"));

    CurveFp::new(
        a_hex, b_hex, p_hex, n_hex, gx_hex, gy_hex, name, oid, nist_name,
    )
}

struct SupportedCurves {
    curves: HashMap<Vec<i64>, CurveFp>,
}

impl SupportedCurves {
    pub fn new() -> SupportedCurves {
        let mut curves: HashMap<Vec<i64>, CurveFp> = HashMap::new();
        for c in vec![secp256k1(), prime256v1()] {
            let oid = c.oid.to_owned();
            curves.insert(oid, c);
        }
        SupportedCurves { curves }
    }

    pub fn get_curve_by_oid(&self, oid: Vec<i64>) -> &CurveFp {
        self.curves
            .get(&oid)
            .expect(&format!("Unknown curve with oid {:?}", &oid))
    }
}
