#[derive(Clone)]
struct Trie {
    index: Option<usize>,
    children: Vec<Option<Trie>>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            index: None,
            children: vec![None; 26],
        }
    }

    fn insert(&mut self, word: &String, index: usize) {
        let mut trie = self;
        for &b in word.as_bytes() {
            trie = trie.children[(b - b'a') as usize].get_or_insert(Trie::new());
        }
        trie.index = Some(index);
    }
}

pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut found = vec![];
    let mut trie = Trie::new();
    for (i, word) in words.iter().enumerate() {
        trie.insert(word, i);
    }

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            backtrack(&mut board, &mut trie, &mut found, i, j);
        }
    }
    found.iter().map(|&i| words[i].clone()).collect()
}


fn backtrack(board: &mut Vec<Vec<char>>, trie: &mut Trie, found: &mut Vec<usize>, i: usize, j: usize) {
    let ch = board[i as usize][j as usize];
    if ch.is_ascii_lowercase() {
        if let Some(t) = trie.children[(ch as u8 - b'a') as usize].as_mut() {
            t.index.take().map(|k| found.push(k));
            board[i][j] = '.';
            if i > 0 { backtrack(board, t, found, i - 1, j); }
            if i + 1 < board.len() { backtrack(board, t, found, i + 1, j); }
            if j > 0 { backtrack(board, t, found, i, j - 1); }
            if j + 1 < board[0].len() { backtrack(board, t, found, i, j + 1) }
            board[i][j] = ch;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![String::from("oath"), String::from("pea"), String::from("eat"), String::from("rain")];
        let target = vec![String::from("oath"), String::from("eat")];
        assert_eq!(find_words(board, words), target);
    }
}
