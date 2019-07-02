pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    if n <= 0 { return vec![]; }
    let n = n as usize;
    let mut matrix = vec![vec![0; n]; n];
    let mut x = 1;
    let (mut t, mut r, mut b, mut l) = (0, n, n, 0);
    while l < r {
        for i in l..r {
            matrix[t][i] = x;
            x += 1;
        }
        t += 1;
        for i in t..b {
            matrix[i][r - 1] = x;
            x += 1;
        }
        r -= 1;
        for i in (l..r).rev() {
            matrix[b - 1][i] = x;
            x += 1;
        }
        b -= 1;
        for i in (t..b).rev() {
            matrix[i][l] = x;
            x += 1;
        }
        l += 1;
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let target = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(generate_matrix(3), target);
    }
}
