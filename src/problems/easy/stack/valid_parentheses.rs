use core::panic;
use std::{collections::HashMap, ptr::slice_from_raw_parts};

/// Problem: Valid Parentheses
///
/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
/// determine if the input string is valid.
///
/// An input string is valid if:
/// 1. Open brackets must be closed by the same type of brackets.
/// 2. Open brackets must be closed in the correct order.
/// 3. Every close bracket has a corresponding open bracket of the same type.
///
/// Example 1:
/// Input: s = "()"
/// Output: true
///
/// Example 2:
/// Input: s = "()[]{}"
/// Output: true
///
/// Example 3:
/// Input: s = "(]"
/// Output: false
///
/// Example 4:
/// Input: s = "([)]"
/// Output: false
///
/// Example 5:
/// Input: s = "{[]}"
/// Output: true
///
/// Constraints:
/// - 1 <= s.length <= 10^4
/// - s consists of parentheses only '()[]{}'.
///
/// Algorithm Pattern: Stack for Matching Pairs
/// This class of problems can be solved with Stack/LIFO when you need to:
/// - Match opening and closing elements (parentheses, HTML tags)
/// - Process nested structures
/// - Validate proper ordering/pairing
/// - Undo operations or backtrack
/// - Function call management
///
/// Translation Strategy:
/// English: "Every opening bracket needs matching closing bracket in correct order"
/// Algorithm: Use stack to remember opening brackets, match with closing ones
///
/// Common Stack Patterns:
/// 1. Matching pairs (parentheses, quotes, tags)
/// 2. Expression evaluation (infix to postfix, calculator)
/// 3. Undo/Redo operations
/// 4. Function call stack simulation
/// 5. Monotonic stack (next greater element)
///
/// Hints:
/// 1. Stack stores opening brackets as you encounter them
/// 2. When you see closing bracket, check if stack top matches
/// 3. If stack is empty when you need to match, invalid
/// 4. If stack has leftover brackets at end, invalid
/// 5. Only valid if stack is empty at the end
///
/// Time Complexity: O(n) - process each character once
/// Space Complexity: O(n) - worst case all opening brackets

#[no_mangle]
pub extern "C" fn is_valid_parentheses(s: *const u8, s_size: i32) -> bool {
    if s.is_null() || s_size == 0 {
        return true;
    }

    let s_slice = unsafe { std::slice::from_raw_parts(s, s_size as usize) };
    let mut stack = Vec::new();
    for b in s_slice {
        match b {
            b'{' | b'[' | b'(' => stack.push(b),
            b'}' => if stack.pop() != Some(&b'{'){ return false;},
            b']' => if stack.pop() != Some(&b'[') { return false;},
            b')' => if stack.pop() != Some(&b'(') { return false;},
            _ => return false
        }
    }

    stack.is_empty()

    // TODO: Implement the stack-based matching algorithm
    //
    // Your implementation should:
    // 1. Create a Vec<u8> to use as stack
    // 2. For each character in string:
    //    a. If opening bracket '(', '[', '{': push to stack
    //    b. If closing bracket ')', ']', '}':
    //       - Check if stack is empty (invalid if true)
    //       - Pop from stack and check if it matches current closing bracket
    //       - If doesn't match, return false
    // 3. After processing all characters, stack should be empty
    // 4. Return stack.is_empty()
    //
    // Matching pairs:
    // - '(' matches ')'
    // - '[' matches ']'
    // - '{' matches '}'
    //
    // Alternative approaches:
    // - Counter approach: works only for single type of bracket
    // - Recursion: more complex, uses call stack
    // - Stack: optimal for multiple bracket types

    // Placeholder - replace with your implementation
}

