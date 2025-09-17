/// Problem: Maximum Subarray (Kadane's Algorithm)
///
/// Given an integer array nums, find the contiguous subarray (containing at least one number)
/// which has the largest sum and return its sum.
///
/// A subarray is a contiguous part of an array.
///
/// Example 1:
/// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
/// Output: 6
/// Explanation: [4,-1,2,1] has the largest sum = 6.
///
/// Example 2:
/// Input: nums = [1]
/// Output: 1
///
/// Example 3:
/// Input: nums = [5,4,-1,7,8]
/// Output: 23
///
/// Constraints:
/// - 1 <= nums.length <= 10^5
/// - -10^4 <= nums[i] <= 10^4
///
/// Algorithm Pattern: Dynamic Programming (Kadane's Algorithm)
/// This class of problems can be solved with Dynamic Programming when you need to:
/// - Find optimal solution by building from subproblems
/// - Make decisions based on previous optimal choices
/// - Avoid recalculating the same subproblems
/// - Transform recursive solutions to iterative with memoization
///
/// Translation Strategy:
/// English: "Find contiguous subarray with maximum sum"
/// Algorithm: At each position, decide whether to extend previous subarray or start fresh
///
/// Key DP Insight:
/// At each position i, you have two choices:
/// 1. Add nums[i] to the previous maximum subarray ending at i-1
/// 2. Start a new subarray from nums[i]
/// Choose whichever gives larger sum.
///
/// Common DP Patterns:
/// 1. 1D DP: Fibonacci, climbing stairs, house robber
/// 2. 2D DP: Longest common subsequence, edit distance
/// 3. Kadane's algorithm: Maximum subarray variants
/// 4. Sliding window: When constraints allow optimization
///
/// Hints:
/// 1. Keep track of maximum sum ending at current position
/// 2. At each step: max_ending_here = max(nums[i], max_ending_here + nums[i])
/// 3. Also track global maximum seen so far
/// 4. Global max = max(global_max, max_ending_here)
/// 5. The decision: extend previous subarray or start new one
///
/// Time Complexity: O(n) - single pass through array
/// Space Complexity: O(1) - only need two variables

#[no_mangle]
pub extern "C" fn max_subarray(nums: *const i32, nums_size: i32) -> i32 {
    if nums.is_null() || nums_size == 0 {
        return 0;
    }

    let nums_slice = unsafe { std::slice::from_raw_parts(nums, nums_size as usize) };

    // TODO: Implement Kadane's algorithm
    //
    // Your implementation should:
    // 1. Initialize max_ending_here = nums[0]
    // 2. Initialize max_so_far = nums[0]
    // 3. For i from 1 to nums_size-1:
    //    a. max_ending_here = max(nums[i], max_ending_here + nums[i])
    //    b. max_so_far = max(max_so_far, max_ending_here)
    // 4. Return max_so_far
    //
    // The key insight:
    // - max_ending_here: best sum ending at current position
    // - If previous sum is negative, better to start fresh
    // - If previous sum is positive, extend it
    //
    // Alternative approaches:
    // - Brute force: O(n²) check all subarrays
    // - Divide and conquer: O(n log n) but more complex
    // - Kadane's: O(n) optimal solution

    // Placeholder - replace with your implementation
    if nums_size > 0 {
        unsafe { *nums }  // Return first element as placeholder
    } else {
        0
    }
}

/// Bonus: Return the actual subarray indices, not just the sum
#[no_mangle]
pub extern "C" fn max_subarray_indices(
    nums: *const i32,
    nums_size: i32,
    start: *mut i32,
    end: *mut i32
) -> i32 {
    if nums.is_null() || start.is_null() || end.is_null() || nums_size == 0 {
        return 0;
    }

    // TODO: Modify Kadane's to also track the start and end indices
    // Hint: Reset start index when starting a new subarray

    // Placeholder
    unsafe {
        *start = 0;
        *end = 0;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray_basic() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, 6); // [4, -1, 2, 1]
    }

    #[test]
    fn test_max_subarray_single_element() {
        let nums = vec![1];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, 1);

        let nums = vec![-1];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_max_subarray_all_positive() {
        let nums = vec![5, 4, -1, 7, 8];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, 23); // Entire array
    }

    #[test]
    fn test_max_subarray_all_negative() {
        let nums = vec![-3, -5, -1, -7];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, -1); // Largest single element
    }

    #[test]
    fn test_max_subarray_mixed() {
        let nums = vec![1, 2, -1, -2, 2, 1, -2, 1];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, 3); // [1, 2] or [2, 1]
    }

    #[test]
    fn test_max_subarray_alternating() {
        let nums = vec![-2, 1, -2, 1, -2, 1];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, 1); // Any single 1
    }

    #[test]
    fn test_max_subarray_edge_cases() {
        // Empty array
        let result = max_subarray(std::ptr::null(), 0);
        assert_eq!(result, 0);

        // Null pointer
        let nums = vec![];
        let result = max_subarray(nums.as_ptr(), 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_max_subarray_large_numbers() {
        let nums = vec![1000, -1, 1000, -1, 1000];
        let result = max_subarray(nums.as_ptr(), nums.len() as i32);
        assert_eq!(result, 2998); // [1000, -1, 1000, -1, 1000]
    }

    #[test]
    fn test_max_subarray_indices_basic() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let mut start = 0i32;
        let mut end = 0i32;

        let result = max_subarray_indices(nums.as_ptr(), nums.len() as i32, &mut start, &mut end);
        assert_eq!(result, 6);

        // Verify the subarray [start..=end] has the maximum sum
        let subarray_sum: i32 = nums[(start as usize)..=(end as usize)].iter().sum();
        assert_eq!(subarray_sum, 6);
    }

    #[test]
    fn test_max_subarray_indices_single() {
        let nums = vec![5];
        let mut start = 0i32;
        let mut end = 0i32;

        let result = max_subarray_indices(nums.as_ptr(), nums.len() as i32, &mut start, &mut end);
        assert_eq!(result, 5);
        assert_eq!(start, 0);
        assert_eq!(end, 0);
    }

    #[test]
    fn test_max_subarray_indices_edge_cases() {
        let mut start = 0i32;
        let mut end = 0i32;

        // Null pointer
        let result = max_subarray_indices(std::ptr::null(), 0, &mut start, &mut end);
        assert_eq!(result, 0);

        // Empty array
        let nums = vec![];
        let result = max_subarray_indices(nums.as_ptr(), 0, &mut start, &mut end);
        assert_eq!(result, 0);
    }
}