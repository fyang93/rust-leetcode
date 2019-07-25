pub fn num_decodings(s: String) -> i32 {
    if s.is_empty() { return 0; }

    // first: cur 后第一个位置上的way数，second: cur 后第二个位置上的way数
    let (mut first, mut second) = (1, 0);
    // 只要second初始值为0，old_byte可以随便初始化
    let mut old_byte = b'0';
    for &b in s.as_bytes().iter().rev() {
        let mut cur = if b == b'0' { 0 } else { first };
        if b == b'1' || (b == b'2' && old_byte <= b'6') {
            cur += second;
        }
        second = first;
        first = cur;
        old_byte = b;
        if first == 0 && second == 0 {
            return 0;
        }
    }

    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(num_decodings(String::from("02")), 0);
        assert_eq!(num_decodings(String::from("27")), 1);
        assert_eq!(num_decodings(String::from("12")), 2);
        assert_eq!(num_decodings(String::from("226")), 3);
    }
}
