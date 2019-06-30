pub fn total_n_queens(n: i32) -> i32 {
    let mut count = 0;
    solve(&mut count, 0, 0, 0, 0, n as usize);
    count
}

fn solve(count: &mut i32, mask_col: i32, mask_45: i32, mask_135: i32, row: usize, n:usize) {
    if row == n {
        *count += 1;
        return;
    }

    for col in 0..n {
        let flag = 1 << col;
        if (mask_col | mask_45 | mask_135) & flag == 0 {
            // 下一行的当前列位置被占用，当前列的前一列位置也被占用(45度)，当前列的后一列位置也被占用(135度)
            solve(count, mask_col | flag, (mask_45 | flag) >> 1, (mask_135 | flag) << 1, row + 1, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(total_n_queens(4), 2);
    }
}
