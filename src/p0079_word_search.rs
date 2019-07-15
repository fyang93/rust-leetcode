pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let chars: Vec<char> = word.chars().collect();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if backtrack(&mut board, &chars, i, j, 0) {
                return true;
            }
        }
    }
    false
}

fn backtrack(board: &mut Vec<Vec<char>>, chars: &Vec<char>, i: usize, j: usize, mut cur: usize) -> bool {
    let ch = board[i][j];
    if ch != chars[cur]  {
        return false;
    }
    cur += 1;
    if cur == chars.len() {
        return true;
    }
    board[i][j] = '.';
    if i > 0 && backtrack(board, chars, i - 1, j, cur) { return true; }
    if i + 1 < board.len() && backtrack(board, chars, i + 1, j, cur) { return true; }
    if j > 0 && backtrack(board, chars, i, j - 1, cur) { return true; }
    if j + 1 < board[0].len() && backtrack(board, chars, i, j + 1, cur) { return true; }
    board[i][j] = ch;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(exist(board.clone(), String::from("ABCCED")), true);
        assert_eq!(exist(board.clone(), String::from("SEE")), true);
        assert_eq!(exist(board.clone(), String::from("ABCB")), false);
    }

    #[test]
    fn it_works_1() {
        let board = vec![vec!['a']];
        assert_eq!(exist(board, String::from("a")), true);
    }
}
