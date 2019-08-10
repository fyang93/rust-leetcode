pub fn majority_element(nums: Vec<i32>) -> i32 {
    let (mut major, mut count) = (0, 0);
    for num in nums {
        if count == 0 {
            major = num;
            count += 1;
        } else {
            count += if major == num { 1 } else { -1 };
        }
    }
    major
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
