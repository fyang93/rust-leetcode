pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let n = bytes.len() as isize;
    let (mut start, mut len) = (0, 0);
    for i in 0..n {
        let len1 = extend(bytes, n, i-1, i+1);
        let len2 = extend(bytes, n, i, i+1);
        let max = len1.max(len2);
        if len < max {
            len = max;
            start = if len1 > len2 {
                i - (len1 / 2)
            } else {
                i - (len2 - 2) / 2
            };
        }
    }
    let (start, len) = (start as usize, len as usize);
    String::from(&s[start..start + len])
}

pub fn extend(bytes: &[u8], n: isize, left: isize, right: isize) -> isize {
    let (mut left, mut right) = (left, right);
    while left >= 0 && right < n {
        if &bytes[left as usize] != &bytes[right as usize] {
            break;
        }
        left -= 1;
        right += 1;
    }
    return right - left - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("babad");
        let res = longest_palindrome(s);
        assert!(res == String::from("aba") || res == String::from("bab"));
    }

    #[test]
    fn it_works_1() {
        let s = String::from("cbbd");
        let res = longest_palindrome(s);
        assert_eq!(res, String::from("bb"));
    }

    #[test]
    fn it_works_2() {
        let s = String::from("bb");
        let res = longest_palindrome(s);
        assert_eq!(res, String::from("bb"));
    }
}
