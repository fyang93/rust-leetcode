use std::io;
use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let val = nums[i];
        match hash.entry(val) {
            Entry::Occupied(_) => return vec![hash.get(&val).unwrap().clone(), i as i32],
            Entry::Vacant(_) => hash.insert(target - val, i as i32),
        };
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), [1, 2]);
    }

    #[test]
    fn it_works_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), [0, 1]);
    }
}
