/// Problem: Merge Intervals
///
/// Given an array of intervals where intervals[i] = [start_i, end_i], merge all overlapping
/// intervals, and return an array of the non-overlapping intervals that cover all the intervals
/// in the input.
///
/// Example 1:
/// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
/// Output: [[1,6],[8,10],[15,18]]
/// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
///
/// Example 2:
/// Input: intervals = [[1,4],[4,5]]
/// Output: [[1,5]]
/// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
///
/// Constraints:
/// - 1 <= intervals.length <= 10^4
/// - intervals[i].length == 2
/// - 0 <= start_i <= end_i <= 10^4
///
/// Algorithm Pattern: Interval Problems / Sorting + Greedy
/// This class of problems can be solved with Sorting + Greedy when you need to:
/// - Merge, insert, or remove overlapping intervals
/// - Find maximum non-overlapping intervals
/// - Schedule meetings or events
/// - Process ranges or time periods
///
/// Translation Strategy:
/// English: "Merge overlapping time periods"
/// Algorithm: Sort by start time, then greedily merge consecutive overlapping intervals
///
/// Common Interval Patterns:
/// 1. Merge intervals (this problem)
/// 2. Insert interval into sorted list
/// 3. Remove covered intervals
/// 4. Find minimum meeting rooms needed
/// 5. Non-overlapping intervals (activity selection)
/// 6. Employee free time
///
/// Key Insight: Sort first, then process in order to make decisions easier
///
/// Hints:
/// 1. Sort intervals by start time
/// 2. Initialize result with first interval
/// 3. For each subsequent interval:
///    a. If it overlaps with last interval in result: merge them
///    b. Otherwise: add it as new interval to result
/// 4. Two intervals [a,b] and [c,d] overlap if b >= c (assuming a <= c after sorting)
/// 5. Merged interval is [min(a,c), max(b,d)] = [a, max(b,d)] after sorting
///
/// Time Complexity: O(n log n) - dominated by sorting
/// Space Complexity: O(n) - for result array (or O(log n) if sorting in-place)

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

#[no_mangle]
pub extern "C" fn merge_intervals(
    intervals: *mut Interval,
    intervals_size: i32,
    result_size: *mut i32
) -> *mut Interval {
    if intervals.is_null() || result_size.is_null() || intervals_size <= 0 {
        unsafe { *result_size = 0; }
        return std::ptr::null_mut();
    }

    let intervals_slice = unsafe { std::slice::from_raw_parts_mut(intervals, intervals_size as usize) };

    // TODO: Implement interval merging algorithm
    //
    // Your implementation should:
    // 1. Sort intervals by start time: intervals.sort_by(|a, b| a.start.cmp(&b.start))
    // 2. Create result vector: let mut merged = Vec::new()
    // 3. Add first interval to result: merged.push(intervals[0])
    // 4. For each remaining interval:
    //    a. Get last interval from result: let last = merged.last_mut().unwrap()
    //    b. If current.start <= last.end: // overlapping
    //       - Merge: last.end = last.end.max(current.end)
    //    c. Else: // non-overlapping
    //       - Add as new interval: merged.push(current)
    // 5. Convert result to C array and set result_size
    //
    // Example walkthrough with [[1,3],[2,6],[8,10],[15,18]]:
    // - After sorting: [[1,3],[2,6],[8,10],[15,18]]
    // - Start with [1,3]
    // - [2,6]: 2 <= 3, so merge → [1,6]
    // - [8,10]: 8 > 6, so add new → [1,6],[8,10]
    // - [15,18]: 15 > 10, so add new → [1,6],[8,10],[15,18]
    //
    // Alternative approaches:
    // - Brute force: Check all pairs O(n²)
    // - Sort + merge: O(n log n) optimal approach
    // - Union-Find: Overkill for this problem

    // Placeholder - replace with your implementation
    unsafe { *result_size = 0; }
    std::ptr::null_mut()
}

