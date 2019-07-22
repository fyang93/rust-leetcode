pub fn is_scramble(s1: String, s2: String) -> bool {
    if s1 == s2 {
        return true;
    }
    if !is_anagram(&s1, &s2) {
        return false;
    }

    for i in 1..s1.len() {
        if is_scramble(s1[0..i].to_string(), s2[0..i].to_string())
            && is_scramble(s1[i..].to_string(), s2[i..].to_string())
        {
            return true;
        }
        if is_scramble(s1[0..i].to_string(), s2[s2.len() - i..].to_string())
            && is_scramble(s1[i..].to_string(), s2[0..s2.len() - i].to_string())
        {
            return true;
        }
    }
    false
}

fn is_anagram(s1: &String, s2: &String) -> bool {
    let mut counts = [0; 26];
    for &b1 in s1.as_bytes() {
        counts[(b1 - b'a') as usize] += 1;
    }
    for &b2 in s2.as_bytes() {
        counts[(b2 - b'a') as usize] -= 1;
    }
    counts.iter().all(|&x| x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_scramble(String::from("great"), String::from("rgeat")));
        assert!(is_scramble(String::from("abb"), String::from("bba")));
        assert!(!is_scramble(String::from("abcde"), String::from("caebd")));
    }
}
