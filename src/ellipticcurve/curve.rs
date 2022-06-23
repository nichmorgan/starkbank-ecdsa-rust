use super::point::Point;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::{collections::HashMap, fmt, str::FromStr};

struct Oid(Vec<i32>);

impl fmt::Display for Oid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let value = self
            .0
            .to_owned()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",");

        fmt.write_str(value.as_str());
        Ok(())
    }
}

pub struct CurveFp {
    a: BigInt,
    b: BigInt,
    p: BigInt,
    n: BigInt,
    g: Point,
    name: String,
    oid: Oid,
    nist_name: String,
}

impl CurveFp {
    fn new(
        a_hex: &str,
        b_hex: &str,
        p_hex: &str,
        n_hex: &str,
        gx_hex: &str,
        gy_hex: &str,
        name: &str,
        oid: Oid,
        nist_name: &str,
    ) -> CurveFp {
        let f0: BigInt = Zero::zero();

        let a = BigInt::from_str(&a_hex).unwrap();
        let b = BigInt::from_str(&b_hex).unwrap();
        let p = BigInt::from_str(&p_hex).unwrap();
        let n = BigInt::from_str(&n_hex).unwrap();
        let g = Point::new(gx_hex.to_string(), gy_hex.to_string(), f0.to_string());

        let name = name.to_string();
        let nist_name = nist_name.to_string();

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

struct Curves {
    curves: HashMap<String, CurveFp>,
}

impl Curves {
    pub fn new() -> Curves {
        let mut curve_map: HashMap<String, CurveFp> = HashMap::new();
        Curves::supported_curves().into_iter().for_each(|c| {
            curve_map.insert(c.oid.to_string(), c);
        });
        Curves { curves: curve_map }
    }

    pub fn get_curve_by_oid(&self, oid: &str) -> &CurveFp {
        self.curves
            .get(oid)
            .expect(&format!("Unknown curve with oid {:?}", &oid))
    }

    fn supported_curves() -> Vec<CurveFp> {
        vec![Curves::prime256v1(), Curves::prime256v1()]
    }

    fn prime256v1() -> CurveFp {
        CurveFp::new(
            "0xffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
            "0x5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
            "0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
            "0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551",
            "0x6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
            "0x4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
            "prime256v1",
            Oid(vec![1, 2, 840, 10045, 3, 1, 7]),
            "P-256",
        )
    }

    fn p256() -> CurveFp {
        Curves::prime256v1()
    }

    fn secp256k1() -> CurveFp {
        CurveFp::new(
            "0x0000000000000000000000000000000000000000000000000000000000000000",
            "0x0000000000000000000000000000000000000000000000000000000000000007",
            "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            "secp256k1",
            Oid(vec![1, 3, 132, 0, 10]),
            "",
        )
    }
}
