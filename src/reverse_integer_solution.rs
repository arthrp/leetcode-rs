pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev: i32 = 0;

        while x != 0 {
            let pop = x % 10;
            x /= 10;

            //Overflow
            if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) {
                return 0;
            }
            //Underflow
            if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) {
                return 0;
            }

            rev = rev * 10 + pop;
        }
        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
    }
}