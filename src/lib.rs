mod ellipticcurve;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use num_bigint::BigInt;

    #[test]
    fn it_works() {
        let v =
            BigInt::from_str("0x0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap();

        assert_eq!(v, BigInt::from(0));
    }
}
