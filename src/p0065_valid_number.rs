// cheat
pub fn is_number(s: String) -> bool {
    s.trim().parse::<f32>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_number(String::from("  -90e3  ")), true);
    }
}
