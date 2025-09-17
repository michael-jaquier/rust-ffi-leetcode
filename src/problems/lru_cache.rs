use std::collections::HashMap;

/// Problem: LRU Cache
///
/// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
///
/// Implement the LRUCache class:
/// - LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
/// - int get(int key) Return the value of the key if the key exists, otherwise return -1.
/// - void put(int key, int value) Update the value of the key if the key exists.
///   Otherwise, add the key-value pair to the cache. If the number of keys exceeds
///   the capacity from this operation, evict the least recently used key.
///
/// The functions get and put must each run in O(1) average time complexity.
///
/// Example:
/// Input: ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
///        [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
/// Output: [null, null, null, 1, null, -1, null, -1, 3, 4]
///
/// Constraints:
/// - 1 <= capacity <= 3000
/// - 0 <= key <= 10^4
/// - 0 <= value <= 10^5
/// - At most 2 * 10^5 calls will be made to get and put.
///
/// Hints:
/// 1. Use a HashMap for O(1) key lookup
/// 2. Use a doubly-linked list to track usage order
/// 3. HashMap points to list nodes for O(1) removal
/// 4. Most recently used goes to head, LRU at tail
/// 5. For this exercise, Vec can simulate the doubly-linked list
///
/// Time Complexity: O(1) for both get and put operations
/// Space Complexity: O(capacity) for the cache storage

pub struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    usage_order: Vec<i32>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            usage_order: Vec::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        // TODO: Implement get operation
        //
        // Your implementation should:
        // 1. Check if key exists in cache HashMap
        // 2. If exists, move key to front of usage_order (most recent)
        // 3. Return the value
        // 4. If not exists, return -1

        // Placeholder - replace with your implementation
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // TODO: Implement put operation
        //
        // Your implementation should:
        // 1. If key already exists:
        //    a. Update value in HashMap
        //    b. Move key to front of usage_order
        // 2. If key doesn't exist:
        //    a. If at capacity, remove LRU (last in usage_order) from both structures
        //    b. Add new key-value to HashMap
        //    c. Add key to front of usage_order
        // 3. Helper: moving key to front means removing it from current position
        //    and inserting at index 0

        // Placeholder - replace with your implementation
    }
}

#[no_mangle]
pub extern "C" fn lru_cache_new(capacity: i32) -> *mut LRUCache {
    if capacity <= 0 {
        return std::ptr::null_mut();
    }

    Box::into_raw(Box::new(LRUCache::new(capacity as usize)))
}

#[no_mangle]
pub extern "C" fn lru_cache_get(cache: *mut LRUCache, key: i32) -> i32 {
    if cache.is_null() {
        return -1;
    }

    unsafe { (*cache).get(key) }
}

#[no_mangle]
pub extern "C" fn lru_cache_put(cache: *mut LRUCache, key: i32, value: i32) {
    if cache.is_null() {
        return;
    }

    unsafe {
        (*cache).put(key, value);
    }
}

#[no_mangle]
pub extern "C" fn lru_cache_free(cache: *mut LRUCache) {
    if !cache.is_null() {
        unsafe {
            let _ = Box::from_raw(cache);
        }
    }
}
