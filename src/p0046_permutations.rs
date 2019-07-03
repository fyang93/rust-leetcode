pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut perm = vec![];
    dfs(&nums, &mut perm, &mut result);
    result
}

fn dfs(nums: &Vec<i32>, perm: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if perm.len() == nums.len() {
        result.push(perm.clone())
    } else {
        for &num in nums.iter() {
            if perm.contains(&num) {
                continue;
            }
            perm.push(num);
            dfs(nums, perm, result);
            perm.pop();
        }
    }
}

// [1,2,3] -> [1,2,3], [2,1,3], [3,2,1] -> [1,2,3], [2,1,3], [3,2,1], [1,3,2], [2,3,1], [3,1,2]
pub fn permute_bfs(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![nums.clone()];

    for i in 0..nums.len() - 1 {
        for j in 0..result.len() {
            for k in i + 1..nums.len() {
                let mut perm = result[j].clone();
                perm.swap(i, k);
                result.push(perm);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result1 = permute(vec![1, 2, 3]);
        let mut result2 = permute_bfs(vec![1, 2, 3]);
        result2.sort();
        assert_eq!(result1, result2);
        assert_eq!(
            result1,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        );
    }
}
