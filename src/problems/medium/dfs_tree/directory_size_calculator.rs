/// Problem: Directory Size Calculator (Common in Tech Company Interviews)
///
/// Given a root directory, find the total size of all the files across all sub-directories.
/// This is a common tech interview question that tests file system traversal and recursion.
///
/// The function should:
/// 1. Traverse all subdirectories recursively
/// 2. Sum up the sizes of all files
/// 3. Handle edge cases (empty directories, permission errors, etc.)
/// 4. Return total size in bytes
///
/// Example Directory Structure:
/// ```
/// /root
/// ├── file1.txt (100 bytes)
/// ├── subdir1/
/// │   ├── file2.txt (200 bytes)
/// │   └── file3.txt (300 bytes)
/// └── subdir2/
///     └── nested/
///         └── file4.txt (400 bytes)
/// ```
/// Total size: 100 + 200 + 300 + 400 = 1000 bytes
///
/// Constraints:
/// - Directory paths can be nested arbitrarily deep
/// - Some files may be inaccessible (permission errors)
/// - Directory may contain symbolic links (bonus: handle cycles)
/// - Very large directories (performance considerations)
///
/// Algorithm Pattern: Tree Traversal / DFS
/// This class of problems can be solved with DFS when you need to:
/// - Traverse hierarchical structures (files, directories)
/// - Process all nodes in a tree/graph
/// - Accumulate values across recursive calls
/// - Handle cycles in graphs (with visited tracking)
///
/// Translation Strategy:
/// English: "Sum all file sizes in directory tree"
/// Algorithm: DFS through directories, accumulating file sizes
///
/// Real-World Context:
/// This mirrors real monitoring tasks:
/// - Disk usage monitoring and alerting
/// - Log file size tracking
/// - Storage capacity planning
/// - File system health checks
/// - Container storage monitoring
///
/// Key Insights:
/// 1. Recursive tree traversal pattern
/// 2. Error handling for inaccessible files
/// 3. Distinguish files from directories
/// 4. Memory efficiency for large file systems
/// 5. Optional: cycle detection for symlinks
///
/// Time Complexity: O(N) where N is total number of files and directories
/// Space Complexity: O(D) where D is maximum directory depth (recursion stack)

use std::collections::HashSet;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DirectorySizeResult {
    pub total_size_bytes: i64,
    pub file_count: i32,
    pub directory_count: i32,
    pub error_count: i32, // Files that couldn't be accessed
}

#[no_mangle]
pub extern "C" fn calculate_directory_size(
    root_path: *const u8,
    root_path_len: i32,
    result: *mut DirectorySizeResult,
) -> bool {
    if root_path.is_null() || result.is_null() || root_path_len <= 0 {
        return false;
    }

    let path_bytes = unsafe { std::slice::from_raw_parts(root_path, root_path_len as usize) };
    let path_str = match std::str::from_utf8(path_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // TODO: Implement directory traversal and size calculation
    //
    // Your implementation should:
    // 1. Use std::fs::read_dir() to list directory contents
    // 2. For each entry:
    //    a. If it's a file: add its size to total
    //    b. If it's a directory: recursively calculate its size
    // 3. Handle errors gracefully (increment error_count)
    // 4. Track statistics (file_count, directory_count)
    // 5. Bonus: Handle symbolic links and detect cycles
    //
    // Algorithm structure:
    // ```rust
    // fn traverse_directory(path: &Path, visited: &mut HashSet<PathBuf>) -> DirectorySizeResult {
    //     let mut result = DirectorySizeResult::new();
    //
    //     match std::fs::read_dir(path) {
    //         Ok(entries) => {
    //             for entry in entries {
    //                 match entry {
    //                     Ok(dir_entry) => {
    //                         let path = dir_entry.path();
    //                         let metadata = dir_entry.metadata();
    //
    //                         if metadata.is_file() {
    //                             result.total_size_bytes += metadata.len();
    //                             result.file_count += 1;
    //                         } else if metadata.is_dir() {
    //                             // Recursive call
    //                             let sub_result = traverse_directory(&path, visited);
    //                             result.merge(sub_result);
    //                         }
    //                     }
    //                     Err(_) => result.error_count += 1,
    //                 }
    //             }
    //         }
    //         Err(_) => result.error_count += 1,
    //     }
    //
    //     result
    // }
    // ```
    //
    // Tech Interview Context:
    // This problem type appears in monitoring scenarios:
    // - Disk usage alerts when directories exceed thresholds
    // - Log rotation based on total log directory size
    // - Container storage monitoring
    // - File system health checks
    // - Storage capacity planning
    //
    // Error Handling:
    // - Permission denied: increment error_count, continue
    // - Invalid paths: return false immediately
    // - Broken symlinks: increment error_count, continue
    // - I/O errors: increment error_count, continue

    // Placeholder implementation
    unsafe {
        *result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };
    }
    false
}

/// Bonus: Calculate size with symlink cycle detection
#[no_mangle]
pub extern "C" fn calculate_directory_size_safe(
    root_path: *const u8,
    root_path_len: i32,
    follow_symlinks: bool,
    result: *mut DirectorySizeResult,
) -> bool {
    if root_path.is_null() || result.is_null() {
        return false;
    }

    // TODO: Implement with cycle detection using visited set
    // Track canonical paths to detect symlink cycles
    // Use std::fs::canonicalize() to resolve symlinks

    unsafe {
        *result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };
    }
    false
}

