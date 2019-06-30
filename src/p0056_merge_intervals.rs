pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() <= 1 {
        return intervals;
    }
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut merged: Vec<Vec<i32>> = vec![intervals[0].clone()];
    for interval in intervals.into_iter().skip(1) {
        if let Some(last) = merged.last_mut() {
            // 存在重叠
            if last[1] >= interval[0] {
                last[1] = last[1].max(interval[1]);
                continue;
            }
        }
        merged.push(interval);
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let intervals = vec![vec![2, 6], vec![1, 3], vec![8, 10], vec![15, 18]];
        let target = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(merge(intervals), target);
    }

    #[test]
    fn it_works_1() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let target = vec![vec![1, 5]];
        assert_eq!(merge(intervals), target);
    }
}
