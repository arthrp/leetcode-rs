// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Handle edge case: empty input vector.
        if lists.is_empty() {
            return None;
        }

        let len = lists.len();
        // Use divide-and-conquer to merge lists in pairs iteratively.
        // This reduces the number of lists logarithmically (log k).
        let mut interval = 1;
        while interval < len {
            // Iterate through lists and merge them at the current interval.
            for i in (0..len - interval).step_by(interval * 2) {
                // Merge lists[i] and lists[i + interval] and store the result in lists[i].
                // .take() is used to move the ownership of the Option content out of the vector.
                lists[i] = Self::merge_two_lists(lists[i].take(), lists[i + interval].take());
            }
            // Double the interval after each pass.
            interval *= 2;
        }

        // The final merged list will be at index 0.
        lists[0].take()
    }

    fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;
        let mut l1 = l1;
        let mut l2 = l2;

        // Iterate while both lists have nodes.
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                // Attach the node from l1.
                let mut node = l1.take().unwrap();
                l1 = node.next.take();
                curr.next = Some(node);
            } else {
                // Attach the node from l2.
                let mut node = l2.take().unwrap();
                l2 = node.next.take();
                curr.next = Some(node);
            }
            // Move current pointer forward.
            curr = curr.next.as_mut().unwrap();
        }

        // Attach the remaining nodes from either l1 or l2.
        curr.next = l1.or(l2);
        // Return the merged list starting from dummy's next node.
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn test_merge_k_lists() {
        let lists = vec![
            to_list(vec![1, 4, 5]),
            to_list(vec![1, 3, 4]),
            to_list(vec![2, 6]),
        ];
        let result = Solution::merge_k_lists(lists);
        let expected = to_list(vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_lists() {
        let lists: Vec<Option<Box<ListNode>>> = vec![];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(result, None);
    }

    #[test]
    fn test_one_empty_list() {
        let lists = vec![None];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(result, None);
    }
}
