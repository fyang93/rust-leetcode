pub fn is_match(s: String, p: String) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());

    // 1. 遍历求解，12ms
    // is_match_recur(&s, &p)

    // 2. 动态规划
    // 2.1 从上往下
    // let mut memo = vec![vec![None; p.len() + 1]; s.len() + 1];  // 注意 +1
    // dp_top_down(0, 0, &s, &p, &mut memo)

    // 2.2 自下向上动态规划
    dp_bottom_up(&s, &p)
}

pub fn is_match_recur(s: &[u8], p: &[u8]) -> bool {
    if p.is_empty() {
        return s.is_empty();
    }
    let first_match = !s.is_empty() && (s[0] == p[0] || p[0] == b'.');
    match p.get(1) {
        // (aab, a*b) -> (aab, b)       *表示0个
        // (aab, a*b) -> (ab, a*b)      *表示多个
        Some(b'*') => is_match_recur(s, &p[2..]) || (first_match && is_match_recur(&s[1..], &p)),
        // (aab, a.b) -> (ab, .b)
        // (aab, aab) -> (ab, ab)
        _ => first_match && is_match_recur(&s[1..], &p[1..]),
    }
}

pub fn dp_top_down(i: usize, j: usize, s: &[u8], p: &[u8], memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    match memo[i][j] {
        Some(b) => return b,
        _ => {
            let ans: bool;
            if j == p.len() {
                ans = i == s.len();
            } else {
                let first_match = (i < s.len()) && (s[i] == p[j] || p[j] == b'.');
                match p.get(j + 1) {
                    Some(b'*') => {
                        ans = dp_top_down(i, j + 2, s, p, memo)
                            || (first_match && dp_top_down(i + 1, j, s, p, memo))
                    }
                    _ => ans = first_match && dp_top_down(i + 1, j + 1, s, p, memo),
                }
            }
            memo[i][j] = Some(ans);
            return ans;
        }
    }
}

pub fn dp_bottom_up(s: &[u8], p: &[u8]) -> bool {
    let mut memo = vec![vec![false; p.len() + 1]; s.len() + 1]; // 注意 +1
    memo[s.len()][p.len()] = true;

    for i in (0..=s.len()).rev() {
        // j == p.len() && i < s.len() 时 memo[i][j] 比为 false，因此不需要考虑 j = p.len()
        for j in (0..p.len()).rev() {
            let first_match = i < s.len() && (s[i] == p[j] || p[j] == b'.');
            match p.get(j + 1) {
                Some(b'*') => memo[i][j] = memo[i][j + 2] || first_match && memo[i + 1][j],
                _ => memo[i][j] = first_match && memo[i + 1][j + 1],
            }
        }
    }

    memo[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_match(String::from("aa"), String::from("a")), false);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(is_match(String::from("abcd"), String::from(".*c")), false);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(is_match(String::from("aab"), String::from("c*a*b")), true);
    }
}
