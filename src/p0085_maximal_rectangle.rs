pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut area = 0;
    let mut hist: Vec<i32> = vec![0; cols];
    for i in 0..rows {
        for j in 0..cols {
            match matrix[i][j] {
                '0' => hist[j] = 0,
                '1' => hist[j] += 1,
                _ => (),
            }
        }
        area = area.max(largest_rectangle_area(&mut hist));
    }

    area
}

fn largest_rectangle_area(heights: &mut Vec<i32>) -> i32 {
    heights.push(0);
    let mut i = 0;
    let mut area = 0;
    let mut stack = vec![];

    while i < heights.len() {
        match stack.last() {
            Some(&j) if heights[i] < heights[j] => {
                let h = heights[stack.pop().unwrap()];
                let w = stack.last().map_or(i, |&k| i - k - 1) as i32;
                area = area.max(h * w);
            }
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
        assert_eq!(
            maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
}
