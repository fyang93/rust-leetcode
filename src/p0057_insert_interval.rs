pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut new_interval = new_interval;
    let n = intervals.len();
    let mut i = 0;

    while i < n && intervals[i][1] < new_interval[0] {
        result.push(intervals[i].clone());
        i += 1;
    }
    while i < n && intervals[i][0] <= new_interval[1] {
        new_interval[0] = new_interval[0].min(intervals[i][0]);
        new_interval[1] = new_interval[1].max(intervals[i][1]);
        i += 1;
    }
    result.push(new_interval);
    while i < n {
        result.push(intervals[i].clone());
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let target = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(insert(intervals, new_interval), target);
    }

    #[test]
    fn it_works_1() {
        let intervals = vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]];
        let new_interval = vec![4, 8];
        let target = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(insert(intervals, new_interval), target);
    }

    #[test]
    fn it_works_2() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 0];
        let target = vec![vec![0, 0], vec![1, 5]];
        assert_eq!(insert(intervals, new_interval), target);
    }

    #[test]
    fn it_works_3() {
        let intervals = vec![];
        let new_interval = vec![5, 7];
        let target = vec![vec![5, 7]];
        assert_eq!(insert(intervals, new_interval), target);
    }

    #[test]
    fn it_works_4() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 7];
        let target = vec![vec![1, 7]];
        assert_eq!(insert(intervals, new_interval), target);
    }
}
