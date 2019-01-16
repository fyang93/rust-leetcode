pub fn int_to_roman(num: i32) -> String {
    let mut num = num;
    let mut chars: Vec<char> = Vec::new();
    if num >= 1000 {
        for _ in 0..(num / 1000) {
            chars.push('M');
        }
        num %= 1000;
    }
    if num >= 100 {
        update_chars(num / 100, &mut chars, 'M', 'D', 'C');
        num %= 100;
    }
    if num >= 10 {
        update_chars(num / 10, &mut chars, 'C', 'L', 'X');
        num %= 10;
    }
    if num > 0 {
        update_chars(num, &mut chars, 'X', 'V' ,'I');
    }
    chars.iter().collect()
}

pub fn update_chars(n: i32, chars: &mut Vec<char>, high: char, mid: char, low: char) {
    match n {
        1 | 2 | 3 => {
            for _ in  0..n {
                chars.push(low);
            }
        },
        4 => {
            chars.push(low);
            chars.push(mid);
        },
        5 | 6 | 7 | 8 => {
            chars.push(mid);
            for _ in 5..n {
                chars.push(low);
            }
        },
        9 => {
            chars.push(low);
            chars.push(high);
        },
        _ => ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(int_to_roman(3), String::from("III"));
    }

    #[test]
    fn it_works_1() {
        assert_eq!(int_to_roman(4), String::from("IV"));
    }

    #[test]
    fn it_works_2() {
        assert_eq!(int_to_roman(58), String::from("LVIII"));
    }

    #[test]
    fn it_works_3() {
        assert_eq!(int_to_roman(1994), String::from("MCMXCIV"));
    }
}
