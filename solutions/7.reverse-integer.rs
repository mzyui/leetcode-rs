// Category: algorithms
// Level: Medium
// Percent: 31.23623%



// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2³¹, 2³¹ - 1], then return 0.
// 
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
// 
//  
// Example 1:
// 
// Input: x = 123
// Output: 321
// 
// 
// Example 2:
// 
// Input: x = -123
// Output: -321
// 
// 
// Example 3:
// 
// Input: x = 120
// Output: 21
// 
// 
//  
// Constraints:
// 
// 
// 	-2³¹ <= x <= 2³¹ - 1
// 
 
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut r = 0;
        while x != 0 {
            let d = x % 10;
            x /= 10;
            if r > 214748364 || (r == 214748364 && d > 7) { return 0; }
            if r < -214748364 || (r == -214748364 && d < -8) { return 0; }
            r = r * 10 + d;
        }
        r
    }
}
