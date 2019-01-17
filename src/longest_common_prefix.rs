pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let n = strs.len();
    match n {
        0 => return String::new(),
        1 => return strs[0].clone(),
        _ => ()
    }

    let mut len = 0;
    let mut iters: Vec<_> = strs.iter().map(|s| s.chars()).collect();

    while let Some(a) = iters[0].next() {
        for i in 1..n {
            match iters[i].next() {
                Some(b) if a == b => continue,
                _ => return String::from(&strs[0][..len]),
            }
        }
        len += 1;
    }
    String::from(&strs[0][..len])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
        assert_eq!(longest_common_prefix(strs), String::from("fl"));
    }

    #[test]
    fn it_works_1() {
        let strs = vec![String::from("dog"), String::from("racecar"), String::from("car")];
        assert_eq!(longest_common_prefix(strs), String::new());
    }

    #[test]
    fn it_works_2() {
        let strs = vec![String::from("aa"), String::from("a"), String::from("a")];
        assert_eq!(longest_common_prefix(strs), String::from("a"));
    }
}
