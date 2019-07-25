pub fn reverse(mut x: i32) -> i32 {
    let mut rev: i32 = 0;
    while x != 0 {
        match rev.checked_mul(10).and_then(|v| v.checked_add(x % 10)) {
            Some(v) => rev = v,
            None => return 0,
        }
        x /= 10;
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(reverse(120), 21);
    }
}