/// Bonus: Insert new interval into sorted non-overlapping intervals
#[no_mangle]
pub extern "C" fn insert_interval(
    intervals: *const Interval,
    intervals_size: i32,
    new_interval: Interval,
    result_size: *mut i32
) -> *mut Interval {
    if intervals.is_null() || result_size.is_null() {
        unsafe { *result_size = 0; }
        return std::ptr::null_mut();
    }

    // TODO: Insert and merge new interval into existing sorted intervals
    // Hint: Add intervals before new_interval, merge overlapping ones, add remaining

    unsafe { *result_size = 0; }
    std::ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals_basic() {
        let mut intervals = vec![
            Interval { start: 1, end: 3 },
            Interval { start: 2, end: 6 },
            Interval { start: 8, end: 10 },
            Interval { start: 15, end: 18 },
        ];
        let mut result_size = 0i32;

        let result = merge_intervals(intervals.as_mut_ptr(), intervals.len() as i32, &mut result_size);

        // Expected: [[1,6],[8,10],[15,18]]
        assert_eq!(result_size, 3);
        if !result.is_null() {
            unsafe {
                let result_slice = std::slice::from_raw_parts(result, result_size as usize);
                // Should have merged [1,3] and [2,6] into [1,6]
                assert_eq!(result_slice[0].start, 1);
                assert_eq!(result_slice[0].end, 6);
                assert_eq!(result_slice[1].start, 8);
                assert_eq!(result_slice[1].end, 10);
                assert_eq!(result_slice[2].start, 15);
                assert_eq!(result_slice[2].end, 18);
            }
        }
    }

    #[test]
    fn test_merge_intervals_touching() {
        let mut intervals = vec![
            Interval { start: 1, end: 4 },
            Interval { start: 4, end: 5 },
        ];
        let mut result_size = 0i32;

        let result = merge_intervals(intervals.as_mut_ptr(), intervals.len() as i32, &mut result_size);

        // Expected: [[1,5]] - touching intervals should merge
        assert_eq!(result_size, 1);
        if !result.is_null() {
            unsafe {
                let result_slice = std::slice::from_raw_parts(result, result_size as usize);
                assert_eq!(result_slice[0].start, 1);
                assert_eq!(result_slice[0].end, 5);
            }
        }
    }

    #[test]
    fn test_merge_intervals_single() {
        let mut intervals = vec![Interval { start: 1, end: 4 }];
        let mut result_size = 0i32;

        let result = merge_intervals(intervals.as_mut_ptr(), intervals.len() as i32, &mut result_size);

        // Expected: [[1,4]]
        assert_eq!(result_size, 1);
        if !result.is_null() {
            unsafe {
                let result_slice = std::slice::from_raw_parts(result, result_size as usize);
                assert_eq!(result_slice[0].start, 1);
                assert_eq!(result_slice[0].end, 4);
            }
        }
    }

    #[test]
    fn test_merge_intervals_no_overlap() {
        let mut intervals = vec![
            Interval { start: 1, end: 2 },
            Interval { start: 3, end: 4 },
            Interval { start: 5, end: 6 },
        ];
        let mut result_size = 0i32;

        let result = merge_intervals(intervals.as_mut_ptr(), intervals.len() as i32, &mut result_size);

        // Expected: [[1,2],[3,4],[5,6]] - no merging
        assert_eq!(result_size, 3);
    }

    #[test]
    fn test_merge_intervals_all_merge() {
        let mut intervals = vec![
            Interval { start: 1, end: 4 },
            Interval { start: 2, end: 3 },
            Interval { start: 3, end: 6 },
            Interval { start: 5, end: 8 },
        ];
        let mut result_size = 0i32;

        let result = merge_intervals(intervals.as_mut_ptr(), intervals.len() as i32, &mut result_size);

        // Expected: [[1,8]] - all intervals should merge into one
        assert_eq!(result_size, 1);
        if !result.is_null() {
            unsafe {
                let result_slice = std::slice::from_raw_parts(result, result_size as usize);
                assert_eq!(result_slice[0].start, 1);
                assert_eq!(result_slice[0].end, 8);
            }
        }
    }

    #[test]
    fn test_merge_intervals_edge_cases() {
        let mut result_size = 0i32;

        // Empty array
        let result = merge_intervals(std::ptr::null_mut(), 0, &mut result_size);
        assert_eq!(result_size, 0);
        assert!(result.is_null());

        // Null pointer
        let result = merge_intervals(std::ptr::null_mut(), -1, &mut result_size);
        assert_eq!(result_size, 0);
        assert!(result.is_null());
    }

    #[test]
    fn test_insert_interval_basic() {
        let intervals = vec![
            Interval { start: 1, end: 3 },
            Interval { start: 6, end: 9 },
        ];
        let new_interval = Interval { start: 2, end: 5 };
        let mut result_size = 0i32;

        let result = insert_interval(intervals.as_ptr(), intervals.len() as i32, new_interval, &mut result_size);

        // Expected: [[1,5],[6,9]]
        assert_eq!(result_size, 2);
        if !result.is_null() {
            unsafe {
                let result_slice = std::slice::from_raw_parts(result, result_size as usize);
                assert_eq!(result_slice[0].start, 1);
                assert_eq!(result_slice[0].end, 5);
                assert_eq!(result_slice[1].start, 6);
                assert_eq!(result_slice[1].end, 9);
            }
        }
    }

    #[test]
    fn test_insert_interval_no_overlap() {
        let intervals = vec![
            Interval { start: 1, end: 2 },
            Interval { start: 4, end: 5 },
        ];
        let new_interval = Interval { start: 3, end: 3 };
        let mut result_size = 0i32;

        let result = insert_interval(intervals.as_ptr(), intervals.len() as i32, new_interval, &mut result_size);

        // Expected: [[1,2],[3,3],[4,5]]
        assert_eq!(result_size, 3);
    }

    #[test]
    fn test_insert_interval_edge_cases() {
        let new_interval = Interval { start: 1, end: 2 };
        let mut result_size = 0i32;

        // Empty intervals
        let result = insert_interval(std::ptr::null(), 0, new_interval, &mut result_size);
        assert_eq!(result_size, 0);
        assert!(result.is_null());

        // Null pointer
        let result = insert_interval(std::ptr::null(), -1, new_interval, &mut result_size);
        assert_eq!(result_size, 0);
        assert!(result.is_null());
    }
}