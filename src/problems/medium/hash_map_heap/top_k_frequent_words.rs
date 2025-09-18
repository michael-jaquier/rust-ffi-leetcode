/// Problem: Top K Frequent Words (Common in Tech Company Interviews)
///
/// Given a paragraph string and an integer K, write a function that returns the top K frequent
/// words in the paragraph along with their frequencies, sorted by frequency in descending order.
/// If two words have the same frequency, sort them lexicographically.
///
/// This is a common interview question at major tech companies.
///
/// Example 1:
/// Input: paragraph = "the quick brown fox jumps over the lazy dog the fox", k = 3
/// Output: [("the", 3), ("fox", 2), ("brown", 1)] or similar structure
/// Explanation: "the" appears 3 times, "fox" appears 2 times, others appear 1 time each
///
/// Example 2:
/// Input: paragraph = "apple banana apple cherry banana apple", k = 2
/// Output: [("apple", 3), ("banana", 2)]
///
/// Constraints:
/// - 1 <= k <= number of unique words
/// - Words are separated by spaces
/// - Case-sensitive comparison
/// - 1 <= paragraph.length <= 10^4
///
/// Algorithm Pattern: Hash Map + Heap (Priority Queue)
/// This class of problems can be solved with HashMap + Heap when you need to:
/// - Count frequencies of elements
/// - Find top K elements by some criteria
/// - Maintain sorted order efficiently
/// - Handle tie-breaking with secondary criteria
///
/// Translation Strategy:
/// English: "Find most frequent words in text"
/// Algorithm: Count frequencies with HashMap, then use heap to get top K elements
///
/// Real-World Context:
/// This mirrors log analysis tasks where you need to:
/// - Find most frequent error messages
/// - Identify top API endpoints by traffic
/// - Analyze most common user actions
/// - Process streaming log data for insights
///
/// Key Insights:
/// 1. Two-phase approach: Count + Select
/// 2. Heap automatically maintains top K elements
/// 3. Custom comparator handles frequency + lexicographic ordering
/// 4. Space-efficient: O(K) heap instead of sorting all words
///
/// Hints:
/// 1. Split paragraph into words
/// 2. Count word frequencies using HashMap
/// 3. Use min-heap of size K to track top K words
/// 4. Custom comparison: frequency (desc), then lexicographic (asc)
/// 5. Convert heap to result format
///
/// Time Complexity: O(N log K) where N is number of words
/// Space Complexity: O(N + K) for HashMap and heap

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WordFreq {
    pub word: *mut u8,      // C-style string
    pub word_len: i32,      // String length
    pub frequency: i32,     // Frequency count
}

