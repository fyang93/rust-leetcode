pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (rows, cols) = (matrix.len(), matrix[0].len());

    let first_row = (0..cols).any(|j| matrix[0][j] == 0);
    let first_col = (0..rows).any(|i| matrix[i][0] == 0);
    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    if first_row {
        for j in 0..cols {
            matrix[0][j] = 0;
        }
    }

    if first_col {
        for i in 0..rows {
            matrix[i][0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let target = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, target);
    }
}
