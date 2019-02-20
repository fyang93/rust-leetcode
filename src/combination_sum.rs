pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut path = vec![];
    dfs(&candidates, target, 0, &mut path, 0, &mut res);
    res
}

fn dfs(nums: &Vec<i32>, target: i32, start: usize, path: &mut Vec<i32>, sum: i32, res: &mut Vec<Vec<i32>>) {
    if sum == target {
        res.push(path.clone());
    } else if sum < target {
        for i in start..nums.len() {
            path.push(nums[i]);
            dfs(nums, target, i, path, sum + nums[i], res);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = combination_sum(vec![2, 3, 6, 7], 7);
        assert_eq!(res, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn it_works_1() {
        let res = combination_sum(vec![2, 3, 5], 8);
        assert_eq!(res, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }
}
