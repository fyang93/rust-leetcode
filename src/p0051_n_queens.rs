// bitmask: 记录下一行哪些位置被占用
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut result = vec![];
    let mut solutions = vec![];
    solve(&mut result, &mut solutions, 0, 0, 0, 0, n);
    result
}

fn solve(result: &mut Vec<Vec<String>>, solutions: &mut Vec<usize>, mask_col: i32, mask_45: i32, mask_135: i32, row: usize, n:usize) {
    if row == n {
        result.push(to_strings(solutions));
        return;
    }

    for col in 0..n {
        let flag = 1 << col;
        if (mask_col | mask_45 | mask_135) & flag == 0 {
            solutions.push(col);
            // 下一行的当前列位置被占用，当前列的前一列位置也被占用(45度)，当前列的后一列位置也被占用(135度)
            solve(result, solutions, mask_col | flag, (mask_45 | flag) >> 1, (mask_135 | flag) << 1, row + 1, n);
            solutions.pop();
        }
    }
}


// bitmask: 记录列以及45、135度对角线是否可用
pub fn solve_n_queens_flag(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut result = vec![];
    let mut solutions = vec![];
    let mut flag_col = vec![true; n];
    let mut flag_45 = vec![true; 2 * n - 1];
    let mut flag_135 = vec![true; 2 * n - 1];
    solve_flag(&mut result, &mut solutions, &mut flag_col, &mut flag_45, &mut flag_135, 0, n);
    result
}

fn solve_flag(result: &mut Vec<Vec<String>>, solutions: &mut Vec<usize>, flag_col: &mut Vec<bool>, flag_45: &mut Vec<bool>, flag_135: &mut Vec<bool>, row: usize, n:usize) {
    if row == n {
        result.push(to_strings(solutions));
        return;
    }

    for col in 0..n {
        let (i, j) = (row + col, n - 1 - row + col);
        if flag_col[col] && flag_45[i] && flag_135[j] {
            flag_col[col] = false;
            flag_45[i] = false;
            flag_135[j] = false;
            solutions.push(col);
            solve_flag(result, solutions, flag_col, flag_45, flag_135, row + 1, n);
            solutions.pop();
            flag_col[col] = true;
            flag_45[i] = true;
            flag_135[j] = true;
        }
    }
}

// backtrack
pub fn solve_n_queens_backtrack(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut result: Vec<Vec<String>> = vec![];
    let mut board = vec![vec!['.'; n]; n];
    solve_backtrack(&mut result, &mut board, 0, n);
    result
}

fn solve_backtrack(result: &mut Vec<Vec<String>>, board: &mut Vec<Vec<char>>, row: usize, n: usize) {
    if row == n {
        result.push(board.iter().map(|chars| chars.iter().collect()).collect());
        return;
    }

    for col in 0..n {
        if is_valid(board, row, col, n) {
            board[row][col] = 'Q';
            solve_backtrack(result, board, row + 1, n);
            board[row][col] = '.';
        }
    }
}

fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, n: usize) -> bool {
    // 因为每一行填一个Q，所以无需按行检查
    for step in 1..=row {
        let r = row - step;
        // 检查列
        if board[r][col] == 'Q' { return false; }
        // 检查45度对角线
        let c = col + step;
        if c < n && board[r][c] == 'Q' { return false; }
        // 检查135度对角线
        if let Some(c) = col.checked_sub(step) {
            if board[r][c] == 'Q' {
                return false;
            }
        }
    }
    true
}

fn to_strings(solution: &Vec<usize>) -> Vec<String> {
    let mut strs = vec![];
    for &i in solution {
        strs.push((0..solution.len()).map(|j| if i == j { 'Q' } else { '.' }).collect());
    }
    strs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let target = vec![
            vec![
                String::from(".Q.."),
                String::from("...Q"),
                String::from("Q..."),
                String::from("..Q."),
            ],
            vec![
                String::from("..Q."),
                String::from("Q..."),
                String::from("...Q"),
                String::from(".Q.."),
            ],
        ];
        assert_eq!(solve_n_queens(4), target);
        assert_eq!(solve_n_queens_flag(4), target);
        assert_eq!(solve_n_queens_backtrack(4), target);
    }
}
