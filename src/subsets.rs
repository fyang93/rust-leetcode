pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut path = vec![];
    dfs(&nums, 0, &mut path, &mut res);
    res
}

fn dfs(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    res.push(path.clone());
    for i in start..nums.len() {
        path.push(nums[i]);
        dfs(&nums, i + 1, path, res);
        path.pop();
    }
}

pub fn subsets_bfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![]];

    for &n in nums.iter() {
        for i in 0..res.len() {
            let mut subset = res[i].clone();
            subset.push(n);
            res.push(subset);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res1 = subsets(vec![1, 2, 3]);
        let mut res2 = subsets_bfs(vec![1, 2, 3]);
        res2.sort();
        assert_eq!(res1, res2);
        assert_eq!(
            res1,
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
