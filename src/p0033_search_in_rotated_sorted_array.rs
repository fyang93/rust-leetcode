pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() { return -1; }
    search_recur(&nums, &target, 0, nums.len() - 1)
}

fn search_recur(nums: &Vec<i32>, target: &i32, left: usize, right: usize) -> i32 {
    if right - left <= 1 {
        if &nums[left] == target {
            return left as i32;
        } else if &nums[right] == target {
            return right as i32;
        } else {
            return -1;
        }
    }

    let mid = left + (right - left) / 2;
    if &nums[mid] == target {
        return mid as i32;
    }
    if (&nums[left] <= target && target < &nums[mid]) || (&nums[mid] <= &nums[right] && (target < &nums[mid] || &nums[right] < target)) {
        return search_recur(nums, target, left, mid - 1);
    } else {
        return search_recur(nums, target, mid + 1, right);
    }
}

pub fn search_1(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() { return -1; }

    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] >= nums[left] {
            if target >= nums[left] && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if target > nums[mid] && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(search(nums, target), 4);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(search(nums, target), -1);
    }

    #[test]
    fn it_works_2() {
        let nums = vec![1, 3];
        let target = 0;
        assert_eq!(search(nums, target), -1);
    }
}
