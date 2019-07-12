// 矩阵对角化解Fibonacci通式
pub fn climb_stairs(n: i32) -> i32 {
    let sqrt5 = 5f64.sqrt();
    let fib_n = (((1. + sqrt5) / 2.).powi(n + 1) - ((1. - sqrt5) / 2.).powi(n + 1)) / sqrt5;
    fib_n.round() as i32
}

pub fn climb_stairs_fib(n: i32) -> i32 {
    (1..n).fold((1, 1), |(a, b), _| (b, a + b)).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(climb_stairs(10), 89);
        assert_eq!(climb_stairs_fib(10), 89);
    }
}
