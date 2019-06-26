pub fn transpose(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (n, m) = (a.len(), a[0].len());
    let mut b = vec![vec![0; n]; m];
    for i in 0..n {
        for j in 0..m {
            b[j][i] = a[i][j];
        }
    }
    b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input= vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let output= vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
        ];
        assert_eq!(transpose(input), output);
    }

    #[test]
    fn it_works_1() {
        let mut input= vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        let output= vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ];
        assert_eq!(transpose(input), output);
    }
}
