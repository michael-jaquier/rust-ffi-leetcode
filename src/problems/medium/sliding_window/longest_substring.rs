use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Problem: Longest Substring Without Repeating Characters
///
/// Given a string s, find the length of the longest substring without repeating characters.
///
/// Example 1:
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
///
/// Example 2:
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
///
/// Constraints:
/// - 0 <= s.length <= 5 * 10^4
/// - s consists of English letters, digits, symbols and spaces.
///
/// Hints:
/// 1. Use the sliding window technique
/// 2. Keep track of characters in current window using a HashSet
/// 3. When you find a duplicate, shrink window from left
/// 4. Keep track of maximum window size seen so far
///
/// Time Complexity: O(n) where n is the length of string
/// Space Complexity: O(min(m, n)) where m is the size of charset

#[no_mangle]
pub extern "C" fn length_of_longest_substring(s: *const c_char) -> i32 {
    if s.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(s) };
    let str_slice = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    // TODO: Implement the sliding window algorithm
    //
    // Your implementation should:
    // 1. Use two pointers (left and right) for the sliding window
    // 2. Use a HashSet to track characters in current window
    // 3. Expand the right pointer and add characters to set
    // 4. If duplicate found, shrink from left until no duplicate
    // 5. Track maximum window size

    // Placeholder - replace with your implementation
    0
}
