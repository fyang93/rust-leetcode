pub fn roman_to_int(s: String) -> i32 {
    if s.is_empty() { return 0; }
    let mut chars = s.chars();
    let mut num = 0;
    let mut cur = parse(chars.next().unwrap());
    for c in chars {
        let next = parse(c);
        num += if cur < next { -cur } else { cur };
        cur = next;
    }
    num + cur
}

fn parse(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = String::from("IX");
        assert_eq!(roman_to_int(input), 9);
    }

    #[test]
    fn it_works_1() {
        let input = String::from("LVIII");
        assert_eq!(roman_to_int(input), 58);
    }
}
