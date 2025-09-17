#ifndef FFI_LEETCODE_H
#define FFI_LEETCODE_H

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

// Common data structures
typedef struct {
    int32_t *data;
    size_t len;
    size_t capacity;
} IntArray;

typedef struct {
    char **data;
    size_t len;
    size_t capacity;
} StringArray;

typedef struct ListNode {
    int32_t val;
    struct ListNode *next;
} ListNode;

typedef struct TreeNode {
    int32_t val;
    struct TreeNode *left;
    struct TreeNode *right;
} TreeNode;

// Memory management
IntArray *int_array_new(size_t capacity);
void int_array_free(IntArray *arr);
StringArray *string_array_new(size_t capacity);
void string_array_free(StringArray *arr);
ListNode *list_node_new(int32_t val);
void list_free(ListNode *head);
TreeNode *tree_node_new(int32_t val);
void tree_free(TreeNode *root);

// Problem 1: Group Anagrams
// Hint: Use sorting or frequency counting. Consider hash map approach.
StringArray *group_anagrams(StringArray *strs);

// Problem 2: Longest Substring Without Repeating Characters
// Hint: Sliding window with hash set. Track max length seen so far.
int32_t length_of_longest_substring(const char *s);

// Problem 3: Add Two Numbers (Linked Lists)
// Hint: Simulate addition with carry. Handle different list lengths.
ListNode *add_two_numbers(ListNode *l1, ListNode *l2);

// Problem 4: Binary Tree Level Order Traversal
// Hint: Use BFS with queue. Track level boundaries.
IntArray **level_order(TreeNode *root, int32_t *return_size, int32_t **column_sizes);

// Problem 5: Container With Most Water
// Hint: Two pointers from ends. Move pointer with smaller height.
int32_t max_area(int32_t *height, int32_t height_size);

// Problem 6: Course Schedule (Topological Sort)
// Hint: Build adjacency list, use Kahn's algorithm or DFS cycle detection.
bool can_finish(int32_t num_courses, int32_t **prerequisites, int32_t prerequisites_size, int32_t *prerequisites_col_size);

// Problem 7: LRU Cache Operations
typedef struct LRUCache LRUCache;
// Hint: Hash map + doubly linked list. O(1) get/put operations.
LRUCache *lru_cache_new(int32_t capacity);
int32_t lru_cache_get(LRUCache *cache, int32_t key);
void lru_cache_put(LRUCache *cache, int32_t key, int32_t value);
void lru_cache_free(LRUCache *cache);

// Problem 8: Meeting Rooms II
// Hint: Sort intervals, use min-heap for end times.
int32_t min_meeting_rooms(int32_t **intervals, int32_t intervals_size, int32_t *intervals_col_size);

#ifdef __cplusplus
}
#endif

#endif // FFI_LEETCODE_H