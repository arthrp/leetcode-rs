struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            // Calculate the area with the current pointers.
            // The width is the distance between the pointers.
            // The height is the minimum of the two lines, as water would spill over the shorter one.
            let width = (right - left) as i32;
            let h = height[left].min(height[right]);
            max_area = max_area.max(width * h);

            // Move the pointers.
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }
}