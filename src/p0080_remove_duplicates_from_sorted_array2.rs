pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 { return nums.len() as i32; }
    let (mut l1, mut l2) = (0, 1);
    for r in 2..nums.len() {
        if nums[l1] == nums[l2] && nums[l2] == nums[r] { continue; }
        l1 += 1;
        l2 += 1;
        nums[l2] = nums[r];
    }
    (l2 + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2, 2, 3]), 5);
        assert_eq!(remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]), 7);
    }
}
