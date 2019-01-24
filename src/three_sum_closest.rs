pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();
    let mut nums = nums;
    nums.sort();
    let mut min_diff = i32::max_value();

    for left in 0..(len-2) {
        if left == 0 || (left > 0 && nums[left] != nums[left-1]) {
            let (mut mid, mut right) = (left+1, len-1);
            while mid < right {
                let diff = nums[left] + nums[mid] + nums[right] - target;
                if diff.abs() < min_diff.abs() {
                    min_diff = diff;
                }
                if diff < 0 {
                    while mid < right && nums[mid] == nums[mid+1] {
                        mid += 1;
                    }
                    mid += 1;
                } else if diff > 0 {
                    while mid < right && nums[right-1] == nums[right] {
                        right -= 1;
                    }
                    right -= 1;
                } else {
                    return target;
                }
            }
        }
    }
    target + min_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1,2,1,-4];
        assert_eq!(three_sum_closest(nums, 1), 2);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![-1,0,1,1,55];
        assert_eq!(three_sum_closest(nums, 3), 2);
    }
}
