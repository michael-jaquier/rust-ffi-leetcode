use std::collections::HashMap;

/// Problem: Two Sum
///
/// Given an array of integers nums and an integer target, return indices of the two numbers
/// such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the
/// same element twice. You can return the answer in any order.
///
/// Example 1:
/// Input: nums = [2,7,11,15], target = 9
/// Output: [0,1]
/// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// Example 2:
/// Input: nums = [3,2,4], target = 6
/// Output: [1,2]
///
/// Example 3:
/// Input: nums = [3,3], target = 6
/// Output: [0,1]
///
/// Constraints:
/// - 2 <= nums.length <= 10^4
/// - -10^9 <= nums[i] <= 10^9
/// - -10^9 <= target <= 10^9
/// - Only one valid answer exists.
///
/// Algorithm Pattern: Hash Map for O(1) Lookups
/// This class of problems can be solved with HashMap/HashSet when you need to:
/// - Track what you've seen before
/// - Find complements (target - current_value)
/// - Convert O(n²) brute force to O(n) with space trade-off
///
/// Translation Strategy:
/// English: "Find two numbers that add up to target"
/// Algorithm: For each number, check if (target - number) exists in what we've seen
///
/// Hints:
/// 1. Brute force: Check every pair O(n²) - nested loops
/// 2. Optimized: Use HashMap to remember values and their indices
/// 3. For each element, check if (target - element) exists in map
/// 4. If found, return both indices; if not, add current element to map
/// 5. HashMap gives O(1) average lookup time
///
/// Time Complexity: O(n) - single pass through array
/// Space Complexity: O(n) - HashMap storage in worst case

#[no_mangle]
pub extern "C" fn two_sum(nums: *const i32, nums_size: i32, target: i32, result: *mut i32) -> bool {
    if nums.is_null() || result.is_null() || nums_size < 2 {
        return false;
    }

    let nums_slice = unsafe { std::slice::from_raw_parts(nums, nums_size as usize) };

    // TODO: Implement the hash map approach
    //
    // Your implementation should:
    // 1. Create a HashMap to store value -> index mapping
    // 2. Iterate through the array with index and value
    // 3. Calculate complement = target - current_value
    // 4. Check if complement exists in the map
    // 5. If yes: write both indices to result array and return true
    // 6. If no: add current value and index to map
    // 7. Continue until solution found
    //
    // Alternative approaches to consider:
    // - Brute force: O(n²) nested loops
    // - Sort + two pointers: O(n log n) but loses original indices
    // - Hash map: O(n) time, O(n) space - optimal for this problem

    // Placeholder - replace with your implementation
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_two_sum_basic() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let mut result = [0i32, 0i32];

        let found = two_sum(nums.as_ptr(), nums.len() as i32, target, result.as_mut_ptr());

        assert!(found);
        // Should return indices 0 and 1
        assert!((result[0] == 0 && result[1] == 1) || (result[0] == 1 && result[1] == 0));
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }

    #[test]
    fn test_two_sum_different_positions() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let mut result = [0i32, 0i32];

        let found = two_sum(nums.as_ptr(), nums.len() as i32, target, result.as_mut_ptr());

        assert!(found);
        // Should return indices 1 and 2
        assert!((result[0] == 1 && result[1] == 2) || (result[0] == 2 && result[1] == 1));
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }

    #[test]
    fn test_two_sum_same_value() {
        let nums = vec![3, 3];
        let target = 6;
        let mut result = [0i32, 0i32];

        let found = two_sum(nums.as_ptr(), nums.len() as i32, target, result.as_mut_ptr());

        assert!(found);
        // Should return indices 0 and 1
        assert!((result[0] == 0 && result[1] == 1) || (result[0] == 1 && result[1] == 0));
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }

    #[test]
    fn test_two_sum_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        let mut result = [0i32, 0i32];

        let found = two_sum(nums.as_ptr(), nums.len() as i32, target, result.as_mut_ptr());

        assert!(found);
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }

    #[test]
    fn test_two_sum_no_solution() {
        let nums = vec![1, 2, 3, 4];
        let target = 10;
        let mut result = [0i32, 0i32];

        let found = two_sum(nums.as_ptr(), nums.len() as i32, target, result.as_mut_ptr());

        assert!(!found);
    }

    #[test]
    fn test_two_sum_edge_cases() {
        let mut result = [0i32, 0i32];

        // Null pointer
        let found = two_sum(std::ptr::null(), 0, 0, result.as_mut_ptr());
        assert!(!found);

        // Empty array
        let nums = vec![];
        let found = two_sum(nums.as_ptr(), 0, 5, result.as_mut_ptr());
        assert!(!found);

        // Single element
        let nums = vec![1];
        let found = two_sum(nums.as_ptr(), 1, 2, result.as_mut_ptr());
        assert!(!found);
    }
}