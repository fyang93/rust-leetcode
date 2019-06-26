// Tags: Backtracking, DP, Greedy

// greedy
// 使每两步尽量覆盖更大的范围，给后续的跳跃提供更多选择
pub fn jump(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 { return 0; }

    let mut jumps = 1;
    let mut left = 0;
    let mut right = nums[0] as usize;

    while right < nums.len() - 1 {
        let mut next = 0;
        for i in left..=right {
            next = next.max(i + nums[i] as usize);
        }
        jumps += 1;
        left = right;
        right = next;
    }

    jumps
}

// dynamic programming bottom-up
pub fn jump_0(nums: Vec<i32>) -> i32 {
    let mut memo = vec![nums.len() as i32; nums.len()];
    memo.last_mut().map(|x| *x = 0);

    for i in (0..nums.len() - 1).rev() {
        let furthest = (i + nums[i] as usize).min(nums.len() - 1);
        for j in i + 1..=furthest {
            memo[i] = memo[i].min(memo[j] + 1);
        }
    }
    memo[0]
}

// dynamic programming top-down
pub fn jump_1(nums: Vec<i32>) -> i32 {
    let mut memo = vec![nums.len() as i32; nums.len()];
    memo.last_mut().map(|x| *x = 0);
    jump_from_position_1(0, &nums, &mut memo)
}

pub fn jump_from_position_1(position: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
    if memo[position] == nums.len() as i32  {
        let furthest = (position + nums[position] as usize).min(nums.len() - 1);
        for i in (position + 1..=furthest).rev() {
            memo[position] = memo[position].min(jump_from_position_1(i, nums, memo) + 1);
        }
    }
    memo[position]
}

// backtracking
pub fn jump_2(nums: Vec<i32>) -> i32 {
    jump_from_position_2(0, &nums)
}

pub fn jump_from_position_2(position: usize, nums: &Vec<i32>) -> i32 {
    if position + 1 == nums.len() {
        return 0;
    }

    let furthest = (position + nums[position] as usize).min(nums.len() - 1);
    let mut jumps = nums.len() as i32;
    for i in (position + 1..=furthest).rev() {
        jumps = jumps.min(jump_from_position_2(i, nums) + 1);
    }
    jumps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(jump(nums.clone()), 2);
        assert_eq!(jump_0(nums.clone()), 2);
        assert_eq!(jump_1(nums.clone()), 2);
        assert_eq!(jump_2(nums.clone()), 2);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(jump(nums.clone()), 2);
        assert_eq!(jump_0(nums.clone()), 2);
        assert_eq!(jump_1(nums.clone()), 2);
        assert_eq!(jump_2(nums.clone()), 2);
    }
}
