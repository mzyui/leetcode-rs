---
title: Longest Substring Without Repeating Characters
---

# 3. Longest Substring Without Repeating Characters

**Category:** algorithms\
**Difficulty:** Medium\
**Acceptance:** 38.123196%

**LeetCode:** [View on LeetCode](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

---

## Problem

Given a string s, find the length of the longest substring without duplicate characters.

---

## Examples

### Example 1

**Input:** s = "abcabcbb"\
**Output:** 3\
**Explanation:** The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.

### Example 2

**Input:** s = "bbbbb"\
**Output:** 1\
**Explanation:** The answer is "b", with the length of 1.

### Example 3

**Input:** s = "pwwkew"\
**Output:** 3\
**Explanation:** The answer is "wke", with the length of 3.\
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

---

## Constraints

- **0** <= *s.length* <= **5** * **10**⁴
- s consists of English letters, digits, symbols and *spaces.*

---

## Source / Solution

<details>

<summary>Click to reveal solution hint</summary>

```rust
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut last = [-1; 128];
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..bytes.len() {
            let c = bytes[right] as usize;

            if last[c] >= left {
                left = last[c] + 1;
            }

            last[c] = right as i32;
            max_len = max_len.max(right as i32 - left + 1);
        }

        max_len
    }
}
```

</details>

---

<small>[Back to index](../)</small>
