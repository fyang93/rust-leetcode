pub fn is_match(s: String, p: String) -> bool {
    let (s, p) = (s.as_bytes(), p.as_bytes());

    // 1. 动态规划
    // 1.1 从上往下
    // let mut memo = vec![vec![None; p.len() + 1]; s.len() + 1];
    // dp_top_down(0, 0, &s, &p, &mut memo)

    // 1.2 自下向上
    // dp_bottom_up(&s, &p)

    // 2. 双指针
    two_pointer(&s, &p)
}

pub fn two_pointer(s: &[u8], p: &[u8]) -> bool {
    let (mut i, mut j, mut matched) = (0, 0, 0);
    let mut star = -1;

    while i < s.len() {
        // 字符匹配
        if j < p.len() && (s[i] == p[j] || p[j] == b'?') {
            i += 1;
            j += 1;
        }
        // 字符不匹配，但匹配到了 *
        // 把 * 当作空继续
        else if j < p.len() && p[j] == b'*' {
            matched = i;
            star = j as i32;
            j += 1;
        }
        // 字符不匹配，也没有匹配到 *，说明之前的 * 不能被当作空
        // 尝试让 * 匹配掉一个字符看看后面能不能匹配上
        else if star != -1 {
            j = star as usize + 1;
            matched += 1;
            i = matched;
        } else {
            return false;
        }
    }

    while j < p.len() && p[j] == b'*' {
        j += 1;
    }

    j == p.len()
}

pub fn dp_top_down(i: usize, j: usize, s: &[u8], p: &[u8], memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    match memo[i][j] {
        Some(b) => return b,
        _ => {
            let ans: bool;
            if j == p.len() {
                ans = i == s.len();
            } else {
                let first_match = i < s.len() && (s[i] == p[j] || p[j] == b'?' || p[j] == b'*');
                match p[j] {
                    b'*' => {
                        ans = dp_top_down(i, j + 1, s, p, memo)
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
    let (m, n) = (s.len(), p.len());
    let mut memo = vec![vec![false; n + 1]; m + 1];
    memo[s.len()][p.len()] = true;

    for i in (0..=m).rev() {
        // j == p.len() && i < s.len() 时 memo[i][j] 必为 false，因此不需要考虑 j = p.len()
        for j in (0..n).rev() {
            let first_match = i < s.len() && (s[i] == p[j] || p[j] == b'?' || p[j] == b'*');
            match p[j] {
                b'*' => memo[i][j] = memo[i][j + 1] || (first_match && memo[i + 1][j]),
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
        assert_eq!(is_match(String::from("aa"), String::from("*")), true);
        assert_eq!(is_match(String::from("cb"), String::from("?a")), false);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(is_match(String::from("ho"), String::from("ho**")), true);
        assert_eq!(is_match(String::from("adceb"), String::from("*a*b")), true);
        assert_eq!(
            is_match(String::from("acdcb"), String::from("a*c?b")),
            false
        );
    }
}
