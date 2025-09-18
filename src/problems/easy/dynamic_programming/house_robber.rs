/// Problem: House Robber
///
/// You are a professional robber planning to rob houses along a street. Each house has a certain
/// amount of money stashed, the only constraint stopping you from robbing each of them is that
/// adjacent houses have security systems connected and it will automatically contact the police
/// if two adjacent houses were broken into on the same night.
///
/// Given an integer array nums representing the amount of money of each house, return the maximum
/// amount of money you can rob tonight without alerting the police.
///
/// Example 1:
/// Input: nums = [1,2,3,1]
/// Output: 4
/// Explanation: Rob house 0 (money = 1) and house 2 (money = 3).
/// Total amount you can rob = 1 + 3 = 4.
///
/// Example 2:
/// Input: nums = [2,7,9,3,1]
/// Output: 12
/// Explanation: Rob house 0 (money = 2), house 2 (money = 9) and house 4 (money = 1).
/// Total amount you can rob = 2 + 9 + 1 = 12.
///
/// Constraints:
/// - 1 <= nums.length <= 100
/// - 0 <= nums[i] <= 400
///
/// Algorithm Pattern: Skip Pattern Dynamic Programming
/// This class of problems can be solved with Skip Pattern DP when you need to:
/// - Make binary decisions (take it or skip it)
/// - Cannot take adjacent elements
/// - Maximize/minimize value over sequences
/// - Each choice affects future valid choices
///
/// Translation Strategy:
/// English: "Maximum money without robbing adjacent houses"
/// Algorithm: At each house, choose max(rob this + best from 2 houses ago, skip this + best from 1 house ago)
///
/// Key DP Insight:
/// At each house i, you have two choices:
/// 1. Rob house i: get money[i] + max money from houses 0 to i-2
/// 2. Skip house i: get max money from houses 0 to i-1
/// Take the maximum of these two options.
///
/// Recurrence Relation:
/// dp[i] = max(nums[i] + dp[i-2], dp[i-1])
/// Base cases: dp[0] = nums[0], dp[1] = max(nums[0], nums[1])
///
/// Common Skip Pattern Problems:
/// 1. House Robber variations (linear, circular, binary tree)
/// 2. Maximum sum non-adjacent elements
/// 3. Jump game variants
/// 4. Delete and earn
/// 5. Paint house problems
///
/// Hints:
/// 1. Define state: dp[i] = max money from houses 0 to i
/// 2. Two choices at each house: rob or skip
/// 3. If rob house i, can't rob house i-1, so add to dp[i-2]
/// 4. If skip house i, take the best up to house i-1
/// 5. Space optimization: only need last 2 values
///
/// Time Complexity: O(n) - process each house once
/// Space Complexity: O(1) - only store last 2 values

#[no_mangle]
pub extern "C" fn rob(nums: *const i32, nums_size: i32) -> i32 {
    if nums.is_null() || nums_size == 0 {
        return 0;
    }

    let nums_slice = unsafe { std::slice::from_raw_parts(nums, nums_size as usize) };

    // TODO: Implement the skip pattern DP solution
    //
    // Your implementation should:
    // 1. Handle base cases:
    //    - 1 house: rob it (return nums[0])
    //    - 2 houses: rob the better one (return max(nums[0], nums[1]))
    // 2. For houses 2 to n-1:
    //    - Choice 1: Rob current house + best from 2 houses ago
    //    - Choice 2: Skip current house, take best from 1 house ago
    //    - Take maximum of both choices
    // 3. Space-optimized approach:
    //    - prev2 = best money up to house i-2
    //    - prev1 = best money up to house i-1
    //    - current = max(nums[i] + prev2, prev1)
    //    - Update prev2, prev1 for next iteration
    //
    // Key insight: This is NOT Fibonacci!
    // Fibonacci: f(n) = f(n-1) + f(n-2) (always sum)
    // House Robber: dp[i] = max(nums[i] + dp[i-2], dp[i-1]) (choose max)
    //
    // Alternative approaches:
    // - Recursive: O(2^n) time, explores all possibilities
    // - Recursive + memoization: O(n) time, O(n) space
    // - DP array: O(n) time, O(n) space
    // - DP optimized: O(n) time, O(1) space (optimal)

    // Placeholder - replace with your implementation
    0
}

