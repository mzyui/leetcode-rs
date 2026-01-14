// Category: algorithms
// Level: Easy
// Percent: 46.813683%

// Write a function to find the longest common prefix string amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
//
//
// Example 1:
//
// Input: strs = ["flower","flow","flight"]
// Output: "fl"
//
//
// Example 2:
//
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
//
//
//
// Constraints:
//
//
// 	1 <= strs.length <= 200
// 	0 <= strs[i].length <= 200
// 	strs[i] consists of only lowercase English letters if it is non-empty.
//

impl Solution {
    pub fn longest_common_prefix(arr: Vec<String>) -> String {
        let min_len = arr.iter().map(|s| s.len()).min().unwrap();
        let first = arr[0].as_bytes();

        let mut result = String::with_capacity(min_len);

        'outer: for i in 0..min_len {
            let c = first[i];

            for s in arr.iter().skip(1) {
                if s.as_bytes()[i] != c {
                    break 'outer;
                }
            }

            result.push(c as char);
        }
        result
    }
}
