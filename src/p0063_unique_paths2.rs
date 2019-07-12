pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if obstacle_grid[0][0] == 1 { return 0; }
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    let mut row = vec![0; n];
    row[0] = 1;
    for i in 0..m {
        row[0] &= !obstacle_grid[i][0];
        for j in 1..n {
            if obstacle_grid[i][j] == 0 {
                row[j] += row[j - 1];
            } else {
                row[j] = 0;
            }
        }
    }
    row[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(unique_paths_with_obstacles(input), 2);
    }
}
