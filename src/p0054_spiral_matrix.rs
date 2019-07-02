pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    if matrix.is_empty() { return result; }
    let (mut t, mut r, mut b, mut l) = (0, matrix[0].len(), matrix.len(), 0);
    loop {
        for i in l..r { result.push(matrix[t][i]); }
        t += 1;
        if t >= b { break; }    // 单行
        for i in t..b { result.push(matrix[i][r - 1]); }
        r -= 1;
        if l >= r { break; }    // 单列
        for i in (l..r).rev() { result.push(matrix[b - 1][i]); }
        b -= 1;
        if t >= b { break; }    // 双行
        for i in (t..b).rev() { result.push(matrix[i][l]); }
        l += 1;
        if l >= r { break; }    // 双列
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![vec![7], vec![9], vec![6]];    // 单列
        let target = vec![7, 9, 6];
        assert_eq!(spiral_order(input), target);
    }

    #[test]
    fn it_works_1() {
        let input = vec![vec![7, 9, 6]];    // 单行
        let target = vec![7, 9, 6];
        assert_eq!(spiral_order(input), target);
    }

    #[test]
    fn it_works_2() {
        let input = vec![vec![2 ,5, 8],vec![4, 0, -1]]; // 双行
        let target = vec![2, 5, 8, -1, 0, 4];
        assert_eq!(spiral_order(input), target);
    }

    #[test]
    fn it_works_3() {
        let input = vec![vec![1 ,11],vec![2, 12], vec![3, 13], vec![4, 14]];    // 双列
        let target = vec![1, 11, 12, 13, 14, 4, 3, 2];
        assert_eq!(spiral_order(input), target);
    }
}
