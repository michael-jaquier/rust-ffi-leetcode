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