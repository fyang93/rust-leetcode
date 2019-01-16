use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut i, mut j) = (0, height.len() - 1);
    let mut water = 0;

    while i < j {
        water = cmp::max(water, cmp::min(height[i], height[j]) * (j - i) as i32);
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(max_area(input), 49);
    }
}
