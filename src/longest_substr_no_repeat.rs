use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut start: usize = 0;
        let mut last_seen: HashMap<char, usize> = HashMap::new();

        for (index, c) in s.chars().enumerate() {
            if let Some(&last_index) = last_seen.get(&c) {
                if last_index >= start {
                    start = last_index + 1;
                }
            }
            last_seen.insert(c, index);
            max_len = max_len.max(index - start + 1);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("aab".to_string()),
            2
        );
        assert_eq!(
            Solution::length_of_longest_substring("dvdf".to_string()),
            3
        );
    }
}