// Easy Problems
pub mod easy {
    pub mod hash_map {
        pub mod two_sum;
        pub use two_sum::*;
    }

    pub mod two_pointers {
        pub mod reverse_string;
        pub use reverse_string::*;
    }

    pub mod stack {
        pub mod valid_parentheses;
        pub use valid_parentheses::*;
    }

    pub mod binary_search {
        pub mod binary_search;
        pub use binary_search::*;
    }

    pub mod dynamic_programming {
        pub mod max_subarray;
        pub use max_subarray::*;
    }

    // Re-export all easy problems
    pub use binary_search::*;
    pub use dynamic_programming::*;
    pub use hash_map::*;
    pub use stack::*;
    pub use two_pointers::*;
}

// Medium Problems
pub mod medium {
    pub mod linked_list {
        pub mod add_two_numbers;
        pub use add_two_numbers::*;
    }

    pub mod sliding_window {
        pub mod longest_substring;
        pub mod rate_limiter;
        pub use longest_substring::*;
        pub use rate_limiter::*;
    }

    pub mod two_pointers {
        pub mod max_area;
        pub use max_area::*;
    }

    pub mod intervals {
        pub mod merge_intervals;
        pub use merge_intervals::*;
    }

    pub mod hash_map {
        pub mod group_anagrams;
        pub use group_anagrams::*;
    }

    pub mod heap {
        pub mod meeting_rooms;
        pub use meeting_rooms::*;
    }

    pub mod hash_map_heap {
        pub mod top_k_frequent_words;
        pub use top_k_frequent_words::*;
    }

    pub mod dfs_tree {
        pub mod directory_size_calculator;
        pub use directory_size_calculator::*;
    }

    pub mod graph_dfs {
        pub mod course_schedule;
        pub use course_schedule::*;
    }

    // Re-export all medium problems
    pub use dfs_tree::*;
    pub use graph_dfs::*;
    pub use hash_map::*;
    pub use hash_map_heap::*;
    pub use heap::*;
    pub use intervals::*;
    pub use linked_list::*;
    pub use sliding_window::*;
    pub use two_pointers::*;
}

// Hard Problems
pub mod hard {
    pub mod hash_map_linked_list {
        pub mod lru_cache;
        pub use lru_cache::*;
    }

    pub mod bfs_tree {
        pub mod level_order;
        pub use level_order::*;
    }

    // Re-export all hard problems
    pub use bfs_tree::*;
    pub use hash_map_linked_list::*;
}

// Re-export all problems at the top level for backward compatibility
pub use easy::*;
pub use hard::*;
pub use medium::*;
