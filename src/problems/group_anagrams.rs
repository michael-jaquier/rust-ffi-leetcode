use crate::data_structures::StringArray;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Problem: Group Anagrams
///
/// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.
///
/// Example 1:
/// Input: strs = ["eat","tea","tan","ate","nat","bat"]
/// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
///
/// Example 2:
/// Input: strs = [""]
/// Output: [[""]]
///
/// Example 3:
/// Input: strs = ["a"]
/// Output: [["a"]]
///
/// Constraints:
/// - 1 <= strs.length <= 10^4
/// - 0 <= strs[i].length <= 100
/// - strs[i] consists of lowercase English letters only.
///
/// Hints:
/// 1. Two strings are anagrams if they have the same character frequency
/// 2. You can either sort each string as a key, or count character frequencies
/// 3. Use a HashMap where key is the canonical form (sorted string or frequency array)
/// 4. Group strings with the same canonical form together
///
/// Time Complexity: O(n * k log k) where n is number of strings, k is max string length
/// Space Complexity: O(n * k) for the hash map and result

#[no_mangle]
pub extern "C" fn group_anagrams(strs: *mut StringArray) -> *mut StringArray {
    if strs.is_null() {
        return std::ptr::null_mut();
    }

    // TODO: Implement the anagram grouping algorithm
    //
    // Your implementation should:
    // 1. Convert C strings to Rust strings
    // 2. For each string, create a canonical key (sorted characters or frequency count)
    // 3. Use HashMap<String, Vec<String>> to group anagrams
    // 4. Convert the grouped results back to C-compatible StringArray
    // 5. Handle memory management properly
    //
    // Approach 1 (Sorting):
    // - Sort characters in each string to create key
    // - "eat" -> "aet", "tea" -> "aet", "ate" -> "aet"
    //
    // Approach 2 (Frequency):
    // - Count frequency of each character as key
    // - "eat" -> [1,0,0,0,1,0,...,1,0] (26-element array)

    // Placeholder - replace with your implementation
    std::ptr::null_mut()
}
