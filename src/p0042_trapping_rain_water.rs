// 在最高柱左侧的积水由从左向右找到的最高处决定
// 在最高柱右侧的积水由从右往左找到的最高处决定
pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut ans = 0;
    let (mut left, mut right) = (0, height.len() - 1);
    let (mut left_max, mut right_max) = (0, 0);

    while left < right {
        if height[left] < height[right] {
            if height[left] < left_max {
                ans += left_max - height[left];
            } else {
                left_max = height[left];
            }
            left += 1;
        } else {
            if height[right] < right_max {
                ans += right_max - height[right];
            } else {
                right_max = height[right];
            }
            right -= 1;
        }
    }

    ans
}

// 纵向累加积水
pub fn trap_dp(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let mut ans = 0;
    let (mut left_max, mut right_max) = (height.clone(), height.clone());

    for i in 1..height.len() {
        left_max[i] = left_max[i - 1].max(height[i]);
    }
    for i in (0..height.len() - 1).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }
    for i in 1..height.len() - 1 {
        ans += left_max[i].min(right_max[i] - height[i]);
    }

    ans
}

// 横向累加积水（好复杂=.=
pub fn trap_stack(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stack = vec![];

    for i in 0..height.len() {
        while !stack.is_empty() && height[i] > height[stack.last().cloned().unwrap()] {
            let top = stack.pop().unwrap();
            if let Some(&j) = stack.last() {
                let dist = i - j - 1;
                let bounded_height = height[i].min(height[j]) - height[top];
                ans += dist as i32 * bounded_height;
            }
        }
        stack.push(i);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap_dp(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(trap_stack(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
