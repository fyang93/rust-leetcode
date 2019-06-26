// Tags: DFS

// dfs
pub fn permute_unique_dfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut perm = vec![];
    let mut used = vec![false; nums.len()];
    let mut nums = nums;
    nums.sort();
    dfs(&nums, &mut perm, &mut used, &mut result);
    result
}

fn dfs(nums: &Vec<i32>, perm: &mut Vec<i32>, used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
    if perm.len() == nums.len() {
        result.push(perm.clone())
    } else {
        for i in 0..nums.len() {
            if used[i] || i > 0 && nums[i] == nums[i - 1] && !used[i - 1] { continue; } // 剪枝，避免重复
            used[i] = true;
            perm.push(nums[i]);
            dfs(nums, perm, used, result);
            used[i] = false;
            perm.pop();
        }
    }
}

// see next_permutation
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut nums = nums;
    nums.sort();
    result.push(nums.clone());
    while let Some(i) = (1..nums.len()).rev().find(|&i| nums[i - 1] < nums[i]) {
        let j = (i..nums.len())
            .rev()
            .find(|&j| nums[i - 1] < nums[j])
            .unwrap();
        nums.swap(i - 1, j);
        nums[i..].reverse();
        result.push(nums.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result1 = permute_unique_dfs(vec![1, 1, 2]);
        let result2 = permute_unique(vec![1, 1, 2]);
        assert_eq!(result1, result2);
        assert_eq!(result1, vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]);
    }

    #[test]
    fn it_works_1() {
        let result1 = permute_unique_dfs(vec![1, 1, 2, 2]);
        let result2 = permute_unique(vec![1, 1, 2, 2]);
        assert_eq!(result1, result2);
        assert_eq!(
            result1,
            vec![
                vec![1, 1, 2, 2],
                vec![1, 2, 1, 2],
                vec![1, 2, 2, 1],
                vec![2, 1, 1, 2],
                vec![2, 1, 2, 1],
                vec![2, 2, 1, 1]
            ]
        );
    }
}
