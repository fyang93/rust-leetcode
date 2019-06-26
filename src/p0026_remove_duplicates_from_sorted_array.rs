pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() { return 0; }
    let mut i = 0;
    for j in 1..nums.len() {
        if nums[i] != nums[j] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    (i + 1) as i32
}

// cheat
pub fn remove_duplicates_1(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(remove_duplicates(&mut vec![1,1,2]), 2);
    }
}
