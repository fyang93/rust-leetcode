// 参考 http://www.matrix67.com/blog/archives/115
pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    let (hay, ndl) = (haystack.as_bytes(), needle.as_bytes());
    let mut pre = vec![0; ndl.len()-1];

    // 预处理
    // pre[j] 表示 j 不匹配时应该重新比对的字符索引
    let mut j = 0;
    for i in 1..ndl.len()-1 {
        while j > 0 && ndl[j] != ndl[i] {
            j = pre[j - 1];
        }
        if ndl[j] == ndl[i] {
            j += 1;
        }
        pre[i] = j;
    }

    j = 0;
    for i in 0..hay.len() {
        while j > 0 && hay[i] != ndl[j] {
            j = pre[j - 1];
        }
        if hay[i] == ndl[j] {
            j += 1;
        }
        if j == ndl.len() {
            return (i - j + 1) as i32
        }
    }

    -1
}

// cheat
pub fn str_str_1(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(l) => l as i32,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(str_str(String::from("aaaaa"), String::from("bba")), -1);
    }
}
