use std::collections::HashMap;

/// Calculates the nth Fibonacci number iteratively.
/// 
/// The Fibonacci sequence is defined as:
/// - F(0) = 0
/// - F(1) = 1  
/// - F(n) = F(n-1) + F(n-2) for n > 1
/// 
/// # Arguments
/// 
/// * `n` - The position in the Fibonacci sequence (0-based)
/// 
/// # Returns
/// 
/// Returns `Ok(fibonacci_number)` if successful, or `Err(error_message)` if the input is too large.
/// 
/// # Examples
/// 
/// ```
/// use gh_actions::calculate_fibonacci;
/// 
/// assert_eq!(calculate_fibonacci(0).unwrap(), 0);
/// assert_eq!(calculate_fibonacci(1).unwrap(), 1);
/// assert_eq!(calculate_fibonacci(10).unwrap(), 55);
/// assert_eq!(calculate_fibonacci(20).unwrap(), 6765);
/// 
/// // Numbers too large will return an error
/// assert!(calculate_fibonacci(100).is_err());
/// ```
pub fn calculate_fibonacci(n: u32) -> Result<u64, String> {
    if n > 93 {
        return Err("Number too large for u64".to_string());
    }
    
    match n {
        0 => Ok(0),
        1 => Ok(1),
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                match a.checked_add(b) {
                    Some(sum) => {
                        a = b;
                        b = sum;
                    }
                    None => return Err("Overflow occurred".to_string()),
                }
            }
            Ok(b)
        }
    }
}

pub fn calculate_fibonacci_recursive(n: u32) -> Result<u64, String> {
    if n > 93 {
        return Err("Number too large for u64".to_string());
    }
    
    fn fib_helper(n: u32, memo: &mut HashMap<u32, u64>) -> Result<u64, String> {
        if let Some(&value) = memo.get(&n) {
            return Ok(value);
        }
        
        let result = match n {
            0 => Ok(0),
            1 => Ok(1),
            _ => {
                let a = fib_helper(n - 1, memo)?;
                let b = fib_helper(n - 2, memo)?;
                match a.checked_add(b) {
                    Some(sum) => Ok(sum),
                    None => Err("Overflow occurred".to_string()),
                }
            }
        }?;
        
        memo.insert(n, result);
        Ok(result)
    }
    
    let mut memo = HashMap::new();
    fib_helper(n, &mut memo)
}

