pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() || target < matrix[0][0] {
        return false;
    }

    let (mut i, mut j) = (0, matrix.len());
    while i + 1 < j {
        let m = (j - i) / 2 + i;
        if matrix[m][0] == target {
            return true;
        } else if matrix[m][0] > target {
            j = m;
        } else {
            i = m;
        }
    }

    matrix[i].binary_search(&target).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]];
        assert_eq!(search_matrix(matrix, 13), false);
    }
}
