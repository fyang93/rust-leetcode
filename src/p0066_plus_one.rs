pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;

    for digit in digits.iter_mut().rev() {
        if *digit == 9 {
            *digit = 0;
        } else {
            *digit += 1;
            return digits;
        }
    }
    digits.insert(0, 1);
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![1, 2, 3];
        let target = vec![1, 2, 4];
        assert_eq!(plus_one(input), target);
    }

    #[test]
    fn it_works_1() {
        let input = vec![9, 9];
        let target = vec![1, 0, 0];
        assert_eq!(plus_one(input), target);
    }
}
