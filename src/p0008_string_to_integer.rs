pub fn my_atoi(s: String) -> i32 {
    let mut in_num = false;
    let mut res: i32 = 0;
    let mut sign: i32 = 1;
    let get_overflow = |sign| if sign == 1 { i32::max_value() } else { i32::min_value() };

    for c in s.chars() {
        if !in_num {
            match c {
                ' ' => continue,
                '0'...'9' => {
                    in_num = true;
                    res = res * 10 + c.to_digit(10).unwrap() as i32;
                },
                '-' => {
                    in_num = true;
                    sign = -1;
                },
                '+' => in_num = true,
                _ => return 0,
            }
        } else {
            match c {
                '0'...'9' => {
                    match res.checked_mul(10).and_then(|x| x.checked_add(c.to_digit(10).unwrap() as i32)) {
                        Some(x) => res = x,
                        None => return get_overflow(sign),
                    }
                },
                _ => break,
            }
        }
    }

    sign * res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(my_atoi(String::from("9223372036854775808")), 2147483647);
        assert_eq!(my_atoi(String::from("words and 987")), 0);
        assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
    }
}