/// Bonus: Minimum parentheses to remove to make valid
/// Uses stack pattern to identify invalid brackets
#[no_mangle]
pub extern "C" fn min_remove_to_make_valid(s: *const u8, s_size: i32) -> i32 {
    if s.is_null() || s_size == 0 {
        return 0;
    }
    let s_slice = unsafe {std::slice::from_raw_parts(s, s_size as usize)};
    let mut count = 0;
    let mut stack = Vec::new();
    let opens = b"{([";
    let closes = b"})]";
    let keys: HashMap<u8, u8> = opens.iter().zip(closes).map(|(a, b)| (*a, *b)).collect();

    for paren in s_slice {
        if opens.contains(paren) {
            stack.push(paren);
        } else {
            if let Some(open) = stack.pop() {
                let expected = keys.get(open).unwrap();
                if expected != paren {
                    count +=1;
                }
            } else { count +=1}

        }

    }

   (count + stack.len()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses_basic() {
        assert_eq!(is_valid_parentheses(b"()".as_ptr(), 2), true);
        assert_eq!(is_valid_parentheses(b"()[]{}".as_ptr(), 6), true);
        assert_eq!(is_valid_parentheses(b"(]".as_ptr(), 2), false);
        assert_eq!(is_valid_parentheses(b"([)]".as_ptr(), 4), false);
        assert_eq!(is_valid_parentheses(b"{[]}".as_ptr(), 4), true);
    }

    #[test]
    fn test_valid_parentheses_single_type() {
        assert_eq!(is_valid_parentheses(b"((()))".as_ptr(), 6), true);
        assert_eq!(is_valid_parentheses(b"[[[]]]".as_ptr(), 6), true);
        assert_eq!(is_valid_parentheses(b"{{{}}}".as_ptr(), 6), true);
    }

    #[test]
    fn test_valid_parentheses_unmatched() {
        assert_eq!(is_valid_parentheses(b"(".as_ptr(), 1), false);
        assert_eq!(is_valid_parentheses(b")".as_ptr(), 1), false);
        assert_eq!(is_valid_parentheses(b"(()".as_ptr(), 3), false);
        assert_eq!(is_valid_parentheses(b"())".as_ptr(), 3), false);
    }

    #[test]
    fn test_valid_parentheses_mixed_invalid() {
        assert_eq!(is_valid_parentheses(b"([{}])".as_ptr(), 6), true);
        assert_eq!(is_valid_parentheses(b"([{]})".as_ptr(), 6), false);
        assert_eq!(is_valid_parentheses(b"{[(])}".as_ptr(), 6), false);
    }

    #[test]
    fn test_valid_parentheses_edge_cases() {
        // Empty string is valid
        assert_eq!(is_valid_parentheses(b"".as_ptr(), 0), true);

        // Null pointer
        assert_eq!(is_valid_parentheses(std::ptr::null(), 0), true);

        // Single brackets
        assert_eq!(is_valid_parentheses(b"[".as_ptr(), 1), false);
        assert_eq!(is_valid_parentheses(b"]".as_ptr(), 1), false);
        assert_eq!(is_valid_parentheses(b"{".as_ptr(), 1), false);
        assert_eq!(is_valid_parentheses(b"}".as_ptr(), 1), false);
    }

    #[test]
    fn test_valid_parentheses_long_sequences() {
        // Deeply nested
        assert_eq!(is_valid_parentheses(b"(((())))".as_ptr(), 8), true);
        assert_eq!(is_valid_parentheses(b"((((())".as_ptr(), 7), false);

        // Mixed long sequence
        assert_eq!(is_valid_parentheses(b"({[()]})".as_ptr(), 8), true);
        assert_eq!(is_valid_parentheses(b"({[()]}))".as_ptr(), 9), false);
    }

    #[test]
    fn test_min_remove_to_make_valid_basic() {
        assert_eq!(min_remove_to_make_valid(b"()())()".as_ptr(), 7), 1);
        assert_eq!(min_remove_to_make_valid(b"((((((".as_ptr(), 6), 6);
        assert_eq!(min_remove_to_make_valid(b")))))))".as_ptr(), 7), 7);
        assert_eq!(min_remove_to_make_valid(b"()".as_ptr(), 2), 0);
    }

    #[test]
    fn test_min_remove_edge_cases() {
        assert_eq!(min_remove_to_make_valid(b"".as_ptr(), 0), 0);
        assert_eq!(min_remove_to_make_valid(std::ptr::null(), 0), 0);
        assert_eq!(min_remove_to_make_valid(b"(".as_ptr(), 1), 1);
        assert_eq!(min_remove_to_make_valid(b")".as_ptr(), 1), 1);
    }
}
