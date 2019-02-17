pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() { return vec![-1,-1]; }
    let mut range = vec![];

    let (mut left, mut right)  = (0, nums.len()-1);
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    range.push(if nums[left] == target { left as i32 } else { -1 });
    right = nums.len() - 1;

    while left < right {
        let mid = right - (right - left) / 2;
        if nums[mid] <= target {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    range.push(if nums[right] == target { right as i32 } else { -1 });

    range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![5,7,7,8,8,10];
        let target = 8;
        assert_eq!(search_range(nums, target), vec![3,4]);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![5,7,7,8,8,10];
        let target = 6;
        assert_eq!(search_range(nums, target), vec![-1,-1]);
    }
}
