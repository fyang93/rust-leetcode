// two-pass
pub fn longest_valid_parentheses(s: String) -> i32 {
    let (mut left, mut right) = (0, 0);
    let mut longest = 0;

    for c in s.chars() {
        match c {
            '(' => left += 1,
            ')' => right += 1,
            _ => (),
        }
        if left == right {
            longest = longest.max(left + right);
        } else if left < right {
            left = 0;
            right = 0;
        }
    }

    left = 0;
    right = 0;

    for c in s.chars().rev() {
        match c {
            '(' => left += 1,
            ')' => right += 1,
            _ => (),
        }
        if left == right {
            longest = longest.max(left + right);
        } else if left > right {
            left = 0;
            right = 0;
        }
    }

    longest
}

pub fn longest_valid_parentheses_stack(s: String) -> i32 {
    let chars: Vec<_> = s.chars().collect();
    let mut stack: Vec<isize> = vec![-1];
    let mut longest = 0;

    for i in 0..s.len() {
        if chars[i] == '(' {
            stack.push(i as isize);
        } else {
            stack.pop();
            if stack.is_empty() {
                stack.push(i as isize);
            } else {
                longest = longest.max(i as isize - stack.last().unwrap());
            }
        }
    }

    longest as i32
}

pub fn longest_valid_parentheses_dp(s: String) -> i32 {
    let n = s.len();
    let chars: Vec<_> = s.chars().collect();
    let mut dp = vec![0; n];
    let mut longest = 0;

    for i in 1..n {
        if chars[i] == ')' {
            if chars[i - 1] == '(' {
                dp[i] = match dp.get(i - 2) {
                    Some(&v) => v,
                    None => 0,
                } + 2;
            } else if chars[i - 1] == ')' {
                if let Some('(') = chars.get(i - dp[i - 1] - 1) {
                    dp[i] = dp[i - 1]
                        + 2
                        + match dp.get(i - dp[i - 1] - 2) {
                            Some(&v) => v,
                            None => 0,
                        };
                }
            }
            longest = longest.max(dp[i]);
        }
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("(()");
        assert_eq!(longest_valid_parentheses(s.clone()), 2);
        assert_eq!(longest_valid_parentheses_stack(s.clone()), 2);
        assert_eq!(longest_valid_parentheses_dp(s), 2);
    }

    #[test]
    fn it_works_1() {
        let s = String::from(")()())");
        assert_eq!(longest_valid_parentheses(s.clone()), 4);
        assert_eq!(longest_valid_parentheses_stack(s.clone()), 4);
        assert_eq!(longest_valid_parentheses_dp(s), 4);
    }
}
