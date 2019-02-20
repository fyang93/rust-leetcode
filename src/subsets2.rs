pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut path = vec![];
    let mut nums = nums;
    nums.sort();
    dfs(&nums, 0, &mut path, &mut res);
    res
}

fn dfs(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    res.push(path.clone());
    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }
        path.push(nums[i]);
        dfs(&nums, i + 1, path, res);
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = subsets_with_dup(vec![1, 2, 2]);
        assert_eq!(
            res,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ]
        );
    }
}
