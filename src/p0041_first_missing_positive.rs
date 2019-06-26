pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[i] != nums[nums[i] as usize - 1] {
            let pos = nums[i] as usize - 1;
            nums.swap(i, pos);
        }
    }

    for i in 0..nums.len() {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }

    nums.len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(first_missing_positive(vec![1,1]), 2);
        assert_eq!(first_missing_positive(vec![1,2,0]), 3);
        assert_eq!(first_missing_positive(vec![3,4,-1,1]), 2);
        assert_eq!(first_missing_positive(vec![7,8,9,11,12]), 1);
    }
}
