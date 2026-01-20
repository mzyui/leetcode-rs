// Category: algorithms
// Level: Medium
// Percent: 50.653057%

// Given the head of a linked list, remove the nth node from the end of the list and return its head.
//
//
// Example 1:
//
// Input: head = [1,2,3,4,5], n = 2
// Output: [1,2,3,5]
//
//
// Example 2:
//
// Input: head = [1], n = 1
// Output: []
//
//
// Example 3:
//
// Input: head = [1,2], n = 1
// Output: [1]
//
//
//
// Constraints:
//
//
// 	The number of nodes in the list is sz.
// 	1 <= sz <= 30
// 	0 <= Node.val <= 100
// 	1 <= n <= sz
//
//
//
// Follow up: Could you do this in one pass?

// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut p = head.as_ref();
        while let Some(node) = p {
            len += 1;
            p = node.next.as_ref();
        }

        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        for _ in 0..(len - n) {
            cur = cur.next.as_mut().unwrap();
        }

        let next = cur.next.as_mut().unwrap().next.take();
        cur.next = next;

        dummy.next
    }
}
