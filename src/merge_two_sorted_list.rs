// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut l1), Some(mut l2)) => {
                // Compare the values of the heads of the two lists.
                if l1.val <= l2.val {
                    // If l1's value is smaller or equal, it becomes the head of the merged list.
                    // Recursively merge the rest of l1 with l2.
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    // If l2's value is smaller, it becomes the head.
                    // Recursively merge l1 with the rest of l2.
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
            // If one list is empty, the other list is the result.
            (Some(l), None) => Some(l),
            (None, Some(l)) => Some(l),
            // If both lists are empty, the result is an empty list.
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a linked list from a vector.
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut new_node = ListNode::new(val);
            new_node.next = current;
            current = Some(Box::new(new_node));
        }
        current
    }

    #[test]
    fn test_merge_two_lists_complex() {
        let list1 = to_list(vec![1, 2, 4]);
        let list2 = to_list(vec![1, 3, 4]);
        let result = Solution::merge_two_lists(list1, list2);
        let expected = to_list(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(result, expected);
    }
}