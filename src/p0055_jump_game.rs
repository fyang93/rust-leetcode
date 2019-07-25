#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Index {
    Good,
    Bad,
    Unknown,
}

// greedy
// 能抵达最左的Good就能抵达列尾
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut leftmost = nums.len() - 1;
    for i in (0..nums.len() - 1).rev() {
        if i + nums[i] as usize >= leftmost {
            leftmost = i;
        }
    }
    leftmost == 0
}

// dynamic programming bottom-up
pub fn can_jump_0(nums: Vec<i32>) -> bool {
    let mut memo = vec![Index::Unknown; nums.len()];
    memo.last_mut().map(|x| *x = Index::Good);

    for i in (0..nums.len() - 1).rev() {
        let furthest = (i + nums[i] as usize).min(nums.len() - 1);
        for j in i + 1..=furthest {
            if memo[j] == Index::Good {
                memo[i] = Index::Good;
                break;
            }
        }
    }

    memo[0] == Index::Good
}

// dynamic programming top-down
pub fn can_jump_1(nums: Vec<i32>) -> bool {
    let mut memo = vec![Index::Unknown; nums.len()];
    memo.last_mut().map(|x| *x = Index::Good);
    can_jump_from_position_1(0, &nums, &mut memo)
}

fn can_jump_from_position_1(position: usize, nums: &Vec<i32>, memo: &mut Vec<Index>) -> bool {
    match memo[position] {
        Index::Good => return true,
        Index::Bad => return false,
        _ => {
            let furthest = (position + nums[position] as usize).min(nums.len() - 1);
            for i in (position + 1..=furthest).rev() {
                if can_jump_from_position_1(i, nums, memo) {
                    memo[position] = Index::Good;
                    return true;
                }
            }
            false
        }
    }
}

// backtracking
pub fn can_jump_2(nums: Vec<i32>) -> bool {
    can_jump_from_position_2(0, &nums)
}

fn can_jump_from_position_2(position: usize, nums: &Vec<i32>) -> bool {
    if position + 1 == nums.len() {
        return true;
    }

    let furthest = (position + nums[position] as usize).min(nums.len() - 1);
    // 若下一步可到的任何位置能抵达列尾则当前位置也可抵达列尾
    for i in (position + 1..=furthest).rev() {
        if can_jump_from_position_2(i, nums) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(can_jump(nums.clone()), true);
        assert_eq!(can_jump_0(nums.clone()), true);
        assert_eq!(can_jump_1(nums.clone()), true);
        assert_eq!(can_jump_2(nums.clone()), true);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(can_jump(nums.clone()), false);
        assert_eq!(can_jump_0(nums.clone()), false);
        assert_eq!(can_jump_1(nums.clone()), false);
        assert_eq!(can_jump_2(nums.clone()), false);
    }
}
