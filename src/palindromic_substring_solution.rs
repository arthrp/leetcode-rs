pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let s_bytes = s.as_bytes();
        let len = s.len();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..len {
            let len1 = Self::expand_around_center(s_bytes, i, i);
            let len2 = Self::expand_around_center(s_bytes, i, i + 1);
            
            let current_max = len1.max(len2);
            if current_max > max_len {
                max_len = current_max;
                start = i - (max_len - 1) / 2;
            }
        }

        if max_len == 0 && len > 0 {
            return s[0..1].to_string();
        }

        s[start..start + max_len].to_string()
    }

    fn expand_around_center(s: &[u8], left: usize, right: usize) -> usize {
        let mut l = left as i32;
        let mut r = right as i32;

        while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        (r - l - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        // Test case 1: Basic case with a clear longest palindrome
        let s1 = String::from("babad");
        let result1 = Solution::longest_palindrome(s1);
        assert!(result1 == "bab" || result1 == "aba");

        // Test case 2: Another common case
        let s2 = String::from("cbbd");
        assert_eq!(Solution::longest_palindrome(s2), "bb");

        // Test case 3: String with all same characters
        let s3 = String::from("aaaa");
        assert_eq!(Solution::longest_palindrome(s3), "aaaa");

        // Test case 4: Single character string
        let s4 = String::from("a");
        assert_eq!(Solution::longest_palindrome(s4), "a");

        // Test case 5: Empty string
        let s5 = String::from("");
        assert_eq!(Solution::longest_palindrome(s5), "");

        // Test case 6: String with no palindrome longer than 1 char
        let s6 = String::from("ac");
        assert_eq!(Solution::longest_palindrome(s6), "a");
    }
}