pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix = strs[0].clone();

        for str in strs.iter().skip(1) {
            while !str.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    return String::new();
                }
            }
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());

        let strs2 = vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ];
        assert_eq!(Solution::longest_common_prefix(strs2), "".to_string());

        assert_eq!(Solution::longest_common_prefix(vec![]), "".to_string());
    }
}
