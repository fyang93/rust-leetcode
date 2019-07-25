pub fn num_decodings(s: String) -> i32 {
    if s.is_empty() { return 0; }

    let (mut first, mut second) = (1u64, 0u64);
    let mut old_byte = b'0';
    for &b in s.as_bytes().iter().rev() {
        let mut cur = match b { b'0' => 0, b'*' => 9 * first, _ => first };
        cur += match b {
            b'1' => match old_byte { b'*' => 9 * second, _ => second },
            b'2' => match old_byte { b'*' => 6 * second, b'0'..=b'6' => second, _ => 0 },
            b'*' => match old_byte { b'*' => 15 * second, b'0'..=b'6' => 2 * second, b'7'..=b'9' => second, _ => 0 }
            _ => 0,
        };
        second = first;
        first = cur % 1_000_000_007;
        old_byte = b;
        if first == 0 && second == 0 {
            return 0;
        }
    }

    first as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(num_decodings(String::from("**********1111111111")), 133236775);
    }
}
