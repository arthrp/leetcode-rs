pub struct Solution;

/*
Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. 
You may return the combinations in any order.
The same number may be chosen from candidates an unlimited number of times. 
Two combinations are unique if the frequency of at least one of the chosen numbers is different.
 */

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // Backtracking (DFS) over candidates.
        //
        // Key ideas:
        // - Sort candidates so we can stop early when `v > remaining`.
        // - Only iterate from `start..` so we generate combinations in non-decreasing
        //   candidate-index order, which avoids duplicates like [2,3,2] vs [2,2,3].
        // - To allow unlimited reuse of a number, we recurse with the same index `i`.
        if target < 0 || candidates.is_empty() {
            return Vec::new();
        }

        let mut candidates = candidates;
        candidates.sort_unstable();

        fn dfs(
            candidates: &[i32],
            start: usize,
            remaining: i32,
            current: &mut Vec<i32>,
            out: &mut Vec<Vec<i32>>,
        ) {
            // Found an exact sum; record the current combination.
            if remaining == 0 {
                out.push(current.clone());
                return;
            }
            if remaining < 0 {
                return;
            }

            for i in start..candidates.len() {
                let v = candidates[i];
                // Because candidates is sorted, any later value will also be too large.
                if v > remaining {
                    break;
                }
                current.push(v);
                // Allow reuse of same element by passing `i`.
                dfs(candidates, i, remaining - v, current, out);
                current.pop();
            }
        }

        let mut out = Vec::new();
        let mut current = Vec::new();
        // Start from index 0 with full target remaining.
        dfs(&candidates, 0, target, &mut current, &mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn sort_nested(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut v {
            row.sort_unstable();
        }
        v.sort();
        v
    }

    #[test]
    fn test_combination_sum_example_1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let result = sort_nested(Solution::combination_sum(candidates, target));
        let expected = sort_nested(vec![vec![2, 2, 3], vec![7]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum_example_2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let result = sort_nested(Solution::combination_sum(candidates, target));
        let expected = sort_nested(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum_example_3() {
        let candidates = vec![2];
        let target = 1;
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }
}