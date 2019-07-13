// 遍历索引值
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    traverse(0, n, k, &mut (0..k).collect(), &mut result);
    result
}

fn traverse(i: i32, n: i32, k: i32, indices: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if i == k {
        result.push(indices.iter().map(|&j| j + 1).collect());
    } else {
        // n - indices[i] 是后面剩下的可供选择的个数
        // k - i 是接下来还需要选择的个数
        while n - indices[i as usize] >= k - i {
            traverse(i + 1, n, k, indices, result);
            let start = indices[i as usize] + 1;
            for j in i..k {
                indices[j as usize] = start + j - i;
            }
        }
    }
}

// 递归
pub fn combine_recur(n: i32, k: i32) -> Vec<Vec<i32>> {
    if k == 0 {
        return vec![vec![]];
    }
    let mut result = vec![];
    for i in k..=n {
        for mut pre in combine(i - 1, k - 1) {
            pre.push(i);
            result.push(pre);
        }
    }
    result
}

pub fn combine_dfs(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    dfs(n, k, 0, &mut vec![], &mut result);
    result
}

fn dfs(n: i32, k: i32, start: i32, comb: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if k == 0 {
        result.push(comb.clone());
        return;
    }
    for i in start..=n - k {
        comb.push(i + 1);
        dfs(n, k - 1, i + 1, comb, result);
        comb.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = combine_dfs(4, 3);
        result.sort();
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 3, 4],
                vec![2, 3, 4],
            ]
        );
    }
}
