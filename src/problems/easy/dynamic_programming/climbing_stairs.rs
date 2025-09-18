/// Problem: Climbing Stairs
///
/// You are climbing a staircase. It takes n steps to reach the top.
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
///
/// Example 1:
/// Input: n = 2
/// Output: 2
/// Explanation: There are two ways to climb to the top.
/// 1. 1 step + 1 step
/// 2. 2 steps
///
/// Example 2:
/// Input: n = 3
/// Output: 3
/// Explanation: There are three ways to climb to the top.
/// 1. 1 step + 1 step + 1 step
/// 2. 1 step + 2 steps
/// 3. 2 steps + 1 step
///
/// Constraints:
/// - 1 <= n <= 45
///
/// Algorithm Pattern: Classic 1D Dynamic Programming
/// This class of problems can be solved with 1D DP when you need to:
/// - Build solution from smaller subproblems
/// - Each state depends on a fixed number of previous states
/// - Overlapping subproblems exist (avoid recalculation)
/// - Optimal substructure property holds
///
/// Translation Strategy:
/// English: "How many ways to reach step n?"
/// Algorithm: ways(n) = ways(n-1) + ways(n-2) because you can reach step n from step n-1 or n-2
///
/// Key DP Insight:
/// To reach step n, you can:
/// 1. Take 1 step from step n-1
/// 2. Take 2 steps from step n-2
/// Total ways = ways to reach n-1 + ways to reach n-2
///
/// This is the Fibonacci sequence in disguise!
/// Base cases: f(1) = 1, f(2) = 2
///
/// Common 1D DP Patterns:
/// 1. Fibonacci-like: Current state depends on 1-2 previous states
/// 2. Sequence problems: Longest increasing subsequence, etc.
/// 3. Decision problems: Take it or leave it (0/1 choices)
/// 4. Path counting: Number of ways to reach a position
///
/// Hints:
/// 1. Identify the recurrence relation: f(n) = f(n-1) + f(n-2)
/// 2. Start with base cases: f(0) = 1, f(1) = 1, f(2) = 2
/// 3. Build up from smallest to largest (bottom-up DP)
/// 4. Optimize space: Only need last 2 values, not entire array
/// 5. Be careful with edge cases and integer overflow
///
/// Implementation approaches:
/// 1. Recursive (exponential time, for understanding)
/// 2. Recursive + memoization (O(n) time, O(n) space)
/// 3. Bottom-up DP array (O(n) time, O(n) space)
/// 4. Bottom-up optimized (O(n) time, O(1) space) - optimal
///
/// Time Complexity: O(n) - calculate each value once
/// Space Complexity: O(1) - only store last 2 values

#[no_mangle]
pub extern "C" fn climb_stairs(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }

    // TODO: Implement the bottom-up DP solution
    //
    // Your implementation should:
    // 1. Handle base cases: n=1 returns 1, n=2 returns 2
    // 2. Use iterative approach with 2 variables to save space
    // 3. For i from 3 to n:
    //    - Calculate ways[i] = ways[i-1] + ways[i-2]
    //    - Update the two variables to prepare for next iteration
    // 4. Return the final result
    //
    // Space-optimized approach (O(1) space):
    // let mut prev2 = 1; // f(n-2)
    // let mut prev1 = 2; // f(n-1)
    // for i in 3..=n {
    //     let current = prev1 + prev2;
    //     prev2 = prev1;
    //     prev1 = current;
    // }
    //
    // Alternative approaches:
    // - Recursive: Simple but O(2^n) time due to repeated calculations
    // - Memoization: O(n) time and space, top-down approach
    // - DP array: O(n) time and space, easier to understand
    // - Mathematical: Fibonacci closed form (complex, potential precision issues)

    // Placeholder - replace with your implementation
    0
}

/// Bonus: Fibonacci number (the foundation pattern)
#[no_mangle]
pub extern "C" fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    // TODO: Implement Fibonacci using same DP pattern
    // This is the exact same algorithm as climb_stairs with different base cases
    // Base cases: f(0) = 0, f(1) = 1
    // Recurrence: f(n) = f(n-1) + f(n-2)

    // Placeholder
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs_basic() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }

    #[test]
    fn test_climb_stairs_larger() {
        assert_eq!(climb_stairs(6), 13);
        assert_eq!(climb_stairs(7), 21);
        assert_eq!(climb_stairs(8), 34);
        assert_eq!(climb_stairs(10), 89);
    }

    #[test]
    fn test_climb_stairs_edge_cases() {
        assert_eq!(climb_stairs(0), 0);
        assert_eq!(climb_stairs(-1), 0);

        // Test reasonably large value
        assert_eq!(climb_stairs(15), 987);
    }

    #[test]
    fn test_climb_stairs_pattern() {
        // Verify it follows Fibonacci pattern: f(n) = f(n-1) + f(n-2)
        // But with different starting values: stairs(1)=1, stairs(2)=2
        let a = climb_stairs(8);  // 34
        let b = climb_stairs(9);  // 55
        let c = climb_stairs(10); // 89
        assert_eq!(c, a + b); // 89 = 34 + 55
    }

    #[test]
    fn test_fibonacci_basic() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }

    #[test]
    fn test_fibonacci_pattern() {
        // Verify true Fibonacci sequence
        let a = fibonacci(10); // 55
        let b = fibonacci(11); // 89
        let c = fibonacci(12); // 144
        assert_eq!(c, a + b); // 144 = 55 + 89
    }

    #[test]
    fn test_fibonacci_edge_cases() {
        assert_eq!(fibonacci(-1), -1); // Handle negative input
        assert_eq!(fibonacci(15), 610);
    }

    #[test]
    fn test_stairs_vs_fibonacci_relationship() {
        // climb_stairs(n) = fibonacci(n+1) for n >= 1
        for n in 1..=10 {
            assert_eq!(climb_stairs(n), fibonacci(n + 1));
        }
    }
}