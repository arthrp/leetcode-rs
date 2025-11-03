pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
        let mut current_row = 0;
        let mut direction = -1;

        for char in s.chars() {
            rows[current_row as usize].push(char);
            if current_row == 0 || current_row == num_rows - 1 {
                direction *= -1;
            }
            current_row += direction;
        }

        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_conversion() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let expected = "PAHNAPLSIIGYIR".to_string();
        let result = Solution::convert(s, num_rows);
        assert_eq!(result, expected);
    }
}