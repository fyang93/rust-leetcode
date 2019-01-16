use std::io;
use std::cmp;
use std::error::Error;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hash = HashMap::new();
    let (mut left, mut right) = (0, 1);
    let mut len = 0;
    for c in s.chars() {
        if let Some(v) = hash.get(&c) {
            left = cmp::max(left, *v);
        }
        len = cmp::max(len, right - left);
        hash.insert(c, right);
        right += 1;
    }
    len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("abcabcbb");
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn it_works_1() {
        let s = String::from("bbbbbb");
        assert_eq!(length_of_longest_substring(s), 1);
    }

    #[test]
    fn it_works_2() {
        let s = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(s), 3);
    }
}
