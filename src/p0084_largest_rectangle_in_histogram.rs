pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
    heights.push(0);
    let mut i = 0;
    let mut area = 0;
    // 可能产生新的最大面积的高度索引，满足 heights[stack[i]] < heights[stack[j]] if i < j
    let mut stack = vec![];

    while i < heights.len() {
        match stack.last() {
            // 出现高度下降时，计算此前可能出现的最大面积
            Some(&j) if heights[i] < heights[j] => {
                let h = heights[stack.pop().unwrap()];
                // 当stack为空的时候说明当前高度为新的最小高度
                let w = stack.last().map_or(i, |&k| i - k - 1) as i32;
                area = area.max(h * w);
            }
            // 当前高度大于stack中最后一个索引对应的高度，则可能产生新的最大面积
            _ => {
                if stack.last().map_or(false, |&j| heights[i] == heights[j]) {
                    stack.pop();
                }
                stack.push(i);
                i += 1;
            }
        }
    }
    heights.pop();
    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(largest_rectangle_area(vec![0, 0, 0, 0, 0, 0, 0, 0, 2147483647]), 2147483647);
        assert_eq!(largest_rectangle_area(vec![0, 1, 0, 1]), 1);
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_area(vec![2, 2, 2]), 6);
    }
}
