pub struct Solution;

impl Solution {
    /// Returns all unique quadruplets in `nums` that sum up to `target`.
    /// 
    /// The algorithm uses a sorted array and two nested loops combined with a two-pointer approach.
    /// Complexity: O(n^3) time, O(1) extra space (excluding result storage).
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let len = nums.len();
        
        // A quadruplet requires at least 4 elements
        if len < 4 {
            return result;
        }

        // Sort the array to use two-pointer technique and skip duplicates easily
        nums.sort_unstable();
        
        // Use i64 for target and sum calculations to prevent integer overflow
        let target = target as i64;

        for i in 0..len - 3 {
            // Skip the same value for the first element to avoid duplicate quadruplets
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            // Optimization: If the smallest possible sum starting with nums[i] is greater than target,
            // no need to continue since the array is sorted.
            if (nums[i] as i64 + nums[i + 1] as i64 + nums[i + 2] as i64 + nums[i + 3] as i64) > target {
                break;
            }
            
            // Optimization: If the largest possible sum starting with nums[i] is less than target,
            // then nums[i] is too small to be part of any valid quadruplet.
            if (nums[i] as i64 + nums[len - 3] as i64 + nums[len - 2] as i64 + nums[len - 1] as i64) < target {
                continue;
            }

            for j in i + 1..len - 2 {
                // Skip the same value for the second element
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                // Optimization: Check smallest possible sum for current i and j
                if (nums[i] as i64 + nums[j] as i64 + nums[j + 1] as i64 + nums[j + 2] as i64) > target {
                    break;
                }
                
                // Optimization: Check largest possible sum for current i and j
                if (nums[i] as i64 + nums[j] as i64 + nums[len - 2] as i64 + nums[len - 1] as i64) < target {
                    continue;
                }

                // Two-pointer approach for the remaining two elements
                let mut left = j + 1;
                let mut right = len - 1;

                while left < right {
                    let current_sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
                    match current_sum.cmp(&target) {
                        std::cmp::Ordering::Less => {
                            // Sum too small, move left pointer right
                            left += 1;
                        },
                        std::cmp::Ordering::Greater => {
                            // Sum too large, move right pointer left
                            right -= 1;
                        },
                        std::cmp::Ordering::Equal => {
                            // Found a valid quadruplet
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            left += 1;
                            right -= 1;

                            // Skip duplicates for the third element
                            while left < right && nums[left] == nums[left - 1] {
                                left += 1;
                            }
                            // Skip duplicates for the fourth element
                            while left < right && nums[right] == nums[right + 1] {
                                right -= 1;
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_four_sum_example1() {
        let nums = vec![1, 0, -1, 0, -2, 2];
        let target = 0;
        let mut result = Solution::four_sum(nums, target);
        let mut expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
        
        result.iter_mut().for_each(|v| v.sort());
        result.sort();
        expected.iter_mut().for_each(|v| v.sort());
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_four_sum_example2() {
        let nums = vec![2, 2, 2, 2, 2];
        let target = 8;
        let mut result = Solution::four_sum(nums, target);
        let mut expected = vec![vec![2, 2, 2, 2]];

        result.iter_mut().for_each(|v| v.sort());
        result.sort();
        expected.iter_mut().for_each(|v| v.sort());
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_four_sum_overflow() {
        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        let target = -294967296;
        let result = Solution::four_sum(nums, target);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }
}
