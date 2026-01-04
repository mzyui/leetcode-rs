// Category: algorithms
// Level: Hard
// Percent: 45.47212%



// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
// 
// The overall run time complexity should be O(log (m+n)).
// 
//  
// Example 1:
// 
// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
// 
// 
// Example 2:
// 
// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
// 
// 
//  
// Constraints:
// 
// 
// 	nums1.length == m
// 	nums2.length == n
// 	0 <= m <= 1000
// 	0 <= n <= 1000
// 	1 <= m + n <= 2000
// 	-10⁶ <= nums1[i], nums2[i] <= 10⁶
// 
 

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        nums1.extend(nums2);
        let n = nums1.len();
        let mid = n / 2;

        if n % 2 == 1 {
            nums1.select_nth_unstable(mid);
            nums1[mid] as f64
        } else {
            nums1.select_nth_unstable(mid);
            let upper = nums1[mid];
            nums1.select_nth_unstable(mid - 1);
            let lower = nums1[mid - 1];
            (lower as f64 + upper as f64) / 2.0
        }        
    }
}
