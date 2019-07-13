pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());
    // 确保num1比较短，这样j一直会取正
    if m > n {
        find_median_sorted_arrays_with_lens(nums2, nums1, n, m)
    } else {
        find_median_sorted_arrays_with_lens(nums1, nums2, m, n)
    }
}

pub fn find_median_sorted_arrays_with_lens(nums1: Vec<i32>, nums2: Vec<i32>, m: usize, n: usize) -> f64 {
    // 短数组: i个元素 | m-i个元素，长数组：j个元素 | n-j个元素
    // +1 表示在 m+n 为奇数时，左半部分多收纳一个元素
    let (mut i, mut j, mut imin, mut imax, half) = (0, 0, 0, m, (m + n + 1) / 2);
    // 用于二分法查找符合条件的i
    while imin <= imax {
        i = (imin + imax) / 2;
        j = half - i;
        // 已满足nums2[j-1]<=nums2[j]
        // 需确保nums2[j-1]<=nums1[i]
        if i < m && nums2[j-1] > nums1[i] {
            imin = i + 1;
        }
        // 已满足nums1[i-1]<=nums1[i]
        // 需确保nums1[i-1]<=nums2[j]
        else if i > 0 && nums1[i-1] > nums2[j] {
            imax = i - 1;
        }
        else {
            let max_of_left = if i == 0 {
                nums2[j-1]
            } else if j == 0 {
                nums1[i-1]
            } else {
                nums1[i-1].max(nums2[j-1])
            };
            // 若n+m为奇数，max_of_left即为中位数，注意左半部分多一个元素
            if (m + n) % 2 == 1 {
                return max_of_left as f64;
            }

            let min_of_right = if i == m {
                nums2[j]
            } else if j == n {
                nums1[i]
            } else {
                nums1[i].min(nums2[j])
            };

            // 若n+m为偶数，结果为左半部分最大值和右半部分最小值的平均
            return (max_of_left + min_of_right) as f64 / 2.0;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn it_works_1() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }
}
