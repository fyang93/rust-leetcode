pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let k = 4;
    let len = nums.len();
    if len < k {
        return vec![];
    };

    let mut nums = nums;
    nums.sort();

    let mut results: Vec<Vec<i32>> = vec![];
    let mut record = [0; 4];

    k_sum(&nums, &mut results, k as i32, &mut record, 0, target, 0, len - 1);
    results
}

pub fn k_sum(
    nums: &Vec<i32>,
    results: &mut Vec<Vec<i32>>,
    k: i32,
    record: &mut [i32],
    rid: usize,
    target: i32,
    left: usize,
    right: usize,
) {
    let (mut left, mut right) = (left, right);
    if k == 2 {
        while left < right {
            let sum = nums[left] + nums[right];
            if sum < target {
                left += 1;
            } else if sum > target {
                right -= 1;
            } else {
                record[rid] = nums[left];
                record[rid + 1] = nums[right];
                results.push(record.to_vec());
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right - 1] == nums[right] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            }
        }
    } else {
        for i in left..=(right - 2) {
            if i == left || (i > left && nums[i] != nums[i - 1]) {
                let curr = nums[i];
                record[rid] = curr;
                k_sum(nums, results, k - 1, record, rid + 1, target - curr, i + 1, right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-1, 0, 1, 2, 0, -2];
        let ans = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        assert_eq!(four_sum(nums, 0), ans);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![0, 0, 0, 0];
        let ans = vec![vec![0, 0, 0, 0]];
        assert_eq!(four_sum(nums, 0), ans);
    }
}
