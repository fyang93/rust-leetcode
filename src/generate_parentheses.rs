pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    if n == 0 {
        results.push(String::new());
    }
    else {
        for i in 0..n {
            for left in generate_parenthesis(i) {
                for right in generate_parenthesis(n-1-i) {
                    results.push(format!("({}){}", left, right));
                }
            }
        }
    }
    results
}

pub fn generate_parenthesis_1(n: i32) -> Vec<String> {
    let mut results: Vec<String> = vec![];
    backtrack(&mut results, String::new(), 0, 0, n);
    results
}

pub fn backtrack(results: &mut Vec<String>, s: String, n_open: i32, n_close: i32, n: i32) {
    if s.len() == 2 * n as usize {
        results.push(s.to_string());
    } else {
        if n_open < n {
            backtrack(results, format!("{}(", s), n_open + 1, n_close, n);
        }
        if n_close < n_open {
            backtrack(results, format!("{})", s), n_open, n_close + 1, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut res = generate_parenthesis(3);
        res.sort();
        let ans = vec![
          String::from("((()))"),
          String::from("(()())"),
          String::from("(())()"),
          String::from("()(())"),
          String::from("()()()"),
        ];
        assert_eq!(res, ans);
    }
}
