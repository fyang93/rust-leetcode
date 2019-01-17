pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    if len < 3 { return vec![]; }

    let mut nums = nums;
    nums.sort();

    let mut results: Vec<Vec<i32>> = Vec::new();
    for left in 0..(len-2) {
        if left == 0 || (left > 0 && nums[left] != nums[left-1]) {
            let target = -nums[left];
            let (mut mid, mut right) = (left+1, len-1);
            while mid < right {
                let sum = nums[mid] + nums[right];
                if sum < target {
                    mid += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    results.push(vec![nums[left], nums[mid], nums[right]]);
                    while mid < right && nums[mid] == nums[mid+1] {
                        mid += 1;
                    }
                    while mid < right && nums[right-1] == nums[right] {
                        right -= 1;
                    }
                    mid += 1;
                    right -= 1;
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ans = vec![vec![-1,-1,2], vec![-1,0,1]];
        assert_eq!(three_sum(nums), ans);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![0,0,0];
        let ans = vec![vec![0,0,0]];
        assert_eq!(three_sum(nums), ans);
    }
}
