use std::os::raw::c_char;

#[repr(C)]
pub struct IntArray {
    pub data: *mut i32,
    pub len: usize,
    pub capacity: usize,
}

#[repr(C)]
pub struct StringArray {
    pub data: *mut *mut c_char,
    pub len: usize,
    pub capacity: usize,
}

#[repr(C)]
pub struct ListNode {
    pub val: i32,
    pub next: *mut ListNode,
}

#[repr(C)]
pub struct TreeNode {
    pub val: i32,
    pub left: *mut TreeNode,
    pub right: *mut TreeNode,
}
