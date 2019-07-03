pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
        return s;
    }

    let mut rows = vec![String::new(); num_rows as usize];
    let mut row: i32 = 0;
    let mut step = 1;

    for c in s.chars() {
        rows[row as usize].push(c);
        row += step;
        if row == 0 || row == num_rows - 1 {
            step *= -1;
        }
    }

    rows.concat()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = convert(String::from("PAYPALISHIRING"), 3);
        assert_eq!(res, String::from("PAHNAPLSIIGYIR"));
    }

    #[test]
    fn it_works_1() {
        let res = convert(String::from("PAYPALISHIRING"), 4);
        assert_eq!(res, String::from("PINALSIGYAHRPI"));
    }

    #[test]
    fn it_works_2() {
        let res = convert(String::from("AB"), 1);
        assert_eq!(res, String::from("AB"));
    }
}
