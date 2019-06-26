// clockwise rotate
// reverse row, then transpose
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    matrix.reverse();
    for i in 0..matrix.len() {
        for j in 0..i {
            let (a, b) = matrix.split_at_mut(i);
            std::mem::swap(&mut a[j][i], &mut b[0][j]);
        }
    }
}

// anti-clockwise rotate
// transpose, then reverse row
pub fn anti_rotate(matrix: &mut Vec<Vec<i32>>) {
    for i in 0..matrix.len() {
        for j in 0..i {
            let (a, b) = matrix.split_at_mut(i);
            std::mem::swap(&mut a[j][i], &mut b[0][j]);
        }
    }
    matrix.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input= vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let output= vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        rotate(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn it_works_1() {
        let mut input= vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let output= vec![
            vec![11, 10, 7, 16],
            vec![9, 8, 6, 12],
            vec![1, 4, 3, 14],
            vec![5, 2, 13, 15],
        ];
        anti_rotate(&mut input);
        assert_eq!(input, output);
    }
}
