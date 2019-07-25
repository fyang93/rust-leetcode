pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let n = s.len();
    let mut ips = vec![];
    for i in 1.max(n.saturating_sub(9))..=3.min(n.saturating_sub(3)) {
        if is_valid(&s[..i]) {
            for j in (i + 1).max(n.saturating_sub(6))..=(i + 3).min(n.saturating_sub(2)) {
                if is_valid(&s[i..j]) {
                    for k in (j + 1).max(n.saturating_sub(3))..=(j + 3).min(n.saturating_sub(1)) {
                        if is_valid(&s[j..k]) && is_valid(&s[k..]) {
                            ips.push(format!("{}.{}.{}.{}", &s[..i], &s[i..j], &s[j..k], &s[k..]));
                        }
                    }
                }
            }
        }
    }
    ips
}

fn is_valid(s: &str) -> bool {
    if s.starts_with("0") && s != "0" {
        false
    } else {
        match s.parse::<i32>() {
            Ok(x) => x <= 255,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            restore_ip_addresses(String::from("25525511135")),
            vec![
                String::from("255.255.11.135"),
                String::from("255.255.111.35")
            ]
        );
    }
}
