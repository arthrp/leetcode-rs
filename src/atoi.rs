pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s_iter = s.trim_start().chars().peekable();

        let mul_sign: i64 = match s_iter.peek() {
            Some('-') => {
                s_iter.next();
                -1
            }
            Some('+') => {
                s_iter.next();
                1
            }
            _ => 1,
        };

        let mut num: i64 = 0;
        for c in s_iter {
            if let Some(digit) = c.to_digit(10) {
                num = num.saturating_mul(10).saturating_add(digit as i64);
            } else {
                break;
            }
        }

        let final_num = num.saturating_mul(mul_sign);

        final_num.clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoi() {
        assert_eq!(Solution::my_atoi(" -042".to_string()), -42);
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
        assert_eq!(Solution::my_atoi("+123".to_string()), 123);
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}