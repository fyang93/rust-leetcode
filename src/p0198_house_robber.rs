pub fn rob(nums: Vec<i32>) -> i32 {
    nums.iter().fold((0, 0), |(last_robbed, last_skipped), this| (last_robbed.max(last_skipped + this), last_robbed)).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rob(vec![]), 0);
        assert_eq!(rob(vec![1]), 1);
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }
}
