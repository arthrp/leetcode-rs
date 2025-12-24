use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        if words.is_empty() || s.is_empty() {
            return Vec::new();
        }

        let word_len = words[0].len();
        let num_words = words.len();
        let total_concat_len = word_len * num_words;

        //If string is shorter than the length of all concatenated words, it cannot possibly contain them
        if s.len() < total_concat_len {
            return Vec::new();
        }

        // Frequency map of the required words (owned keys)
        let mut need: HashMap<String, i32> = HashMap::new();
        for w in &words {
            *need.entry(w.clone()).or_insert(0) += 1;
        }

        let s_bytes = s.as_bytes();

        // Try each offset within a word-length window (0..word_len)
        for offset in 0..word_len {
            let mut left = offset;
            let mut count = 0usize;
            let mut window: HashMap<&str, i32> = HashMap::new();
            let mut i = offset;

            while i + word_len <= s.len() {
                let w = std::str::from_utf8(&s_bytes[i..i + word_len]).unwrap();

                if need.contains_key(w) {
                    *window.entry(w).or_insert(0) += 1;
                    count += 1;

                    // If this word's count exceeds needed, shrink from left
                    let needed = *need.get(w).unwrap();
                    while window.get(w).unwrap() > &needed {
                        let left_w = std::str::from_utf8(&s_bytes[left..left + word_len]).unwrap();
                        if let Some(e) = window.get_mut(left_w) {
                            *e -= 1;
                        }
                        left += word_len;
                        count -= 1;
                    }

                    // If window has all words, record start index
                    if count == num_words {
                        res.push(left as i32);
                        // Move left by one word to continue searching for next possible index
                        let left_w = std::str::from_utf8(&s_bytes[left..left + word_len]).unwrap();
                        if let Some(e) = window.get_mut(left_w) {
                            *e -= 1;
                        }
                        left += word_len;
                        count -= 1;
                    }
                } else {
                    // word not in need: reset window
                    window.clear();
                    count = 0;
                    left = i + word_len;
                }

                i += word_len;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_works(){
        let res = Solution::find_substring("barfoothefoobarman".to_string(), vec!["foo".to_string(),"bar".to_string()]);
        assert_eq!(res, vec![0, 9])
    }
}
