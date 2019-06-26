pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut path = vec![];
    let mut nums = candidates;
    nums.sort();
    dfs(&nums, target, 0, &mut path, 0, &mut res);
    res
}

fn dfs(nums: &Vec<i32>, target: i32, start: usize, path: &mut Vec<i32>, sum: i32, res: &mut Vec<Vec<i32>>) {
    if sum == target {
        res.push(path.clone());
    } else if sum < target {
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            path.push(nums[i]);
            dfs(nums, target, i + 1, path, sum + nums[i], res);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        assert_eq!(
            res,
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }
}
