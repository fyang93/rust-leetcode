pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![]
    }

    let letters = vec![vec![], vec![' '], vec!['a', 'b', 'c'], vec!['d', 'e', 'f'],
                       vec!['g', 'h', 'i'], vec!['j', 'k', 'l'], vec!['m', 'n', 'o'],
                       vec!['p', 'q', 'r', 's'], vec!['t', 'u', 'v'], vec!['w', 'x', 'y', 'z']];
    let ids: Vec<_> = digits.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut results = vec![String::new()];

    for i in &ids {
        let mut temp: Vec<String> = vec![];
        for c in &letters[*i] {
            for r in &results {
                temp.push(format!("{}{}", r, c));
            }
        }
        results = temp;
    }
    results
}

pub fn letter_combinations_1(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![]
    }

    let letters = vec![vec![], vec![' '], vec!['a', 'b', 'c'], vec!['d', 'e', 'f'],
                       vec!['g', 'h', 'i'], vec!['j', 'k', 'l'], vec!['m', 'n', 'o'],
                       vec!['p', 'q', 'r', 's'], vec!['t', 'u', 'v'], vec!['w', 'x', 'y', 'z']];
    let nums: Vec<_> = letters.iter().map(|s| s.len()).collect();
    let ids: Vec<_> = digits.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let size = ids.iter().map(|&i| nums[i]).product();
    let mut results = vec![String::new(); size];

    let mut stride = size;
    for c in digits.chars() {
        let i = c.to_digit(10).unwrap() as usize;

        stride /= nums[i];
        for j in 0..size {
            results[j].push(letters[i][j / stride % nums[i]]);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = String::from("23");
        let output: Vec<String> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter()
                                      .map(|s| s.to_string()).collect();
        let mut results = letter_combinations(input);
        results.sort();
        assert_eq!(results, output);
    }
}
