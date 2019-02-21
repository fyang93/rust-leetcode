pub fn multiply(num1: String, num2: String) -> String {
    if &num1 == "0" || &num2 == "0" {
        return String::from("0");
    }

    let num1: Vec<_> = num1.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let num2: Vec<_> = num2.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut nums = vec![0; num1.len() + num2.len()];

    for i in (0..num1.len()).rev() {
        for j in (0..num2.len()).rev() {
            let p1 = i + j;
            let p2 = p1 + 1;
            let sum = num1[i] * num2[j] + nums[p2];
            // nums[p1] 可能 >=10，但 nums[p2] 永远 <10
            nums[p1] += sum / 10;
            nums[p2] = sum % 10;
        }
    }

    nums.into_iter()
        .skip_while(|&x| x == 0)
        .map(|d| std::char::from_digit(d, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            multiply(String::from("2"), String::from("3")),
            String::from("6")
        );
    }

    #[test]
    fn it_works_1() {
        assert_eq!(
            multiply(String::from("123"), String::from("456")),
            String::from("56088")
        );
    }
}
