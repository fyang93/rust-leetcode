pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let mut res = 0;
    let mut dvd = (dividend as i64).abs();
    let dvs = (divisor as i64).abs();

    // 考虑二进制： dvd >= (2^? + 2^? + ... ) * dvs
    while dvd >= dvs {
        let order = (1..).find(|i| dvs << i > dvd).unwrap() - 1;
        dvd -= dvs << order;
        res += 1 << order;
    }

    if (dividend < 0) ^ (divisor < 0) {
        res = -res;
    }
    if res > i32::max_value() as i64 {
        i32::max_value()
    } else if res < i32::min_value() as i64 {
        i32::min_value()
    } else {
        res as i32
    }
}

// log-based
pub fn divide_2(dividend: i32, divisor: i32) -> i32 {
    if dividend == 0 {
        return 0;
    }
    if divisor == 0 {
        return i32::max_value();
    }

    let mut res = ((dividend as f64).abs().ln() - (divisor as f64).abs().ln()).exp();

    if (dividend < 0) ^ (divisor < 0) {
        res = -res;
    }
    if res > i32::max_value() as f64 {
        i32::max_value()
    } else if res < i32::min_value() as f64 {
        i32::min_value()
    } else {
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(divide(10, 3), 3);
    }
}
