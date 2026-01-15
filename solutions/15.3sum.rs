// Category: algorithms
// Level: Medium
// Percent: 38.302032%

// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//
// Notice that the solution set must not contain duplicate triplets.
//
//
// Example 1:
//
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.
//
//
// Example 2:
//
// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.
//
//
// Example 3:
//
// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.
//
//
//
// Constraints:
//
//
// 	3 <= nums.length <= 3000
// 	-10⁵ <= nums[i] <= 10⁵
//

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res = Vec::new();
        if n < 3 {
            return res;
        }

        nums.sort_unstable();

        for i in 0..n - 2 {
            let a = nums[i];

            if a > 0 {
                break;
            }

            if i > 0 && a == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = n - 1;

            while l < r {
                let sum = a + nums[l] + nums[r];

                if sum == 0 {
                    res.push(vec![a, nums[l], nums[r]]);

                    let left_val = nums[l];
                    let right_val = nums[r];

                    while l < r && nums[l] == left_val {
                        l += 1;
                    }
                    while l < r && nums[r] == right_val {
                        r -= 1;
                    }
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }

        res
    }
}
