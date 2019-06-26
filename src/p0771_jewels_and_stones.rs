use std::collections::HashSet;

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let jewels: HashSet<char> = j.chars().collect();
    if jewels.is_empty() { 0 } else { s.chars().filter(|c| jewels.contains(c)).count() as i32 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let j = String::from("aA");
        let s = String::from("aAAbbbb");
        assert_eq!(num_jewels_in_stones(j, s), 3)
    }

    #[test]
    fn it_works_1() {
        let j = String::from("z");
        let s = String::from("ZZ");
        assert_eq!(num_jewels_in_stones(j, s), 0)
    }
}
