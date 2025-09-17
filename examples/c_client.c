#include <stdio.h>
#include <string.h>
#include <assert.h>
#include "ffi_leetcode.h"

void test_group_anagrams() {
    printf("Testing group_anagrams...\n");

    StringArray *input = string_array_new(4);
    // You'll need to populate this based on your implementation

    StringArray *result = group_anagrams(input);

    // Add your assertions here
    printf("✓ Group anagrams test passed\n");

    string_array_free(input);
    string_array_free(result);
}

void test_longest_substring() {
    printf("Testing length_of_longest_substring...\n");

    int32_t result1 = length_of_longest_substring("abcabcbb");
    assert(result1 == 3); // "abc"

    int32_t result2 = length_of_longest_substring("bbbbb");
    assert(result2 == 1); // "b"

    int32_t result3 = length_of_longest_substring("pwwkew");
    assert(result3 == 3); // "wke"

    printf("✓ Longest substring test passed\n");
}

void test_add_two_numbers() {
    printf("Testing add_two_numbers...\n");

    // Create lists: [2,4,3] + [5,6,4] = [7,0,8]
    ListNode *l1 = list_node_new(2);
    l1->next = list_node_new(4);
    l1->next->next = list_node_new(3);

    ListNode *l2 = list_node_new(5);
    l2->next = list_node_new(6);
    l2->next->next = list_node_new(4);

    ListNode *result = add_two_numbers(l1, l2);

    assert(result->val == 7);
    assert(result->next->val == 0);
    assert(result->next->next->val == 8);

    printf("✓ Add two numbers test passed\n");

    list_free(l1);
    list_free(l2);
    list_free(result);
}

void test_max_area() {
    printf("Testing max_area...\n");

    int32_t height1[] = {1,8,6,2,5,4,8,3,7};
    int32_t result1 = max_area(height1, 9);
    assert(result1 == 49);

    int32_t height2[] = {1,1};
    int32_t result2 = max_area(height2, 2);
    assert(result2 == 1);

    printf("✓ Max area test passed\n");
}

void test_lru_cache() {
    printf("Testing LRU Cache...\n");

    LRUCache *cache = lru_cache_new(2);

    lru_cache_put(cache, 1, 1);
    lru_cache_put(cache, 2, 2);
    assert(lru_cache_get(cache, 1) == 1);

    lru_cache_put(cache, 3, 3); // evicts key 2
    assert(lru_cache_get(cache, 2) == -1);

    printf("✓ LRU Cache test passed\n");

    lru_cache_free(cache);
}

int main() {
    printf("Running FFI LeetCode Tests\n");
    printf("==========================\n");

    // Uncomment as you implement each function
    // test_group_anagrams();
    test_longest_substring();
    test_add_two_numbers();
    test_max_area();
    test_lru_cache();

    printf("\nAll tests passed! 🎉\n");
    return 0;
}