/// Bonus: House Robber II (Circular arrangement)
/// Houses are arranged in a circle, so first and last houses are adjacent
#[no_mangle]
pub extern "C" fn rob_circular(nums: *const i32, nums_size: i32) -> i32 {
    if nums.is_null() || nums_size == 0 {
        return 0;
    }

    if nums_size == 1 {
        return unsafe { *nums };
    }

    // TODO: Handle circular constraint
    // Hint: Either rob houses [0..n-2] OR [1..n-1], take maximum
    // This ensures first and last houses are never both robbed

    // Placeholder
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob_basic() {
        let nums1 = vec![1, 2, 3, 1];
        assert_eq!(rob(nums1.as_ptr(), nums1.len() as i32), 4); // [1,3]

        let nums2 = vec![2, 7, 9, 3, 1];
        assert_eq!(rob(nums2.as_ptr(), nums2.len() as i32), 12); // [2,9,1]
    }

    #[test]
    fn test_rob_single_house() {
        let nums = vec![5];
        assert_eq!(rob(nums.as_ptr(), nums.len() as i32), 5);
    }

    #[test]
    fn test_rob_two_houses() {
        let nums1 = vec![1, 2];
        assert_eq!(rob(nums1.as_ptr(), nums1.len() as i32), 2);

        let nums2 = vec![5, 1];
        assert_eq!(rob(nums2.as_ptr(), nums2.len() as i32), 5);
    }

    #[test]
    fn test_rob_all_same() {
        let nums = vec![2, 2, 2, 2];
        assert_eq!(rob(nums.as_ptr(), nums.len() as i32), 4); // Rob houses 0,2

        let nums2 = vec![3, 3, 3, 3, 3];
        assert_eq!(rob(nums2.as_ptr(), nums2.len() as i32), 9); // Rob houses 0,2,4
    }

    #[test]
    fn test_rob_increasing() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(rob(nums.as_ptr(), nums.len() as i32), 9); // [1,3,5] or [2,4,5]
    }

    #[test]
    fn test_rob_edge_cases() {
        // Empty array
        assert_eq!(rob(std::ptr::null(), 0), 0);

        // Zero values
        let nums = vec![0, 0, 0];
        assert_eq!(rob(nums.as_ptr(), nums.len() as i32), 0);

        // Mix of zeros and values
        let nums2 = vec![0, 1, 0, 3, 0];
        assert_eq!(rob(nums2.as_ptr(), nums2.len() as i32), 4); // [1,3]
    }

    #[test]
    fn test_rob_alternating_pattern() {
        let nums = vec![5, 1, 2, 6];
        assert_eq!(rob(nums.as_ptr(), nums.len() as i32), 11); // [5,6]

        let nums2 = vec![2, 1, 1, 9];
        assert_eq!(rob(nums2.as_ptr(), nums2.len() as i32), 11); // [2,9]
    }

    #[test]
    fn test_rob_circular_basic() {
        let nums1 = vec![2, 3, 2];
        assert_eq!(rob_circular(nums1.as_ptr(), nums1.len() as i32), 3);

        let nums2 = vec![1, 2, 3, 1];
        assert_eq!(rob_circular(nums2.as_ptr(), nums2.len() as i32), 4);
    }

    #[test]
    fn test_rob_circular_single() {
        let nums = vec![5];
        assert_eq!(rob_circular(nums.as_ptr(), nums.len() as i32), 5);
    }

    #[test]
    fn test_rob_vs_circular() {
        // Linear version can rob both ends, circular cannot
        let nums = vec![1, 2, 3];
        assert_eq!(rob(nums.as_ptr(), nums.len() as i32), 4); // [1,3]
        assert_eq!(rob_circular(nums.as_ptr(), nums.len() as i32), 3); // [2] or [3]
    }
}