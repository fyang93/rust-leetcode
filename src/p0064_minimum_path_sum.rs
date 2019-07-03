pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut row = grid[0].clone();
    for i in 1..n {
        row[i] += row[i - 1];
    }
    for i in 1..m {
        row[0] += grid[i][0];
        for j in 1..n {
            row[j] = row[j].min(row[j - 1]) + grid[i][j];
        }
    }
    row[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(min_path_sum(input), 7);
    }
}
