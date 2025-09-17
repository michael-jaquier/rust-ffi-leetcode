use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

pub mod data_structures;
pub mod problems;

use data_structures::*;

// Re-export the problems for easier access
pub use problems::*;

// Memory management functions
#[no_mangle]
pub extern "C" fn int_array_new(capacity: usize) -> *mut IntArray {
    Box::into_raw(Box::new(IntArray {
        data: Vec::with_capacity(capacity).as_mut_ptr(),
        len: 0,
        capacity,
    }))
}

#[no_mangle]
pub extern "C" fn int_array_free(arr: *mut IntArray) {
    if !arr.is_null() {
        unsafe {
            let _ = Box::from_raw(arr);
        }
    }
}

#[no_mangle]
pub extern "C" fn string_array_new(capacity: usize) -> *mut StringArray {
    Box::into_raw(Box::new(StringArray {
        data: Vec::with_capacity(capacity).as_mut_ptr(),
        len: 0,
        capacity,
    }))
}

#[no_mangle]
pub extern "C" fn string_array_free(arr: *mut StringArray) {
    if !arr.is_null() {
        unsafe {
            let _ = Box::from_raw(arr);
        }
    }
}

#[no_mangle]
pub extern "C" fn list_node_new(val: i32) -> *mut ListNode {
    Box::into_raw(Box::new(ListNode {
        val,
        next: ptr::null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn list_free(head: *mut ListNode) {
    unsafe {
        let mut current = head;
        while !current.is_null() {
            let next = (*current).next;
            let _ = Box::from_raw(current);
            current = next;
        }
    }
}

#[no_mangle]
pub extern "C" fn tree_node_new(val: i32) -> *mut TreeNode {
    Box::into_raw(Box::new(TreeNode {
        val,
        left: ptr::null_mut(),
        right: ptr::null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn tree_free(root: *mut TreeNode) {
    if !root.is_null() {
        unsafe {
            let node = Box::from_raw(root);
            tree_free(node.left);
            tree_free(node.right);
        }
    }
}
