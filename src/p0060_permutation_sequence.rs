// factorials 0! to 9!
const factorials: [i32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

pub fn get_permutation(n: i32, k: i32) -> String {
    let mut n = n as usize;
    let mut k = k - 1;  // index starts from 0 instead of 1
    let mut result: Vec<char> = vec![];
    let mut chars: Vec<char> = (b'1'..=b'9').take(n).map(char::from).collect();
    while n > 0 {
        let i = (k / factorials[n - 1]) as usize;
        result.push(chars[i]);
        k %= factorials[n - 1];
        chars.remove(i);
        n -= 1;
    }
    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_permutation(3, 3), "213");
    }

    #[test]
    fn it_works_1() {
        assert_eq!(get_permutation(4, 9), "2314");
    }
}
