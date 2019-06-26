// Tags: DFS

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut subset = vec![];
    let mut nums = nums;
    nums.sort();
    dfs(&nums, 0, &mut subset, &mut result);
    result
}

fn dfs(nums: &Vec<i32>, start: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(subset.clone());
    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue;
        }
        subset.push(nums[i]);
        dfs(&nums, i + 1, subset, result);
        subset.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = subsets_with_dup(vec![1, 2, 2]);
        assert_eq!(
            result,
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
