pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            match mid.checked_sub(1) {
                Some(v) => right = v,
                _ => break
            }
        }
    }

    left as i32
}

// cheat
pub fn search_insert_1(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(v) => v as i32,
        Err(v) => v as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert(nums, target), 2);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert(nums, target), 1);
    }

    #[test]
    fn it_works_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert(nums, target), 4);
    }

    #[test]
    fn it_works_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        assert_eq!(search_insert(nums, target), 0);
    }
}
