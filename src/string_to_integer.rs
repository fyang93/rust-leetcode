pub fn my_atoi(s: String) -> i32 {
    let (i32_min, i32_max) = (i32::min_value(), i32::max_value());
    let mut in_num = false;
    let mut res: i32 = 0;
    let mut sign: i32 = 1;
    let mut overflow = false;

    let c_iter = s.chars();
    for c in s.chars() {
        if !in_num {
            match c {
                ' ' => continue,
                '0'..='9' => {
                    in_num = true;
                    res = res * 10 + c.to_digit(10).unwrap() as i32;
                },
                '-' => {
                    in_num = true;
                    sign = -1;
                },
                '+' => {
                    in_num = true;
                },
                _ => return 0,
            }
        } else {
            match c {
                '0'..='9' => {
                    match res.checked_mul(10) {
                        Some(v) => match v.checked_add(c.to_digit(10).unwrap() as i32) {
                            Some(x) => res = x,
                            None => {
                                overflow = true;
                                break;
                            }
                        }
                        None => {
                            overflow = true;
                            break;
                        }
                    }
                },
                _ => break,
            }
        }
    }

    if overflow {
        if sign == 1 {
            i32_max
        } else {
            i32_min
        }
    } else {
        sign * res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(my_atoi(String::from("9223372036854775808")), 2147483647);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(my_atoi(String::from("words and 987")), 0);
    }

    #[test]
    fn it_works_2() {
        assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
    }
}
