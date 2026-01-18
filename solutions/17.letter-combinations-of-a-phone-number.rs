// Category: algorithms
// Level: Medium
// Percent: 65.205795%

// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
//
// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
//
//
// Example 1:
//
// Input: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//
//
// Example 2:
//
// Input: digits = "2"
// Output: ["a","b","c"]
//
//
//
// Constraints:
//
//
// 	1 <= digits.length <= 4
// 	digits[i] is a digit in the range ['2', '9'].
//

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let map = [
            "", "", "abc", "def", "ghi",
            "jkl", "mno", "pqrs", "tuv", "wxyz"
        ];

        let mut result = vec![String::new()];

        for &d in digits.as_bytes() {
            let letters = map[(d - b'0') as usize];
            let mut next = Vec::with_capacity(result.len() * letters.len());

            for prefix in result {
                for c in letters.chars() {
                    let mut s = prefix.clone();
                    s.push(c);
                    next.push(s);
                }
            }

            result = next;
        }

        result
    }
}
