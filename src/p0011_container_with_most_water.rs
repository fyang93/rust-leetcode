pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut i, mut j) = (0, height.len() - 1);
    let mut water = 0;

    while i < j {
        let h = height[i].min(height[j]);
        water = water.max(h * (j - i) as i32);
        while height[i] <= h && i < j {
            i += 1;
        }
        while height[j] <= h && i < j {
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