/// Get directory statistics without calculating total size (faster)
#[no_mangle]
pub extern "C" fn get_directory_stats(
    root_path: *const u8,
    root_path_len: i32,
    max_depth: i32,
    result: *mut DirectorySizeResult,
) -> bool {
    // TODO: Implement depth-limited traversal
    // Useful for large directories where full traversal is too slow
    // Only traverse up to max_depth levels deep

    if result.is_null() {
        return false;
    }

    unsafe {
        *result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn create_test_directory() -> std::path::PathBuf {
        // Create a temporary test directory structure
        let temp_dir = std::env::temp_dir().join("tech_test_dir");

        // Clean up if exists
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
        }

        // Create test structure
        fs::create_dir_all(&temp_dir).expect("Failed to create test directory");

        // Create files and subdirectories for testing
        fs::write(temp_dir.join("file1.txt"), "Hello").expect("Failed to create file1");

        let subdir = temp_dir.join("subdir");
        fs::create_dir_all(&subdir).expect("Failed to create subdir");
        fs::write(subdir.join("file2.txt"), "World!").expect("Failed to create file2");

        temp_dir
    }

    fn cleanup_test_directory(path: &Path) {
        if path.exists() {
            let _ = fs::remove_dir_all(path);
        }
    }

    #[test]
    fn test_directory_size_basic() {
        let test_dir = create_test_directory();
        let path_str = test_dir.to_string_lossy();
        let mut result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };

        let success = calculate_directory_size(
            path_str.as_ptr(),
            path_str.len() as i32,
            &mut result,
        );

        // With placeholder implementation, should return false
        assert!(!success);
        assert_eq!(result.total_size_bytes, 0);

        cleanup_test_directory(&test_dir);
    }

    #[test]
    fn test_directory_size_nonexistent() {
        let path_str = "/nonexistent/directory/path";
        let mut result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };

        let success = calculate_directory_size(
            path_str.as_ptr(),
            path_str.len() as i32,
            &mut result,
        );

        // Should return false for nonexistent path
        assert!(!success);
    }

    #[test]
    fn test_directory_size_empty_directory() {
        let temp_dir = std::env::temp_dir().join("empty_test_dir");
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
        }
        fs::create_dir_all(&temp_dir).expect("Failed to create empty test directory");

        let path_str = temp_dir.to_string_lossy();
        let mut result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };

        let success = calculate_directory_size(
            path_str.as_ptr(),
            path_str.len() as i32,
            &mut result,
        );

        // Should handle empty directory
        assert!(!success); // Placeholder implementation

        cleanup_test_directory(&temp_dir);
    }

    #[test]
    fn test_directory_size_with_symlinks() {
        let test_dir = create_test_directory();
        let path_str = test_dir.to_string_lossy();
        let mut result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };

        let success = calculate_directory_size_safe(
            path_str.as_ptr(),
            path_str.len() as i32,
            true, // Follow symlinks
            &mut result,
        );

        assert!(!success); // Placeholder

        cleanup_test_directory(&test_dir);
    }

    #[test]
    fn test_directory_stats_depth_limited() {
        let test_dir = create_test_directory();
        let path_str = test_dir.to_string_lossy();
        let mut result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };

        let success = get_directory_stats(
            path_str.as_ptr(),
            path_str.len() as i32,
            2, // Max depth
            &mut result,
        );

        assert!(!success); // Placeholder

        cleanup_test_directory(&test_dir);
    }

    #[test]
    fn test_edge_cases() {
        let mut result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };

        // Null path
        assert!(!calculate_directory_size(std::ptr::null(), 0, &mut result));

        // Empty path
        assert!(!calculate_directory_size(b"".as_ptr(), 0, &mut result));

        // Null result
        assert!(!calculate_directory_size(b"/tmp".as_ptr(), 4, std::ptr::null_mut()));
    }

    #[test]
    fn test_monitoring_scenario() {
        // Simulate monitoring agent checking log directory size
        let log_dir = std::env::temp_dir().join("tech_logs");
        if log_dir.exists() {
            let _ = fs::remove_dir_all(&log_dir);
        }

        // Create simulated log structure
        fs::create_dir_all(&log_dir).expect("Failed to create log dir");

        // Create various log files
        fs::write(log_dir.join("agent.log"), "A".repeat(1000)).expect("Failed to create agent.log");
        fs::write(log_dir.join("trace.log"), "B".repeat(2000)).expect("Failed to create trace.log");

        let archived_dir = log_dir.join("archived");
        fs::create_dir_all(&archived_dir).expect("Failed to create archived dir");
        fs::write(archived_dir.join("old.log.gz"), "C".repeat(500)).expect("Failed to create old.log.gz");

        let path_str = log_dir.to_string_lossy();
        let mut result = DirectorySizeResult {
            total_size_bytes: 0,
            file_count: 0,
            directory_count: 0,
            error_count: 0,
        };

        let success = calculate_directory_size(
            path_str.as_ptr(),
            path_str.len() as i32,
            &mut result,
        );

        // Expected: 3500 bytes total (1000 + 2000 + 500)
        assert!(!success); // Placeholder implementation

        cleanup_test_directory(&log_dir);
    }
}