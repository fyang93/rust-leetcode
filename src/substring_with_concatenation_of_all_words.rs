use std::collections::HashMap;

// 有点类似 bag-of-words
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut indices = vec![];
    if words.is_empty() {
        return indices;
    }

    // 所有单词长度一样
    let word_len = words[0].len();
    let window = word_len * words.len();
    if s.len() < window {
        return indices;
    }

    let mut word_map = HashMap::new();
    for word in &words {
        let count = word_map.entry(word.as_str()).or_insert(0);
        *count += 1;
    }

    for i in 0..=(s.len() - window) {
        let substr = &s[i..i + window];
        let mut map = HashMap::new();
        let mut num = 0;
        for j in (0..window).step_by(word_len) {
            let sub = &substr[j..j + word_len];
            if word_map.contains_key(sub) {
                let count = map.entry(sub).or_insert(0);
                *count += 1;
                if *count > *word_map.get(sub).unwrap() {
                    break;
                }
            } else {
                break;
            }
            num += 1;
        }
        if num == words.len() {
            indices.push(i as i32);
        }
    }

    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("barfoothefoobarman");
        let words = vec![String::from("foo"), String::from("bar")];
        assert_eq!(find_substring(s, words), vec![0, 9]);
    }

    #[test]
    fn it_works_1() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![String::from("word"), String::from("good"), String::from("best"), String::from("word")];
        assert_eq!(find_substring(s, words), vec![]);
    }

    #[test]
    fn it_works_2() {
        let s = String::from("wordgoodgoodgoodbestword");
        let words = vec![String::from("word"), String::from("good"), String::from("best"), String::from("good")];
        assert_eq!(find_substring(s, words), vec![8]);
    }

    #[test]
    fn it_works_3() {
        let s = String::from("");
        let words = vec![String::from("a")];
        assert_eq!(find_substring(s, words), vec![]);
    }
}
