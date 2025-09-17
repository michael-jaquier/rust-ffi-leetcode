# FFI LeetCode Practice Library

Rust library with C FFI bindings for practicing LeetCode problems with comprehensive algorithmic pattern explanations. Features educational problem sets with translation strategies from English descriptions to algorithms.

## Structure

```
ffi_leetcode/
├── src/
│   ├── lib.rs              # Main library and FFI exports
│   ├── data_structures.rs  # C-compatible data structures
│   └── problems/           # Problems organized by difficulty and algorithm
│       ├── mod.rs
│       ├── easy/
│       │   ├── hash_map/
│       │   │   └── two_sum.rs           # Hash map for O(1) lookups
│       │   ├── two_pointers/
│       │   │   └── reverse_string.rs    # Two pointers technique
│       │   ├── stack/
│       │   │   └── valid_parentheses.rs # Stack for matching pairs
│       │   ├── binary_search/
│       │   │   └── binary_search.rs     # Divide and conquer
│       │   └── dynamic_programming/
│       │       └── max_subarray.rs      # Kadane's algorithm
│       ├── medium/
│       │   ├── linked_list/
│       │   │   └── add_two_numbers.rs   # Linked list manipulation
│       │   ├── sliding_window/
│       │   │   └── longest_substring.rs # Sliding window technique
│       │   ├── two_pointers/
│       │   │   └── max_area.rs          # Two pointers optimization
│       │   ├── intervals/
│       │   │   └── merge_intervals.rs   # Sorting + greedy
│       │   ├── hash_map/
│       │   │   └── group_anagrams.rs    # HashMap + sorting
│       │   ├── heap/
│       │   │   └── meeting_rooms.rs     # Priority queue/heap
│       │   └── graph_dfs/
│       │       └── course_schedule.rs   # Graph cycle detection
│       └── hard/
│           ├── hash_map_linked_list/
│           │   └── lru_cache.rs         # LRU cache implementation
│           └── bfs_tree/
│               └── level_order.rs       # BFS tree traversal
├── include/
│   └── ffi_leetcode.h      # C header file
├── examples/
│   └── c_client.c          # C client for testing
├── tests/
│   └── integration_tests.rs # Rust integration tests
└── Makefile                # Individual problem test targets
```

## Algorithm Patterns Covered

### Problems Organized by Difficulty and Algorithm:

**EASY (5 problems)**:
- **Hash Map**: Two Sum - O(1) lookups and complements
- **Two Pointers**: Reverse String - In-place array manipulation
- **Stack**: Valid Parentheses - LIFO matching patterns
- **Binary Search**: Binary Search - Divide and conquer on sorted arrays
- **Dynamic Programming**: Maximum Subarray - Kadane's algorithm

**MEDIUM (7 problems)**:
- **Linked List**: Add Two Numbers - Pointer manipulation with carry
- **Sliding Window**: Longest Substring - Variable window optimization
- **Two Pointers**: Container With Most Water - Optimization technique
- **Intervals**: Merge Intervals - Sorting + greedy merging
- **Hash Map**: Group Anagrams - Frequency counting and grouping
- **Heap**: Meeting Rooms II - Priority queue scheduling
- **Graph/DFS**: Course Schedule - Cycle detection and topological sort

**HARD (2 problems)**:
- **Hash Map + Linked List**: LRU Cache - Combined data structure design
- **BFS/Tree**: Binary Tree Level Order - Breadth-first tree traversal

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
