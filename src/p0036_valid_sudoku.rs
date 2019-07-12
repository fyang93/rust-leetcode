use std::collections::HashMap;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![HashMap::new(); 9];
    let mut cols = vec![HashMap::new(); 9];
    let mut boxes = vec![HashMap::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            if let Some(num) = board[i][j].to_digit(10) {
                let box_id = i / 3 * 3 + j / 3;
                let count = rows[i].entry(num).or_insert(0);
                if *count > 0 {
                    return false;
                }
                *count += 1;
                let count = cols[j].entry(num).or_insert(0);
                if *count > 0 {
                    return false;
                }
                *count += 1;
                let count = boxes[box_id].entry(num).or_insert(0);
                if *count > 0 {
                    return false;
                }
                *count += 1;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sudoku(board), true);
    }
}
