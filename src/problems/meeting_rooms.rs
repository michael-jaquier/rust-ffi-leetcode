/// Problem: Meeting Rooms II
///
/// Given an array of meeting time intervals where intervals[i] = [starti, endi],
/// return the minimum number of conference rooms required.
///
/// Example 1:
/// Input: intervals = [[0,30],[5,10],[15,20]]
/// Output: 2
/// Explanation: We need one room for [0,30] and another for [5,10] and [15,20].
/// Note that [5,10] and [15,20] can use the same room after [5,10] ends.
///
/// Example 2:
/// Input: intervals = [[7,10],[2,4]]
/// Output: 1
///
/// Constraints:
/// - 1 <= intervals.length <= 10^4
/// - 0 <= starti < endi <= 10^6
///
/// Hints:
/// 1. Think about what happens when meetings start and end
/// 2. Sort intervals by start time
/// 3. Use a min-heap to track end times of ongoing meetings
/// 4. For each meeting, remove ended meetings from heap
/// 5. Add current meeting's end time to heap
/// 6. Max heap size is the answer
///
/// Alternative approach:
/// 1. Create events for start (+1 room) and end (-1 room)
/// 2. Sort events by time (end events before start if same time)
/// 3. Sweep through events tracking current rooms needed
///
/// Time Complexity: O(n log n) for sorting
/// Space Complexity: O(n) for the heap/events

#[no_mangle]
pub extern "C" fn min_meeting_rooms(
    intervals: *const *const i32,
    intervals_size: i32,
    intervals_col_size: *const i32,
) -> i32 {
    if intervals.is_null() || intervals_size == 0 {
        return 0;
    }

    // TODO: Implement meeting rooms algorithm
    //
    // Your implementation should use one of these approaches:
    //
    // Approach 1 (Min-Heap):
    // 1. Sort intervals by start time
    // 2. Use BinaryHeap (or Vec as min-heap) to track end times
    // 3. For each interval:
    //    a. Remove all meetings that ended before current start
    //    b. Add current meeting's end time to heap
    //    c. Update max heap size seen so far
    //
    // Approach 2 (Event Sweep):
    // 1. Create (time, event_type) pairs: (start, +1), (end, -1)
    // 2. Sort events (end before start if same time)
    // 3. Sweep through events:
    //    a. Add/subtract rooms needed
    //    b. Track maximum rooms needed
    //
    // Note: intervals[i] points to array [start, end]

    // Placeholder - replace with your implementation
    0
}
