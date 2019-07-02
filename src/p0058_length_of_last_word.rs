pub fn length_of_last_word(s: String) -> i32 {
    s.trim_end().rsplitn(2, ' ').next().map_or(0, |w| w.len() as i32)
}

pub fn length_of_last_word_1(s: String) -> i32 {
    s.split_whitespace().last().map_or(0, |w| w.len() as i32)
}

pub fn length_of_last_word_2(s: String) -> i32 {
    s.chars().rev().skip_while(|&c| c == ' ').take_while(|&c| c != ' ').count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = String::from("Hello World");
        assert_eq!(length_of_last_word(input), 5);
    }

    #[test]
    fn it_works_1() {
        let input = String::from("  ");
        assert_eq!(length_of_last_word(input), 0);
    }
}
