use std::collections::HashMap;

pub fn get_hint(secret: String, guess: String) -> String {
    let mut hash_s = HashMap::new();
    let mut hash_g = HashMap::new();
    let mut bulls = 0;
    for (a, b) in secret.chars().zip(guess.chars()) {
        if a == b {
            bulls += 1;
        } else {
            *hash_s.entry(a).or_insert(0) += 1;
            *hash_g.entry(b).or_insert(0) += 1;
        }
    }
    let cows: i32 = hash_s
        .keys()
        .filter(|k| hash_g.contains_key(&k))
        .map(|k| hash_s[&k].min(hash_g[&k]))
        .sum();
    format!("{}A{}B", bulls, cows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let secret = String::from("1807");
        let guess = String::from("7810");
        assert_eq!(get_hint(secret, guess), String::from("1A3B"));
    }

    #[test]
    fn it_works_1() {
        let secret = String::from("1123");
        let guess = String::from("0111");
        assert_eq!(get_hint(secret, guess), String::from("1A1B"));
    }
}
