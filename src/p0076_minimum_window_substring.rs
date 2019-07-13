use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    let (sb, tb) = (s.as_bytes(), t.as_bytes());

    let mut hash = HashMap::new();
    for b in tb.iter() {
        *hash.entry(b).or_insert(0) += 1;
    }

    // 需要考虑的字符种数
    let mut count = hash.keys().len();
    let (mut left, mut start, mut end) = (0, 0, sb.len());
    for right in 0..sb.len() {
        if let Some(r) = hash.get_mut(&sb[right]) {
            *r -= 1;
            // 一种字符已经足够
            if *r == 0 {
                count -= 1;
                while count == 0 {
                    if let Some(l) = hash.get_mut(&sb[left]) {
                        // sb[left]是唯一一个剩下来的该类字符
                        if *l == 0 {
                            if right - left < end - start {
                                start = left;
                                end = right;
                            }
                            count += 1;
                        }
                        *l += 1;
                    }
                    left += 1;
                }
            }
        }
    }

    if end == sb.len() {
        return String::new();
    }
    s[start..=end].to_string()
}

pub fn min_window_ascii(s: String, t: String) -> String {
    let (sb, tb) = (s.as_bytes(), t.as_bytes());

    let mut ascii = [0; 128];
    let mut count = 0;
    for &b in tb.iter() {
        if ascii[b as usize] == 0 {
            count += 1;
        }
        ascii[b as usize] += 1;
    }

    let (mut left, mut start, mut end) = (0, 0, sb.len());
    for right in 0..sb.len() {
        ascii[sb[right] as usize] -= 1;
        if ascii[sb[right] as usize] == 0 {
            count -= 1;
            while count == 0 {
                if ascii[sb[left] as usize] == 0 {
                    if right - left < end - start {
                        start = left;
                        end = right;
                    }
                    count += 1;
                }
                ascii[sb[left] as usize] += 1;
                left += 1;
            }
        }
    }

    if end == sb.len() {
        return String::new();
    }
    s[start..=end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(min_window_ascii(String::from("ADOBECODEBANC"), String::from("ABC")), String::from("BANC"));
        assert_eq!(min_window_ascii(String::from("aa"), String::from("aa")), String::from("aa"));
    }
}
