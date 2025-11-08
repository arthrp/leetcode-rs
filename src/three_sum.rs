pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        nums.sort_unstable();

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let current_sum = nums[i] + nums[left] + nums[right];
                match current_sum.cmp(&0) {
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;

                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
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
    fn test_three_sum_example() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = Solution::three_sum(nums);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        result.sort();

        assert_eq!(result, expected);
    }
}