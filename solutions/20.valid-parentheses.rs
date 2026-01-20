// Category: algorithms
// Level: Easy
// Percent: 43.45381%

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
//
// 	Open brackets must be closed by the same type of brackets.
// 	Open brackets must be closed in the correct order.
// 	Every close bracket has a corresponding open bracket of the same type.
//
//
//
// Example 1:
//
//
// Input: s = "()"
//
// Output: true
//
//
// Example 2:
//
//
// Input: s = "()[]{}"
//
// Output: true
//
//
// Example 3:
//
//
// Input: s = "(]"
//
// Output: false
//
//
// Example 4:
//
//
// Input: s = "([])"
//
// Output: true
//
//
// Example 5:
//
//
// Input: s = "([)]"
//
// Output: false
//
//
//
// Constraints:
//
//
// 	1 <= s.length <= 10⁴
// 	s consists of parentheses only '()[]{}'.
//

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::with_capacity(s.len());

        for b in s.as_bytes() {
            match b {
                b'(' => stack.push(b')'),
                b'[' => stack.push(b']'),
                b'{' => stack.push(b'}'),
                _ => {
                    if stack.pop() != Some(*b) {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}
