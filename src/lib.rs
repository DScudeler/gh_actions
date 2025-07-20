pub mod task;
pub mod wasm;
pub mod user_manager;
pub mod utils;

pub use task::{Task, TaskManager};
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

// Optional: Use wee_alloc as the global allocator for smaller WASM binary size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    
    #[test]
    fn test_task_manager_integration() {
        let mut task_manager = TaskManager::new();
        let id = task_manager.add_task("Test Task".to_string(), "Test Description".to_string());
        
        assert_eq!(task_manager.get_total_count(), 1);
        assert_eq!(task_manager.get_completed_count(), 0);
        
        task_manager.toggle_task(id);
        assert_eq!(task_manager.get_completed_count(), 1);
    }
}