// For internal use in heap
#[derive(Debug, Clone, PartialEq, Eq)]
struct HeapEntry {
    word: String,
    frequency: i32,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: reverse frequency order, then lexicographic
        match other.frequency.cmp(&self.frequency) {
            Ordering::Equal => self.word.cmp(&other.word), // Lexicographic ascending
            other => other, // Frequency descending (reversed for min-heap)
        }
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[no_mangle]
pub extern "C" fn top_k_frequent_words(
    paragraph: *const u8,
    paragraph_len: i32,
    k: i32,
    result: *mut WordFreq,
    result_size: *mut i32,
) -> bool {
    if paragraph.is_null() || result.is_null() || result_size.is_null() || k <= 0 {
        unsafe {
            if !result_size.is_null() {
                *result_size = 0;
            }
        }
        return false;
    }

    let text = unsafe { std::slice::from_raw_parts(paragraph, paragraph_len as usize) };
    let text_str = match std::str::from_utf8(text) {
        Ok(s) => s,
        Err(_) => {
            unsafe { *result_size = 0; }
            return false;
        }
    };

    // TODO: Implement top K frequent words algorithm
    //
    // Your implementation should:
    // 1. Split text into words: text_str.split_whitespace()
    // 2. Count frequencies: let mut freq_map = HashMap::new()
    // 3. Create min-heap of size K: let mut heap = BinaryHeap::new()
    // 4. For each word-frequency pair:
    //    a. If heap size < K: add to heap
    //    b. If current frequency > heap top frequency: replace heap top
    // 5. Convert heap to result array in descending frequency order
    // 6. Handle C-style strings: allocate and copy word strings
    //
    // Tech Interview Context:
    // This problem simulates real log analysis scenarios:
    // - Finding most frequent error types
    // - Identifying top API endpoints
    // - Analyzing user behavior patterns
    // - Processing streaming metrics data
    //
    // Alternative approaches:
    // - Sort all entries: O(N log N) time, simpler but less efficient
    // - QuickSelect: O(N) average time, complex implementation
    // - Heap approach: O(N log K) time, optimal for K << N

    unsafe { *result_size = 0; }
    false
}

/// Bonus: Streaming version for real-time log processing
#[no_mangle]
pub extern "C" fn update_word_frequencies(
    word: *const u8,
    word_len: i32,
    global_frequencies: *mut std::ffi::c_void, // Opaque pointer to internal state
) -> bool {
    // TODO: Implement streaming word frequency updates
    // This simulates real-time log processing in monitoring systems
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_basic() {
        let paragraph = "the quick brown fox jumps over the lazy dog the fox";
        let mut result = vec![WordFreq {
            word: std::ptr::null_mut(),
            word_len: 0,
            frequency: 0
        }; 3];
        let mut result_size = 0i32;

        let success = top_k_frequent_words(
            paragraph.as_ptr(),
            paragraph.len() as i32,
            3,
            result.as_mut_ptr(),
            &mut result_size,
        );

        // For now, just test that function doesn't crash
        assert!(!success); // Should fail with placeholder implementation
        assert_eq!(result_size, 0);
    }

    #[test]
    fn test_top_k_frequent_tie_breaking() {
        let paragraph = "apple banana cherry apple banana date";
        let mut result = vec![WordFreq {
            word: std::ptr::null_mut(),
            word_len: 0,
            frequency: 0
        }; 4];
        let mut result_size = 0i32;

        let success = top_k_frequent_words(
            paragraph.as_ptr(),
            paragraph.len() as i32,
            4,
            result.as_mut_ptr(),
            &mut result_size,
        );

        // Test with placeholder implementation
        assert!(!success);
    }

    #[test]
    fn test_top_k_frequent_single_word() {
        let paragraph = "test";
        let mut result = vec![WordFreq {
            word: std::ptr::null_mut(),
            word_len: 0,
            frequency: 0
        }; 1];
        let mut result_size = 0i32;

        let success = top_k_frequent_words(
            paragraph.as_ptr(),
            paragraph.len() as i32,
            1,
            result.as_mut_ptr(),
            &mut result_size,
        );

        assert!(!success); // Placeholder
    }

    #[test]
    fn test_top_k_frequent_edge_cases() {
        let mut result_size = 0i32;

        // Null paragraph
        let success = top_k_frequent_words(
            std::ptr::null(),
            0,
            1,
            std::ptr::null_mut(),
            &mut result_size,
        );
        assert!(!success);
        assert_eq!(result_size, 0);

        // K = 0
        let paragraph = "test";
        let success = top_k_frequent_words(
            paragraph.as_ptr(),
            paragraph.len() as i32,
            0,
            std::ptr::null_mut(),
            &mut result_size,
        );
        assert!(!success);
    }

    #[test]
    fn test_log_analysis_scenario() {
        // Simulate common log analysis scenario
        let log_snippet = "ERROR database ERROR network INFO login ERROR database INFO";
        let mut result = vec![WordFreq {
            word: std::ptr::null_mut(),
            word_len: 0,
            frequency: 0
        }; 3];
        let mut result_size = 0i32;

        let success = top_k_frequent_words(
            log_snippet.as_ptr(),
            log_snippet.len() as i32,
            3,
            result.as_mut_ptr(),
            &mut result_size,
        );

        // Expected: ERROR(3), INFO(2), database(2) or network(1)
        assert!(!success); // Placeholder test
    }

    #[test]
    fn test_heap_entry_ordering() {
        // Test internal heap ordering logic
        let entry1 = HeapEntry { word: "apple".to_string(), frequency: 3 };
        let entry2 = HeapEntry { word: "banana".to_string(), frequency: 3 };
        let entry3 = HeapEntry { word: "cherry".to_string(), frequency: 2 };

        // Same frequency: lexicographic order (apple < banana)
        assert!(entry1 > entry2); // Min-heap: smaller elements are "greater"

        // Different frequency: higher frequency is "smaller" in min-heap
        assert!(entry1 < entry3);
    }
}