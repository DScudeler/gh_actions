mod common;

use gh_actions::{User, UserManager, calculate_fibonacci, validate_email, is_prime, factorial, gcd, lcm};
use common::*;

// Property-based testing without external crates
// We'll use a simple approach with loops and random-like data

#[test]
fn property_fibonacci_sequence() {
    // Test that fibonacci sequence follows the mathematical property: F(n) = F(n-1) + F(n-2)
    for i in 2..=30 {
        let fib_i = calculate_fibonacci(i).unwrap();
        let fib_i_minus_1 = calculate_fibonacci(i - 1).unwrap();
        let fib_i_minus_2 = calculate_fibonacci(i - 2).unwrap();
        
        assert_eq!(fib_i, fib_i_minus_1 + fib_i_minus_2,
                  "Fibonacci property failed for n={}", i);
    }
}

#[test]
fn property_fibonacci_monotonic() {
    // Test that fibonacci sequence is monotonically increasing (except for F(0))
    let mut prev = calculate_fibonacci(1).unwrap();
    
    for i in 2..=40 {
        let current = calculate_fibonacci(i).unwrap();
        assert!(current >= prev, "Fibonacci sequence not monotonic at n={}", i);
        prev = current;
    }
}

#[test]
fn property_gcd_properties() {
    let test_pairs = vec![
        (12, 8), (48, 18), (100, 25), (17, 13), (1071, 462),
        (25, 15), (36, 24), (54, 24), (81, 27), (120, 80)
    ];
    
    for (a, b) in test_pairs {
        let g = gcd(a, b);
        
        // Property 1: gcd(a, b) divides both a and b
        assert_eq!(a % g, 0, "gcd({}, {}) = {} doesn't divide {}", a, b, g, a);
        assert_eq!(b % g, 0, "gcd({}, {}) = {} doesn't divide {}", a, b, g, b);
        
        // Property 2: gcd(a, b) = gcd(b, a) (commutative)
        assert_eq!(gcd(a, b), gcd(b, a), "gcd not commutative for ({}, {})", a, b);
        
        // Property 3: gcd(a, 0) = a
        assert_eq!(gcd(a, 0), a);
        assert_eq!(gcd(0, b), b);
        
        // Property 4: gcd(a, b) * lcm(a, b) = a * b (when no overflow)
        if let Some(l) = lcm(a, b) {
            // Check only if multiplication won't overflow
            if g <= u64::MAX / l {
                assert_eq!(g * l, a * b, "gcd-lcm identity failed for ({}, {})", a, b);
            }
        }
    }
}

#[test]
fn property_prime_checking() {
    let known_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let known_composites = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28];
    
    // Test known primes
    for &p in &known_primes {
        assert!(is_prime(p), "{} should be prime", p);
    }
    
    // Test known composites
    for &c in &known_composites {
        assert!(!is_prime(c), "{} should not be prime", c);
    }
    
    // Property: all even numbers > 2 are not prime
    for i in (4..=100).step_by(2) {
        assert!(!is_prime(i), "Even number {} should not be prime", i);
    }
    
    // Property: 1 and 0 are not prime
    assert!(!is_prime(0));
    assert!(!is_prime(1));
}

#[test]
fn property_factorial_properties() {
    // Test factorial properties
    for i in 0..=10 {
        let fact_i = factorial(i).unwrap();
        
        if i > 0 {
            let fact_i_minus_1 = factorial(i - 1).unwrap();
            // Property: n! = n * (n-1)!
            assert_eq!(fact_i, i * fact_i_minus_1, 
                      "Factorial property failed for n={}", i);
        }
        
        // Property: n! >= n for all n >= 1
        if i >= 1 {
            assert!(fact_i >= i, "n! < n for n={}", i);
        }
    }
    
    // Base case
    assert_eq!(factorial(0).unwrap(), 1);
    assert_eq!(factorial(1).unwrap(), 1);
}

#[test]
fn property_email_validation_structure() {
    let test_emails = vec![
        ("valid@example.com", true),
        ("test.email@domain.co.uk", true),
        ("user+tag@example.org", true),
        ("", false),
        ("no-at-sign", false),
        ("@no-local-part.com", false),
        ("no-domain-part@", false),
        ("double@@at.com", false),
        ("no.domain@nodot", false),
        (".starts-with-dot@example.com", false),
        ("ends-with-dot.@example.com", false),
    ];
    
    for (email, expected) in test_emails {
        assert_eq!(validate_email(email), expected, 
                  "Email validation failed for: {}", email);
    }
    
    // Property: valid emails must contain exactly one '@'
    let emails_with_at = vec![
        "test@example.com",
        "user@domain.org",
        "valid@test.co.uk",
    ];
    
    for email in emails_with_at {
        if validate_email(email) {
            let at_count = email.chars().filter(|&c| c == '@').count();
            assert_eq!(at_count, 1, "Valid email {} doesn't have exactly one @", email);
        }
    }
}

