// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut length = 0;
        
        // First pass: Calculate length
        {
            let mut curr = &head;
            while let Some(node) = curr {
                length += 1;
                curr = &node.next;
            }
        }

        let target = length - n;
        
        // If we need to remove the head
        if target == 0 {
            return head.and_then(|node| node.next);
        }

        // Second pass: Traverse to the node before the one to remove
        let mut curr = &mut head;
        for _ in 0..target - 1 {
            if let Some(node) = curr {
                curr = &mut node.next;
            }
        }

        // Remove the node
        if let Some(node) = curr {
            node.next = node.next.take().and_then(|next_node| next_node.next);
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        // Create list 1->2->3->4->5
        let mut head = Some(Box::new(ListNode::new(1)));
        let mut curr = &mut head;
        for i in 2..=5 {
            if let Some(node) = curr {
                node.next = Some(Box::new(ListNode::new(i)));
                curr = &mut node.next;
            }
        }

        // Remove 2nd from end (node with val 4)
        let result = Solution::remove_nth_from_end(head, 2);

        // Verify result: 1->2->3->5
        let mut curr = &result;
        let expected = vec![1, 2, 3, 5];
        for val in expected {
            assert_eq!(curr.as_ref().unwrap().val, val);
            curr = &curr.as_ref().unwrap().next;
        }
        assert!(curr.is_none());
    }

    #[test]
    fn test_remove_head() {
        // Create list 1->2
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));

        // Remove 2nd from end (node with val 1)
        let result = Solution::remove_nth_from_end(head, 2);

        // Verify result: 2
        let mut curr = &result;
        let expected = vec![2];
        for val in expected {
            assert_eq!(curr.as_ref().unwrap().val, val);
            curr = &curr.as_ref().unwrap().next;
        }
        assert!(curr.is_none());
    }

    #[test]
    fn test_remove_single() {
        // Create list 1
        let head = Some(Box::new(ListNode::new(1)));

        // Remove 1st from end
        let result = Solution::remove_nth_from_end(head, 1);

        // Verify result: empty
        assert!(result.is_none());
    }
}
