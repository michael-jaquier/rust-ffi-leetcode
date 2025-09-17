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