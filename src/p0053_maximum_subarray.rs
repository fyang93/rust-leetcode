pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = i32::min_value();
    let mut last_max = i32::min_value();
    for num in nums {
        last_max = last_max.max(0) + num;
        max = max.max(last_max);
    }
    max
}

// divide and conquer
pub fn max_sub_array_1(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());
    sub(&nums, 0, nums.len() - 1)
}

fn sub(nums: &Vec<i32>, left: usize, right: usize) -> i32 {
    if left == right {
        return nums[left];
    }
    let mid = (right - left) / 2 + left;
    let left_max = sub(nums, left, mid);
    let right_max = sub(nums, mid + 1, right);
    let mut sum_to_left = i32::min_value();
    (left..=mid).rev().fold(0, |sum, i| {
        let sum = sum + nums[i];
        sum_to_left = sum_to_left.max(sum);
        sum
    });
    let mut sum_to_right = i32::min_value();
    (mid+1..=right).fold(0, |sum, i| {
        let sum = sum + nums[i];
        sum_to_right = sum_to_right.max(sum);
        sum
    });
    left_max.max(right_max).max(sum_to_left + sum_to_right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums.clone()), 6);
        assert_eq!(max_sub_array_1(nums.clone()), 6);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![-2, -1];
        assert_eq!(max_sub_array(nums.clone()), -1);
        assert_eq!(max_sub_array_1(nums.clone()), -1);
    }
}
