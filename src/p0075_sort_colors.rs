pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut n0, mut n1, mut n2) = (0, 0, 0);
    for i in 0..nums.len() {
        match nums[i] {
            0 => {
                nums[n2] = 2;
                n2 += 1;
                nums[n1] = 1;
                n1 += 1;
                nums[n0] = 0;
                n0 += 1;
            },
            1 => {
                nums[n2] = 2;
                n2 += 1;
                nums[n1] = 1;
                n1 += 1;
            },
            2 => {
                nums[n2] = 2;
                n2 += 1;
            }
            _ => unreachable!(),
        }
    }
}

// 将0和2分别归置到两端，只适用于color数在3以内的情况
pub fn sort_colors_1(nums: &mut Vec<i32>) {
    let (mut left, mut right) = (0, nums.len() - 1);
    let mut i = 0;
    while i <= right {
        if nums[i] == 0 && i != left {
            nums.swap(i, left);
            left += 1;
        } else if nums[i] == 2 && i != right {
            nums.swap(i, right);
            right -= 1;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let target = vec![0, 0, 1, 1, 2, 2];
        sort_colors_1(&mut nums);
        assert_eq!(nums, target);
    }
}
