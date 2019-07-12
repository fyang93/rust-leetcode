// bottom-up
pub fn min_distance(word1: String, word2: String) -> i32 {
    let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
    let (m, n) = (word1.len(), word2.len());
    let mut memo = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        memo[i][0] = i as i32;
    }
    for j in 1..=n {
        memo[0][j] = j as i32;
    }
    for i in 1..=m {
        for j in 1..=n {
            if word1[i - 1] == word2[j - 1] {
                memo[i][j] = memo[i - 1][j - 1];
            } else {
                memo[i][j] = memo[i - 1][j].min(memo[i][j - 1]).min(memo[i - 1][j - 1]) + 1;
            }
        }
    }
    memo[m][n]
}

pub fn min_distance_optim(word1: String, word2: String) -> i32 {
    let (word1, word2) = (word1.into_bytes(), word2.into_bytes());
    let (m, n) = (word1.len(), word2.len());
    let mut memo: Vec<_> = (0..=n as i32).collect();
    let mut pre;
    let mut tmp;

    for i in 1..=m {
        tmp = memo[0];
        memo[0] = i as i32;

        for j in 1..=n {
            pre = tmp;
            tmp = memo[j];
            if word1[i - 1] == word2[j - 1] {
                memo[j] = pre;
            } else {
                memo[j] = pre.min(memo[j]).min(memo[j - 1]) + 1;
            }
        }
    }

    memo[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(min_distance_optim(String::from("horse"), String::from("ros")), 3);
    }
}
