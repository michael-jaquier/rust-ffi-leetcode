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

    let range = s_slice.as_mut_ptr_range();
    let mut left = range.start;
    let mut right = unsafe { range.end.sub(1) };

    while left < right {
        unsafe {
            let temp: u8 = *left;
            *left = *right;
            *right = temp;
            left = left.add(1);
            right = right.sub(1);
        }
    }

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

    let mut left = s_slice.as_ptr();
    let mut right = unsafe { left.add(s_size as usize - 1) };

    while left < right {
        unsafe {
            if *left != *right {
                return false;
            }
        }
        left = unsafe { left.add(1) };
        right = unsafe { right.sub(1) };
    }

    // Placeholder
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string_basic() {
        let mut s = b"hello".to_vec();
        reverse_string(s.as_mut_ptr(), s.len() as i32);
        assert_eq!(&s, b"olleh");
    }

    #[test]
    fn test_reverse_string_even_length() {
        let mut s = b"Hannah".to_vec();
        reverse_string(s.as_mut_ptr(), s.len() as i32);
        assert_eq!(&s, b"hannaH");
    }

    #[test]
    fn test_reverse_string_single_char() {
        let mut s = b"A".to_vec();
        reverse_string(s.as_mut_ptr(), s.len() as i32);
        assert_eq!(&s, b"A");
    }

    #[test]
    fn test_reverse_string_two_chars() {
        let mut s = b"ab".to_vec();
        reverse_string(s.as_mut_ptr(), s.len() as i32);
        assert_eq!(&s, b"ba");
    }

    #[test]
    fn test_reverse_string_empty() {
        let mut s = b"".to_vec();
        reverse_string(s.as_mut_ptr(), s.len() as i32);
        assert_eq!(&s, b"");
    }

    #[test]
    fn test_reverse_string_palindrome() {
        let mut s = b"racecar".to_vec();
        reverse_string(s.as_mut_ptr(), s.len() as i32);
        assert_eq!(&s, b"racecar");
    }

    #[test]
    fn test_is_palindrome_basic() {
        assert_eq!(is_palindrome(b"racecar".as_ptr(), 7), true);
        assert_eq!(is_palindrome(b"race a car".as_ptr(), 10), false);
        assert_eq!(is_palindrome(b"hello".as_ptr(), 5), false);
    }

    #[test]
    fn test_is_palindrome_single_char() {
        assert_eq!(is_palindrome(b"a".as_ptr(), 1), true);
    }

    #[test]
    fn test_is_palindrome_empty() {
        assert_eq!(is_palindrome(b"".as_ptr(), 0), true);
    }

    #[test]
    fn test_is_palindrome_even_length() {
        assert_eq!(is_palindrome(b"abba".as_ptr(), 4), true);
        assert_eq!(is_palindrome(b"abcd".as_ptr(), 4), false);
    }

    #[test]
    fn test_edge_cases() {
        // Null pointer should not crash
        reverse_string(std::ptr::null_mut(), 0);

        // Zero length
        let mut s = b"test".to_vec();
        reverse_string(s.as_mut_ptr(), 0);
        assert_eq!(&s, b"test"); // Should be unchanged

        // Negative length (treated as <= 1)
        let mut s = b"test".to_vec();
        reverse_string(s.as_mut_ptr(), -1);
        assert_eq!(&s, b"test"); // Should be unchanged
    }
}
