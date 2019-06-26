// 把数组整体当作一个十进制大数，将数组元素可组合成的所有大数由小到大排列成一个集合
// 获取下一个数组排列，可以看作寻找当前大数在所有大数集合中的下一个大数
// 例：[1, 3, 8, 4, 7, 6, 5, 5, 1]
pub fn next_permutation(nums: &mut Vec<i32>) {
    // 从后往前找到第一个 nums[i-1] < nums[i] 的位置 (4)
    if let Some(i) = (1..nums.len()).rev().find(|&i| nums[i-1] < nums[i]) {
        // 在 i 后从后往前找第一个大于 nums[i-1] 的数
        let j = (i..nums.len()).rev().find(|&j| nums[i-1] < nums[j]).unwrap();
        // 与 nums[i-1] 互换 (倒数第二个5)
        nums.swap(i - 1, j);
        // 将 i 后的数组倒序
        nums[i..].reverse();
    } else {
        nums.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1,2,3];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1,3,2]);
    }
}
