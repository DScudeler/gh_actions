pub mod user_manager;
pub mod utils;

pub use user_manager::{User, UserManager};
pub use utils::{
    calculate_fibonacci, 
    calculate_fibonacci_recursive, 
    validate_email, 
    is_prime, 
    factorial, 
    gcd, 
    lcm, 
    reverse_string, 
    is_palindrome, 
    count_words
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        let mut manager = UserManager::new();
        let user = User {
            id: 1,
            name: "Integration Test".to_string(),
            email: "integration@test.com".to_string(),
            active: true,
        };
        
        manager.add_user(user).unwrap();
        assert_eq!(manager.get_users().len(), 1);
        assert!(validate_email("integration@test.com"));
        assert_eq!(calculate_fibonacci(5).unwrap(), 5);
    }
}