#[test]
fn property_user_manager_invariants() {
    let mut manager = UserManager::new();
    
    // Property: count should always equal the number of users
    assert_eq!(manager.count(), manager.get_users().len());
    
    // Add users and check invariants
    let mut expected_count = 0;
    for i in 1..=10 {
        let user = create_test_user(i);
        manager.add_user(user).unwrap();
        expected_count += 1;
        
        // Invariant: count increases by 1
        assert_eq!(manager.count(), expected_count);
        assert_eq!(manager.count(), manager.get_users().len());
        
        // Invariant: active + inactive users = total users
        let active_count = manager.get_active_users().len();
        let inactive_count = manager.get_inactive_users().len();
        assert_eq!(active_count + inactive_count, manager.count());
        
        // Invariant: we can retrieve the user we just added
        assert!(manager.get_user(i).is_some());
    }
    
    // Test deletion invariants
    for i in 1..=5 {
        manager.delete_user(i).unwrap();
        expected_count -= 1;
        
        // Invariant: count decreases by 1
        assert_eq!(manager.count(), expected_count);
        
        // Invariant: deleted user cannot be retrieved
        assert!(manager.get_user(i).is_none());
    }
}

#[test]
fn property_user_manager_state_consistency() {
    let mut manager = create_test_manager_with_users(20);
    
    // Get initial state
    let initial_active = manager.get_active_users().len();
    let initial_inactive = manager.get_inactive_users().len();
    let initial_total = manager.count();
    
    assert_eq!(initial_active + initial_inactive, initial_total);
    
    // Deactivate some users and check consistency
    for i in (1..=10).step_by(2) {
        if manager.get_user(i).is_some() {
            manager.deactivate_user(i).unwrap();
            
            // State should remain consistent
            let active = manager.get_active_users().len();
            let inactive = manager.get_inactive_users().len();
            assert_eq!(active + inactive, manager.count());
        }
    }
    
    // Activate some users and check consistency  
    for i in (2..=20).step_by(2) {
        if manager.get_user(i).is_some() {
            manager.activate_user(i).unwrap();
            
            // State should remain consistent
            let active = manager.get_active_users().len();
            let inactive = manager.get_inactive_users().len();
            assert_eq!(active + inactive, manager.count());
        }
    }
}

#[test]
fn property_mathematical_function_ranges() {
    // Test that our functions handle edge cases properly
    
    // Fibonacci edge cases
    assert!(calculate_fibonacci(0).is_ok());
    assert!(calculate_fibonacci(1).is_ok());
    assert!(calculate_fibonacci(93).is_ok());
    assert!(calculate_fibonacci(94).is_err());
    
    // Factorial edge cases
    assert!(factorial(0).is_ok());
    assert!(factorial(20).is_ok());
    assert!(factorial(21).is_err());
    
    // Prime checking edge cases
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    
    // GCD edge cases
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn property_stress_test_user_operations() {
    let mut metrics = TestMetrics::new();
    
    // Test with various sizes to ensure consistent performance characteristics
    for size in [10, 50, 100, 500].iter() {
        let mut manager = UserManager::new();
        
        // Measure bulk insertion time
        let insert_result = metrics.time_operation(
            &format!("bulk_insert_{}", size),
            || {
                for i in 1..=*size {
                    let user = create_test_user(i as u32);
                    manager.add_user(user).unwrap();
                }
            }
        );
        
        // Verify all users were added correctly
        assert_eq!(manager.count(), *size);
        
        // Measure filtering performance
        metrics.time_operation(
            &format!("filter_active_{}", size),
            || {
                let active_users = manager.get_active_users();
                assert!(!active_users.is_empty());
            }
        );
        
        // Measure lookup performance
        metrics.time_operation(
            &format!("lookup_{}", size),
            || {
                for i in 1..=*size {
                    assert!(manager.get_user(i as u32).is_some());
                }
            }
        );
    }
    
    // Print performance metrics for analysis
    metrics.print_metrics();
}