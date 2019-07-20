pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.is_empty() {
        false
    } else {
        search_recur(&nums, &target, 0, nums.len() - 1)
    }
}

fn search_recur(nums: &Vec<i32>, target: &i32, left: usize, right: usize) -> bool {
    if right - left <= 1 {
        return &nums[left] == target || &nums[right] == target;
    }

    let mid = left + (right - left) / 2;
    if &nums[mid] == target {
        true
    } else if &nums[left] <= target && target < &nums[mid] {
        search_recur(nums, target, left, mid - 1)
    } else if &nums[mid] < target && target <= &nums[right] {
        search_recur(nums, target, mid + 1, right)
    } else {
        search_recur(nums, target, left, mid - 1) || search_recur(nums, target, mid + 1, right)
    }
}

pub fn search_1(nums: Vec<i32>, target: i32) -> bool {
    if nums.is_empty() { return false; }

    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return true;
        }
        if nums[left] == nums[mid] && nums[mid] == nums[right] {
            match (left..right).find(|&i| nums[i] != nums[i + 1]) {
                Some(i) => left = i + 1,
                None => return false,
            }
            match (left..right).rev().find(|&i| nums[i] != nums[i + 1]) {
                Some(i) => right = i,
                None => return false,
            }
        } else if nums[left] <= nums[mid] {
            if nums[left] <= target && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        assert_eq!(search_1(nums, 0), true);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![1, 3, 1, 1, 1];
        assert_eq!(search_1(nums, 3), true);
    }

    #[test]
    fn it_works_2() {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        assert_eq!(search_1(nums, 3), false);
    }
}
