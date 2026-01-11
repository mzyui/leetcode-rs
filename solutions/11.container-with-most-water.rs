// Category: algorithms
// Level: Medium
// Percent: 59.234516%

// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
//
// Find two lines that together with the x-axis form a container, such that the container contains the most water.
//
// Return the maximum amount of water a container can store.
//
// Notice that you may not slant the container.
//
//
// Example 1:
//
// Input: height = [1,8,6,2,5,4,8,3,7]
// Output: 49
// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
//
//
// Example 2:
//
// Input: height = [1,1]
// Output: 1
//
//
//
// Constraints:
//
//
// 	n == height.length
// 	2 <= n <= 10⁵
// 	0 <= height[i] <= 10⁴
//

/*
left = 0
right = n - 1
maxArea = 0

while left < right:
    area = (right - left) * min(height[left], height[right])
    maxArea = max(maxArea, area)

    if height[left] < height[right]:
        left += 1
    else:
        right -= 1

return maxArea
*/

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let (mut l, mut r, mut m) = (0, n - 1, 0i32);
        while l < r {
            let a = (r - l) as i32 * height[l].min(height[r]);
            m = m.max(a);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        m
    }
}
