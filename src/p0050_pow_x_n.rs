pub fn my_pow(x: f64, n: i32) -> f64 {
    match n {
        0 => 1.0,
        1 => x,
        -1 => 1. / x,
        n => {
            let y = my_pow(x, n / 2);
            y * y * my_pow(x, n % 2)
        }
    }
}

pub fn nearly_equal(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b { // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < std::f64::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (std::f64::EPSILON * std::f64::MIN_POSITIVE)
    } else { // Use relative error.
        (diff / (abs_a + abs_b).min(std::f64::MAX)) < std::f64::EPSILON
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(nearly_equal(my_pow(2., 10), 1024.0));
        assert!(nearly_equal(my_pow(2.1, 3), 9.261));
        assert!(nearly_equal(my_pow(2., -2), 0.25));
    }
}
