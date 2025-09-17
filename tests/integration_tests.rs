use ffi_leetcode::*;
use std::ptr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_basic() {
        // Test cases for longest substring without repeating characters
        let test_cases = vec![
            ("abcabcbb", 3), // "abc"
            ("bbbbb", 1),    // "b"
            ("pwwkew", 3),   // "wke"
            ("", 0),         // empty string
            ("dvdf", 3),     // "vdf"
        ];

        for (input, expected) in test_cases {
            let c_string = std::ffi::CString::new(input).unwrap();
            let result = unsafe { length_of_longest_substring(c_string.as_ptr()) };
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_max_area_basic() {
        let height1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result1 = unsafe { max_area(height1.as_ptr(), height1.len() as i32) };
        assert_eq!(result1, 49);

        let height2 = vec![1, 1];
        let result2 = unsafe { max_area(height2.as_ptr(), height2.len() as i32) };
        assert_eq!(result2, 1);

        let height3 = vec![4, 3, 2, 1, 4];
        let result3 = unsafe { max_area(height3.as_ptr(), height3.len() as i32) };
        assert_eq!(result3, 16); // indices 0 and 4: min(4,4) * (4-0) = 16
    }

    #[test]
    fn test_add_two_numbers_basic() {
        // Create first list: [2,4,3] representing 342
        let l1 = unsafe {
            let node1 = list_node_new(2);
            let node2 = list_node_new(4);
            let node3 = list_node_new(3);
            (*node1).next = node2;
            (*node2).next = node3;
            node1
        };

        // Create second list: [5,6,4] representing 465
        let l2 = unsafe {
            let node1 = list_node_new(5);
            let node2 = list_node_new(6);
            let node3 = list_node_new(4);
            (*node1).next = node2;
            (*node2).next = node3;
            node1
        };

        let result = unsafe { add_two_numbers(l1, l2) };

        // Verify result: [7,0,8] representing 807
        unsafe {
            assert!(!result.is_null());
            assert_eq!((*result).val, 7);
            assert!(!(*result).next.is_null());
            assert_eq!((*(*result).next).val, 0);
            assert!(!(*(*result).next).next.is_null());
            assert_eq!((*(*(*result).next).next).val, 8);

            // Clean up
            list_free(l1);
            list_free(l2);
            list_free(result);
        }
    }

    #[test]
    fn test_lru_cache_basic() {
        let cache = unsafe { lru_cache_new(2) };
        assert!(!cache.is_null());

        unsafe {
            // Test basic put/get
            lru_cache_put(cache, 1, 1);
            lru_cache_put(cache, 2, 2);
            assert_eq!(lru_cache_get(cache, 1), 1);

            // Test eviction
            lru_cache_put(cache, 3, 3); // Should evict key 2
            assert_eq!(lru_cache_get(cache, 2), -1);
            assert_eq!(lru_cache_get(cache, 3), 3);
            assert_eq!(lru_cache_get(cache, 1), 1);

            lru_cache_free(cache);
        }
    }

    #[test]
    fn test_memory_management() {
        // Test that we can create and free data structures without crashes
        let int_arr = unsafe { int_array_new(10) };
        assert!(!int_arr.is_null());
        unsafe {
            int_array_free(int_arr);
        }

        let str_arr = unsafe { string_array_new(5) };
        assert!(!str_arr.is_null());
        unsafe {
            string_array_free(str_arr);
        }

        let list_node = unsafe { list_node_new(42) };
        assert!(!list_node.is_null());
        unsafe {
            assert_eq!((*list_node).val, 42);
            assert!((*list_node).next.is_null());
            list_free(list_node);
        }

        let tree_node = unsafe { tree_node_new(100) };
        assert!(!tree_node.is_null());
        unsafe {
            assert_eq!((*tree_node).val, 100);
            assert!((*tree_node).left.is_null());
            assert!((*tree_node).right.is_null());
            tree_free(tree_node);
        }
    }

    #[test]
    fn test_course_schedule_basic() {
        // Test case 1: Can finish - no cycle
        let prereq1 = [0i32, 1i32];
        let prereqs1 = [prereq1.as_ptr()];
        let col_sizes1 = [2i32];

        let result1 = unsafe { can_finish(2, prereqs1.as_ptr(), 1, col_sizes1.as_ptr()) };
        assert_eq!(result1, true);

        // Test case 2: Cannot finish - cycle exists
        let prereq2a = [1i32, 0i32];
        let prereq2b = [0i32, 1i32];
        let prereqs2 = [prereq2a.as_ptr(), prereq2b.as_ptr()];
        let col_sizes2 = [2i32, 2i32];

        let result2 = unsafe { can_finish(2, prereqs2.as_ptr(), 2, col_sizes2.as_ptr()) };
        assert_eq!(result2, false);
    }

    #[test]
    fn test_meeting_rooms_basic() {
        // Test case 1: [[0,30],[5,10],[15,20]] -> 2 rooms needed
        let interval1 = [0i32, 30i32];
        let interval2 = [5i32, 10i32];
        let interval3 = [15i32, 20i32];
        let intervals1 = [interval1.as_ptr(), interval2.as_ptr(), interval3.as_ptr()];
        let col_sizes1 = [2i32, 2i32, 2i32];

        let result1 = unsafe { min_meeting_rooms(intervals1.as_ptr(), 3, col_sizes1.as_ptr()) };
        assert_eq!(result1, 2);

        // Test case 2: [[7,10],[2,4]] -> 1 room needed
        let interval4 = [7i32, 10i32];
        let interval5 = [2i32, 4i32];
        let intervals2 = [interval4.as_ptr(), interval5.as_ptr()];
        let col_sizes2 = [2i32, 2i32];

        let result2 = unsafe { min_meeting_rooms(intervals2.as_ptr(), 2, col_sizes2.as_ptr()) };
        assert_eq!(result2, 1);
    }
}
