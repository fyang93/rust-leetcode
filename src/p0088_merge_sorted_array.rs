pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if n == 0 {
        return;
    }
    if m == 0 {
        std::mem::swap(nums1, nums2);
        return;
    }

    let (mut i, mut j) = ((m - 1) as usize, (n - 1) as usize);
    for k in (0..nums1.len()).rev() {
        if nums1[i] > nums2[j] {
            nums1[k] = nums1[i];
            if i == 0 {
                for k in (0..=j).rev() {
                    nums1[k] = nums2[k];
                }
                break;
            }
            i -= 1;
        } else {
            nums1[k] = nums2[j];
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums1 = vec![2, 0];
        let mut nums2 = vec![1];
        merge(&mut nums1, 1, &mut nums2, 1);
        assert_eq!(nums1, vec![1, 2]);
    }
}
