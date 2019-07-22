pub fn gray_code(n: i32) -> Vec<i32> {
    (0..1 << n).map(|i| i ^ (i >> 1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(gray_code(0), vec![0]);
        assert_eq!(gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
    }
}
