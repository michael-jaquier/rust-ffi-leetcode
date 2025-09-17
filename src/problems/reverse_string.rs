/// Problem: Reverse String
///
/// Write a function that reverses a string. The input string is given as an array of characters s.
/// You must do this by modifying the input array in-place with O(1) extra memory.
///
/// Example 1:
/// Input: s = ['h','e','l','l','o']
/// Output: ['o','l','l','e','h']
///
/// Example 2:
/// Input: s = ['H','a','n','n','a','h']
/// Output: ['h','a','n','n','a','H']
///
/// Constraints:
/// - 1 <= s.length <= 10^5
/// - s[i] is a printable ascii character.
///
/// Algorithm Pattern: Two Pointers Technique
/// This class of problems can be solved with two pointers when you need to:
/// - Process array/string from both ends moving inward
/// - Swap or compare elements from opposite ends
/// - Avoid using extra space (in-place operations)
/// - Find pairs or palindromes
///
/// Translation Strategy:
/// English: "Reverse the string in-place"
/// Algorithm: Use two pointers, one at start, one at end, swap and move toward center
///
/// Common Two Pointer Patterns:
/// 1. Opposite ends moving inward (this problem, palindrome check)
/// 2. Fast/slow pointers (cycle detection, finding middle)
/// 3. Left/right pointers on sorted array (two sum on sorted array)
/// 4. Sliding window (subarray problems)
///
/// Hints:
/// 1. Place one pointer at start (index 0), another at end (index len-1)
/// 2. Swap characters at both positions
/// 3. Move left pointer right, right pointer left
/// 4. Continue until pointers meet or cross
/// 5. No additional storage needed - modify in place
///
/// Time Complexity: O(n) - visit each element once
/// Space Complexity: O(1) - only using two pointer variables

#[no_mangle]
pub extern "C" fn reverse_string(s: *mut u8, s_size: i32) {
    if s.is_null() || s_size <= 1 {
        return;
    }

    let s_slice = unsafe { std::slice::from_raw_parts_mut(s, s_size as usize) };

    // TODO: Implement the two pointers algorithm
    //
    // Your implementation should:
    // 1. Initialize left = 0, right = s_size - 1
    // 2. While left < right:
    //    a. Swap s[left] with s[right]
    //    b. Increment left
    //    c. Decrement right
    // 3. Continue until pointers meet
    //
    // Alternative approaches:
    // - Recursive: O(n) space due to call stack
    // - Using extra array: O(n) space, not in-place
    // - Two pointers: O(1) space, optimal for this constraint

    // Placeholder - replace with your implementation
}

/// Bonus Problem: Check if string is palindrome
/// Uses the same two-pointer pattern but with comparison instead of swapping
#[no_mangle]
pub extern "C" fn is_palindrome(s: *const u8, s_size: i32) -> bool {
    if s.is_null() || s_size <= 1 {
        return true;
    }

    let s_slice = unsafe { std::slice::from_raw_parts(s, s_size as usize) };

    // TODO: Implement palindrome check using two pointers
    // Same pattern as reverse_string but compare instead of swap

    // Placeholder
    false
}