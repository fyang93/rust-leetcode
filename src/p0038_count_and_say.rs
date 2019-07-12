pub fn count_and_say(n: i32) -> String {
    use std::fmt::Write;

    if n == 0 {
        return String::new();
    };

    let mut s = String::from("1");
    for _ in 1..n {
        let mut tmp = String::new();
        let chars: Vec<_> = s.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            let next = (i + 1..)
                .find(|&j| chars.get(j) != Some(&chars[i]))
                .unwrap();
            write!(&mut tmp, "{}{}", next - i, chars[i]).unwrap();
            i = next;
        }

        s = tmp;
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(count_and_say(4), String::from("1211"));
    }
}