/// Validates an email address format.
/// 
/// This function performs basic email validation checking for:
/// - Presence of exactly one '@' symbol
/// - Non-empty local and domain parts
/// - Presence of at least one '.' in the domain
/// - Length constraints and character validation
/// - Protection against common malformed patterns
/// 
/// # Arguments
/// 
/// * `email` - The email address string to validate
/// 
/// # Returns
/// 
/// Returns `true` if the email appears to be valid, `false` otherwise.
/// 
/// # Examples
/// 
/// ```
/// use gh_actions::validate_email;
/// 
/// assert!(validate_email("user@example.com"));
/// assert!(validate_email("test.email@domain.co.uk"));
/// assert!(validate_email("user+tag@example.org"));
/// 
/// assert!(!validate_email("invalid-email"));
/// assert!(!validate_email("@domain.com"));
/// assert!(!validate_email("user@"));
/// assert!(!validate_email("user@@domain.com"));
/// ```
pub fn validate_email(email: &str) -> bool {
    if email.len() < 5 || email.len() > 320 {
        return false;
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let (local, domain) = (parts[0], parts[1]);
    
    if local.is_empty() || local.len() > 64 || domain.is_empty() || domain.len() > 253 {
        return false;
    }
    
    if !domain.contains('.') {
        return false;
    }
    
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return false;
    }
    
    if domain.starts_with('.') || domain.ends_with('.') || domain.contains("..") {
        return false;
    }
    
    local.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-' || c == '+') &&
    domain.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn factorial(n: u64) -> Result<u64, String> {
    if n > 20 {
        return Err("Number too large for u64 factorial".to_string());
    }
    
    let mut result = 1u64;
    for i in 1..=n {
        match result.checked_mul(i) {
            Some(val) => result = val,
            None => return Err("Overflow occurred".to_string()),
        }
    }
    Ok(result)
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    
    let gcd_val = gcd(a, b);
    a.checked_div(gcd_val)?.checked_mul(b)
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    cleaned == reverse_string(&cleaned)
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(calculate_fibonacci(0).unwrap(), 0);
        assert_eq!(calculate_fibonacci(1).unwrap(), 1);
        assert_eq!(calculate_fibonacci(2).unwrap(), 1);
    }

    #[test]
    fn test_fibonacci_normal_cases() {
        assert_eq!(calculate_fibonacci(5).unwrap(), 5);
        assert_eq!(calculate_fibonacci(10).unwrap(), 55);
        assert_eq!(calculate_fibonacci(15).unwrap(), 610);
    }

    #[test]
    fn test_fibonacci_large_valid() {
        assert_eq!(calculate_fibonacci(50).unwrap(), 12586269025);
    }

    #[test]
    fn test_fibonacci_too_large() {
        let result = calculate_fibonacci(94);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too large"));
    }

    #[test]
    fn test_fibonacci_recursive() {
        assert_eq!(calculate_fibonacci_recursive(0).unwrap(), 0);
        assert_eq!(calculate_fibonacci_recursive(1).unwrap(), 1);
        assert_eq!(calculate_fibonacci_recursive(10).unwrap(), 55);
        assert_eq!(calculate_fibonacci_recursive(20).unwrap(), 6765);
    }

    #[test]
    fn test_fibonacci_both_methods_agree() {
        for i in 0..=20 {
            assert_eq!(
                calculate_fibonacci(i).unwrap(),
                calculate_fibonacci_recursive(i).unwrap()
            );
        }
    }

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("test@example.com"));
        assert!(validate_email("user.name@domain.co.uk"));
        assert!(validate_email("test+tag@example.org"));
        assert!(validate_email("user123@test-domain.com"));
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(!validate_email(""));
        assert!(!validate_email("test"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@"));
        assert!(!validate_email("user@@domain.com"));
        assert!(!validate_email("user@domain"));
        assert!(!validate_email(".user@domain.com"));
        assert!(!validate_email("user.@domain.com"));
        assert!(!validate_email("user..name@domain.com"));
        assert!(!validate_email("user@.domain.com"));
        assert!(!validate_email("user@domain.com."));
    }

    #[test]
    fn test_validate_email_edge_cases() {
        assert!(!validate_email("a@b.c")); // Too short overall
        assert!(!validate_email(&"a".repeat(65) + "@example.com")); // Local part too long
    }

    #[test]
    fn test_is_prime() {
        // Test small primes
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        
        // Test larger primes
        assert!(is_prime(17));
        assert!(is_prime(97));
        assert!(is_prime(101));
        
        // Test composites
        assert!(!is_prime(100));
        assert!(!is_prime(121));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).unwrap(), 1);
        assert_eq!(factorial(1).unwrap(), 1);
        assert_eq!(factorial(5).unwrap(), 120);
        assert_eq!(factorial(10).unwrap(), 3628800);
    }

    #[test]
    fn test_factorial_too_large() {
        let result = factorial(21);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too large"));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), Some(12));
        assert_eq!(lcm(17, 13), Some(221));
        assert_eq!(lcm(0, 5), Some(0));
        assert_eq!(lcm(5, 0), Some(0));
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
        assert_eq!(reverse_string("12345"), "54321");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome("race a car")); // This should be false due to spaces
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("Madam"));
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("   "), 0);
        assert_eq!(count_words("single"), 1);
        assert_eq!(count_words("  hello   world  test  "), 3);
    }

    #[test]
    fn test_performance_fibonacci() {
        let start = std::time::Instant::now();
        let _ = calculate_fibonacci(40);
        let duration = start.elapsed();
        
        // Should complete in reasonable time (less than 1 second)
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_performance_fibonacci_recursive() {
        let start = std::time::Instant::now();
        let _ = calculate_fibonacci_recursive(30);
        let duration = start.elapsed();
        
        // Recursive with memoization should be fast
        assert!(duration.as_millis() < 100);
    }
}