// 可以看作组合问题，n个箱里放m个球，可以有空箱
// (n+m-2) choose (n-1) or (m-1)
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let upper = n + m - 1;
    (1..m.min(n)).fold(1, |res, x| res * (upper - x) as u64 / x as u64) as i32
}

// 每个格子的路径数=左边格子路径数+上面格子路径数
pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    // 每次下一行都默认填上当前行（上面格子）的值然后根据左边格子更新
    let mut row = vec![1; n as usize];
    for _ in 1..m {
        for i in 1..n {
            row[i] += row[i - 1];
        }
    }
    row[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(unique_paths(10, 10), 48620);
        assert_eq!(unique_paths(19, 13), 86493225);
    }

    #[test]
    fn it_works_dp() {
        assert_eq!(unique_paths_dp(10, 10), 48620);
        assert_eq!(unique_paths_dp(19, 13), 86493225);
    }
}
