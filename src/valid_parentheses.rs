pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            },
            ')' => {
                if Some('(') != stack.pop() {
                    return false;
                }
            },
            ']' => {
                if Some('[') != stack.pop() {
                    return false;
                }
            }
            '}' => {
                if Some('{') != stack.pop() {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_valid(String::from("(){}[]")), true);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(is_valid(String::from("([)]")), false);
    }
}
