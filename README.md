# FFI LeetCode Practice Library

Rust library with C FFI bindings for practicing LeetCode problems with comprehensive algorithmic pattern explanations. Features educational problem sets with translation strategies from English descriptions to algorithms.

## Structure

```
ffi_leetcode/
├── src/
│   ├── lib.rs              # Main library and FFI exports
│   ├── data_structures.rs  # C-compatible data structures
│   └── problems/           # Individual problem implementations
│       ├── mod.rs
│       ├── two_sum.rs           # Hash map pattern
│       ├── reverse_string.rs    # Two pointers pattern
│       ├── valid_parentheses.rs # Stack pattern
│       ├── max_subarray.rs      # Dynamic programming pattern
│       ├── binary_search.rs     # Divide and conquer pattern
│       ├── merge_intervals.rs   # Sorting + greedy pattern
│       ├── add_two_numbers.rs   # Linked list manipulation
│       ├── course_schedule.rs   # Graph algorithms pattern
│       ├── longest_substring.rs # Sliding window pattern
│       ├── max_area.rs          # Two pointers optimization
│       ├── lru_cache.rs         # Hash map + LRU pattern
│       ├── group_anagrams.rs    # HashMap + sorting pattern
│       ├── level_order.rs       # BFS traversal pattern
│       └── meeting_rooms.rs     # Interval scheduling pattern
├── include/
│   └── ffi_leetcode.h      # C header file
├── examples/
│   └── c_client.c          # C client for testing
├── tests/
│   └── integration_tests.rs # Rust integration tests
└── Makefile                # Individual problem test targets
```

## Algorithm Patterns Covered

### Core Patterns with Learning Focus:
1. **Hash Map for O(1) Lookups** - Two Sum, LRU Cache
2. **Two Pointers Technique** - Reverse String, Container With Most Water
3. **Stack for Matching Pairs** - Valid Parentheses
4. **Dynamic Programming** - Maximum Subarray (Kadane's Algorithm)
5. **Binary Search / Divide & Conquer** - Binary Search, Search Insert Position
6. **Interval Problems** - Merge Intervals, Meeting Rooms
7. **Graph Algorithms** - Course Schedule (Topological Sort)
8. **Sliding Window** - Longest Substring Without Repeating Characters
9. **Tree Traversal** - Binary Tree Level Order Traversal

### Educational Features:
- **Translation Strategy**: How to convert English problem descriptions to algorithmic thinking
- **Pattern Recognition**: "This class of problems can be solved with XYZ when you need to..."
- **Multiple Approaches**: Brute force → Optimized with complexity analysis
- **Common Variations**: Extensions and related problems for each pattern

## Building & Testing

```bash
# Build the library
make build

# Run all tests
make test

# Test individual problems
make test-two-sum
make test-lru-cache
make test-binary-search

# Test problem categories
make test-new-problems      # Test pattern-based problems
make test-existing-problems # Test original problems

# Build and run C client
make run-c-client

# Format and check code
make format
make check
```

## Usage

Each problem includes:
- **Algorithm Pattern explanation** with when/why to use it
- **Translation Strategy** from English to algorithmic thinking
- **Multiple solution approaches** with complexity trade-offs
- **Comprehensive test cases** covering edge cases and variations
- **Hints and step-by-step guidance**

Example problem structure:
```rust
/// Algorithm Pattern: Hash Map for O(1) Lookups
/// This class of problems can be solved with HashMap when you need to:
/// - Track what you've seen before
/// - Find complements (target - current_value)
/// - Convert O(n²) brute force to O(n) with space trade-off
///
/// Translation Strategy:
/// English: "Find two numbers that add up to target"
/// Algorithm: For each number, check if (target - number) exists in what we've seen

#[no_mangle]
pub extern "C" fn two_sum(nums: *const i32, nums_size: i32, target: i32, result: *mut i32) -> bool {
    // TODO: Implement hash map approach with detailed hints
}

#[cfg(test)]
mod tests {
    // Comprehensive test cases covering all scenarios
}
```

## Memory Management

C-compatible memory management functions:
- `int_array_new()` / `int_array_free()`
- `string_array_new()` / `string_array_free()`
- `list_node_new()` / `list_free()`
- `tree_node_new()` / `tree_free()`

## Learning Approach

Problems are designed for algorithmic pattern learning:

1. **Pattern Recognition**: Learn to identify when to use specific algorithms
2. **Translation Skills**: Convert problem descriptions to algorithmic thinking
3. **Complexity Analysis**: Understand time/space trade-offs
4. **Edge Case Handling**: Comprehensive test coverage
5. **Multiple Solutions**: Compare brute force vs optimized approaches
