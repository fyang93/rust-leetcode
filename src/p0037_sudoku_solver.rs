pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    solve(board);
}

pub fn solve(board: &mut Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                for num in 1..=9 {
                    let c = std::char::from_digit(num, 10).unwrap();
                    if is_valid(board, i, j, c) {
                        board[i][j] = c;
                        if solve(board) {
                            return true;
                        } else {
                            board[i][j] = '.';
                        }
                    }
                }
                return false;
            }
        }
    }
    true
}

fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
    let (_row, _col) = (row / 3 * 3, col / 3 * 3);
    for i in 0..9 {
        if board[i][col] == c {
            return false;
        }
        if board[row][i] == c {
            return false;
        }
        if board[_row + i / 3][_col + i % 3] == c {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut board = vec![
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
        let expected = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        solve_sudoku(&mut board);
        assert_eq!(board, expected);
    }
}
