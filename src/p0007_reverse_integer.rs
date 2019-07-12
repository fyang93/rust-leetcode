pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut rev: i32 = 0;
    loop {
        let pop = x % 10;
        match rev.checked_mul(10) {
            Some(v) => {
                match v.checked_add(pop) {
                    Some(x) => rev = x,
                    None => break,
                }
            }
            None => break,
        }

        x /= 10;
        if x == 0 {
            return rev;
        }
    }
    0
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
