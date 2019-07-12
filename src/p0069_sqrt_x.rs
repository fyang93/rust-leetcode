// 二分法
pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 { return x; }
    let (mut l, mut r) = (0, x);
    while r - l > 1 {
        let m = (r - l) / 2 + l;
        if x / m < m {
            r = m;
        } else {
            l = m;
        }
    }
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(8192), 90);
        assert_eq!(my_sqrt(2147395599), 46339);
    }
}
