/// Problem: Design a Rate Limiter (Common in Tech Company Interviews)
///
/// Design a rate limiter that limits the number of requests that can be processed within
/// a given time window. This is a common system design question adapted for coding.
///
/// The rate limiter should support:
/// 1. allow_request(user_id, current_time) -> bool
/// 2. Different rate limiting algorithms
/// 3. Configurable rate limits per user
///
/// Example Usage:
/// - Rate limit: 5 requests per 60 seconds
/// - User A makes 3 requests at time 0, 10, 20 -> all allowed
/// - User A makes 3 more requests at time 30, 40, 50 -> first 2 allowed, last rejected
/// - At time 70, User A can make requests again (window reset)
///
/// Constraints:
/// - Support multiple users simultaneously
/// - Handle high-throughput scenarios
/// - Memory efficient
/// - Thread-safe (bonus)
///
/// Algorithm Pattern: Sliding Window / Token Bucket
/// This class of problems can be solved when you need to:
/// - Control resource access over time
/// - Implement fair queuing systems
/// - Prevent abuse/DoS attacks
/// - Manage API throttling
///
/// Translation Strategy:
/// English: "Allow max N requests per time window"
/// Algorithm: Track request timestamps and clean expired entries
///
/// Real-World Context:
/// Rate limiting is crucial for:
/// - API endpoint protection
/// - Metrics ingestion throttling
/// - Alert notification limiting
/// - Resource usage management
/// - Cost control and fair usage
///
/// Common Rate Limiting Algorithms:
/// 1. Fixed Window: Simple, but can have burst issues
/// 2. Sliding Window Log: Accurate, but memory intensive
/// 3. Sliding Window Counter: Good balance of accuracy and efficiency
/// 4. Token Bucket: Smooth rate limiting, allows bursts
/// 5. Leaky Bucket: Strict rate limiting, no bursts
///
/// Time Complexity: O(log N) per request (with cleanup)
/// Space Complexity: O(N * M) where N=users, M=requests per window

use std::collections::{HashMap, VecDeque};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RateLimiterConfig {
    pub max_requests: i32,      // Maximum requests allowed
    pub window_size_seconds: i32, // Time window in seconds
    pub algorithm: i32,         // 0=Fixed Window, 1=Sliding Window, 2=Token Bucket
}

// Opaque handle for C interface
#[repr(C)]
pub struct RateLimiter {
    config: RateLimiterConfig,
    // Internal state - using HashMap for simplicity
    // In production: use more efficient data structures
    user_windows: HashMap<i32, VecDeque<i64>>, // user_id -> timestamps
    last_cleanup: i64,
}

#[no_mangle]
pub extern "C" fn rate_limiter_new(config: RateLimiterConfig) -> *mut RateLimiter {
    let limiter = RateLimiter {
        config,
        user_windows: HashMap::new(),
        last_cleanup: 0,
    };
    Box::into_raw(Box::new(limiter))
}

#[no_mangle]
pub extern "C" fn rate_limiter_free(limiter: *mut RateLimiter) {
    if !limiter.is_null() {
        unsafe {
            let _ = Box::from_raw(limiter);
        }
    }
}

#[no_mangle]
pub extern "C" fn rate_limiter_allow_request(
    limiter: *mut RateLimiter,
    user_id: i32,
    current_time_seconds: i64,
) -> bool {
    if limiter.is_null() {
        return false;
    }

    let limiter_ref = unsafe { &mut *limiter };

    // TODO: Implement sliding window rate limiting
    //
    // Your implementation should:
    // 1. Get or create request window for user
    // 2. Clean up expired timestamps (older than window_size)
    // 3. Check if current request count < max_requests
    // 4. If allowed, add current timestamp and return true
    // 5. If rejected, return false
    //
    // Sliding Window Algorithm:
    // - Keep timestamps of all requests in current window
    // - On each request: remove expired timestamps
    // - Count remaining timestamps vs limit
    // - Add new timestamp if under limit
    //
    // Tech Interview Context:
    // This pattern is used for:
    // - API rate limiting (requests per minute/hour)
    // - Metrics ingestion throttling
    // - Alert notification rate control
    // - Log processing backpressure
    //
    // Production considerations:
    // - Use more memory-efficient data structures
    // - Implement background cleanup
    // - Add monitoring/metrics
    // - Handle clock skew
    // - Thread safety with locks/atomic operations

    // Placeholder implementation
    false
}

/// Token Bucket implementation (alternative algorithm)
#[no_mangle]
pub extern "C" fn rate_limiter_allow_request_token_bucket(
    limiter: *mut RateLimiter,
    user_id: i32,
    current_time_seconds: i64,
) -> bool {
    if limiter.is_null() {
        return false;
    }

    // TODO: Implement Token Bucket algorithm
    // Algorithm:
    // 1. Each user has a bucket with tokens
    // 2. Tokens are added at a fixed rate (refill_rate)
    // 3. Each request consumes 1 token
    // 4. If no tokens available, request is rejected
    // 5. Allows bursts up to bucket capacity
    //
    // Benefits:
    // - Smooth rate limiting
    // - Allows temporary bursts
    // - Memory efficient
    // - Easy to reason about

    false
}

