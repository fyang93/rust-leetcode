pub fn num_trees(n: i32) -> i32 {
    let mut nums = vec![1; n as usize + 1];

    for i in 2..=n as usize {
        nums[i] = (0..i).fold(0, |sum, j| sum + nums[j] * nums[i - j - 1]);
    }

    nums[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(num_trees(3), 5);
    }
}
