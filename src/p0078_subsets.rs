pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut subset = vec![];
    dfs(&nums, 0, &mut subset, &mut result);
    result
}

fn dfs(nums: &Vec<i32>, start: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(subset.clone());
    for i in start..nums.len() {
        subset.push(nums[i]);
        dfs(nums, i + 1, subset, result);
        subset.pop();
    }
}

pub fn subsets_bfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![vec![]];

    for &n in nums.iter() {
        for i in 0..result.len() {
            let mut subset = result[i].clone();
            subset.push(n);
            result.push(subset);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result1 = subsets(vec![1, 2, 3]);
        let mut result2 = subsets_bfs(vec![1, 2, 3]);
        result2.sort();
        assert_eq!(result1, result2);
        assert_eq!(
            result1,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ]
        );
    }
}