/// Get current usage statistics for monitoring
#[no_mangle]
pub extern "C" fn rate_limiter_get_usage(
    limiter: *const RateLimiter,
    user_id: i32,
    current_time_seconds: i64,
) -> i32 {
    if limiter.is_null() {
        return 0;
    }

    // TODO: Return current request count in window
    // Useful for monitoring and debugging
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_creation() {
        let config = RateLimiterConfig {
            max_requests: 5,
            window_size_seconds: 60,
            algorithm: 1, // Sliding window
        };

        let limiter = rate_limiter_new(config);
        assert!(!limiter.is_null());

        rate_limiter_free(limiter);
    }

    #[test]
    fn test_rate_limiter_basic_allow() {
        let config = RateLimiterConfig {
            max_requests: 3,
            window_size_seconds: 10,
            algorithm: 1,
        };

        let limiter = rate_limiter_new(config);

        // First 3 requests should be allowed (placeholder will reject)
        assert!(!rate_limiter_allow_request(limiter, 1, 0));  // Placeholder behavior
        assert!(!rate_limiter_allow_request(limiter, 1, 1));
        assert!(!rate_limiter_allow_request(limiter, 1, 2));

        // 4th request should be rejected
        assert!(!rate_limiter_allow_request(limiter, 1, 3));

        rate_limiter_free(limiter);
    }

    #[test]
    fn test_rate_limiter_window_reset() {
        let config = RateLimiterConfig {
            max_requests: 2,
            window_size_seconds: 5,
            algorithm: 1,
        };

        let limiter = rate_limiter_new(config);

        // Use requests at time 0, 1
        rate_limiter_allow_request(limiter, 1, 0);
        rate_limiter_allow_request(limiter, 1, 1);

        // Should be rejected at time 2 (within window)
        assert!(!rate_limiter_allow_request(limiter, 1, 2));

        // Should be allowed at time 7 (first request expired)
        assert!(!rate_limiter_allow_request(limiter, 1, 7)); // Placeholder still rejects

        rate_limiter_free(limiter);
    }

    #[test]
    fn test_rate_limiter_multiple_users() {
        let config = RateLimiterConfig {
            max_requests: 2,
            window_size_seconds: 10,
            algorithm: 1,
        };

        let limiter = rate_limiter_new(config);

        // Different users should have independent limits
        rate_limiter_allow_request(limiter, 1, 0);
        rate_limiter_allow_request(limiter, 2, 0);
        rate_limiter_allow_request(limiter, 1, 1);
        rate_limiter_allow_request(limiter, 2, 1);

        // Both users should be at limit
        assert!(!rate_limiter_allow_request(limiter, 1, 2));
        assert!(!rate_limiter_allow_request(limiter, 2, 2));

        rate_limiter_free(limiter);
    }

    #[test]
    fn test_token_bucket_algorithm() {
        let config = RateLimiterConfig {
            max_requests: 5,
            window_size_seconds: 10,
            algorithm: 2, // Token bucket
        };

        let limiter = rate_limiter_new(config);

        // Token bucket allows bursts up to capacity
        assert!(!rate_limiter_allow_request_token_bucket(limiter, 1, 0)); // Placeholder

        rate_limiter_free(limiter);
    }

    #[test]
    fn test_rate_limiter_usage_monitoring() {
        let config = RateLimiterConfig {
            max_requests: 10,
            window_size_seconds: 60,
            algorithm: 1,
        };

        let limiter = rate_limiter_new(config);

        // Initially no usage
        assert_eq!(rate_limiter_get_usage(limiter, 1, 0), 0);

        // After some requests (placeholder won't actually track)
        rate_limiter_allow_request(limiter, 1, 0);
        rate_limiter_allow_request(limiter, 1, 1);
        assert_eq!(rate_limiter_get_usage(limiter, 1, 2), 0); // Placeholder

        rate_limiter_free(limiter);
    }

    #[test]
    fn test_api_scenario() {
        // Simulate tech company API rate limiting scenario
        let config = RateLimiterConfig {
            max_requests: 100,     // 100 requests
            window_size_seconds: 3600, // per hour
            algorithm: 1,
        };

        let limiter = rate_limiter_new(config);
        let user_id = 12345;

        // Simulate steady API usage
        for i in 0..50 {
            rate_limiter_allow_request(limiter, user_id, i * 60); // 1 req/min
        }

        // Should still be under limit
        assert!(!rate_limiter_allow_request(limiter, user_id, 3000)); // Placeholder

        rate_limiter_free(limiter);
    }

    #[test]
    fn test_edge_cases() {
        let config = RateLimiterConfig {
            max_requests: 1,
            window_size_seconds: 1,
            algorithm: 1,
        };

        let limiter = rate_limiter_new(config);

        // Null limiter
        assert!(!rate_limiter_allow_request(std::ptr::null_mut(), 1, 0));

        // Zero/negative time
        assert!(!rate_limiter_allow_request(limiter, 1, -1));

        rate_limiter_free(limiter);
    }
}