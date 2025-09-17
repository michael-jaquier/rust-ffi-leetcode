/// Problem: Binary Search
///
/// Given an array of integers nums which is sorted in ascending order, and an integer target,
/// write a function to search target in nums. If target exists, then return its index.
/// Otherwise, return -1.
///
/// You must write an algorithm with O(log n) runtime complexity.
///
/// Example 1:
/// Input: nums = [-1,0,3,5,9,12], target = 9
/// Output: 4
/// Explanation: 9 exists in nums and its index is 4
///
/// Example 2:
/// Input: nums = [-1,0,3,5,9,12], target = 2
/// Output: -1
/// Explanation: 2 does not exist in nums so return -1
///
/// Constraints:
/// - 1 <= nums.length <= 10^4
/// - -10^4 < nums[i], target < 10^4
/// - All integers in nums are unique.
/// - nums is sorted in ascending order.
///
/// Algorithm Pattern: Binary Search / Divide and Conquer
/// This class of problems can be solved with Binary Search when you have:
/// - Sorted array or search space
/// - Need to find specific target or insertion point
/// - Want O(log n) time complexity instead of O(n) linear search
/// - Search space can be divided in half based on comparison
///
/// Translation Strategy:
/// English: "Find target in sorted array efficiently"
/// Algorithm: Repeatedly cut search space in half by comparing middle element
///
/// Common Binary Search Variations:
/// 1. Exact match (this problem)
/// 2. Find insertion position (lower/upper bound)
/// 3. Search in rotated sorted array
/// 4. Find peak element
/// 5. Search in 2D matrix
/// 6. Find minimum/maximum that satisfies condition
///
/// Key Insight: Eliminate half of search space at each step
///
/// Hints:
/// 1. Initialize left = 0, right = array.length - 1
/// 2. While left <= right:
///    a. Calculate mid = left + (right - left) / 2  // Avoid overflow
///    b. If nums[mid] == target: return mid
///    c. If nums[mid] < target: search right half (left = mid + 1)
///    d. If nums[mid] > target: search left half (right = mid - 1)
/// 3. If loop ends without finding, return -1
/// 4. Be careful with boundary conditions and integer overflow
///
/// Time Complexity: O(log n) - divide search space by 2 each iteration
/// Space Complexity: O(1) - only using constant extra variables

#[no_mangle]
pub extern "C" fn binary_search(nums: *const i32, nums_size: i32, target: i32) -> i32 {
    if nums.is_null() || nums_size == 0 {
        return -1;
    }

    let nums_slice = unsafe { std::slice::from_raw_parts(nums, nums_size as usize) };

    // TODO: Implement binary search algorithm
    //
    // Your implementation should:
    // 1. Initialize left = 0, right = nums_size - 1
    // 2. While left <= right:
    //    a. Calculate mid = left + (right - left) / 2
    //    b. Compare nums[mid] with target:
    //       - If equal: return mid
    //       - If nums[mid] < target: left = mid + 1
    //       - If nums[mid] > target: right = mid - 1
    // 3. Return -1 if target not found
    //
    // Common mistakes to avoid:
    // - Using (left + right) / 2 can cause integer overflow
    // - Using left < right vs left <= right affects termination
    // - Off-by-one errors in boundary updates
    //
    // Alternative approaches:
    // - Linear search: O(n) time, simpler but slower
    // - Recursive binary search: O(log n) time, O(log n) space
    // - Iterative binary search: O(log n) time, O(1) space (optimal)

    // Placeholder - replace with your implementation
    -1
}

/// Bonus: Find insertion position (lower bound)
/// If target exists, return its index; otherwise return where it should be inserted
#[no_mangle]
pub extern "C" fn search_insert(nums: *const i32, nums_size: i32, target: i32) -> i32 {
    if nums.is_null() || nums_size == 0 {
        return 0;
    }

    // TODO: Modify binary search to find insertion position
    // Hint: When target not found, 'left' will be the insertion position

    // Placeholder
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 9), 4);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, -1), 0);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 12), 5);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 3), 2);
    }

    #[test]
    fn test_binary_search_not_found() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 2), -1);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 13), -1);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, -2), -1);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 7), -1);
    }

    #[test]
    fn test_binary_search_single_element() {
        let nums = vec![5];
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 5), 0);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 4), -1);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 6), -1);
    }

    #[test]
    fn test_binary_search_two_elements() {
        let nums = vec![1, 3];
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 1), 0);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 3), 1);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 2), -1);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 0), -1);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 4), -1);
    }

    #[test]
    fn test_binary_search_duplicates() {
        // Note: problem states all integers are unique, but testing edge case
        let nums = vec![1, 2, 2, 2, 3];
        let result = binary_search(nums.as_ptr(), nums.len() as i32, 2);
        // Should find one of the indices containing 2
        assert!(result >= 1 && result <= 3);
        assert_eq!(nums[result as usize], 2);
    }

    #[test]
    fn test_binary_search_edge_cases() {
        // Empty array
        assert_eq!(binary_search(std::ptr::null(), 0, 5), -1);

        // Null pointer
        let nums = vec![];
        assert_eq!(binary_search(nums.as_ptr(), 0, 5), -1);
    }

    #[test]
    fn test_binary_search_large_array() {
        let nums: Vec<i32> = (0..10000).collect();
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 5000), 5000);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 0), 0);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 9999), 9999);
        assert_eq!(binary_search(nums.as_ptr(), nums.len() as i32, 10000), -1);
    }

    #[test]
    fn test_search_insert_basic() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 5), 2);
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 2), 1);
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 7), 4);
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 0), 0);
    }

    #[test]
    fn test_search_insert_exact_match() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 1), 0);
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 3), 1);
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 6), 3);
    }

    #[test]
    fn test_search_insert_edge_cases() {
        // Empty array
        assert_eq!(search_insert(std::ptr::null(), 0, 5), 0);

        // Single element
        let nums = vec![1];
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 0), 0);
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 1), 0);
        assert_eq!(search_insert(nums.as_ptr(), nums.len() as i32, 2), 1);
    }
}