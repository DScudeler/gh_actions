use gh_actions::{User, UserManager, calculate_fibonacci, validate_email, is_prime, factorial};
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_user_manager_full_workflow() {
    let mut manager = UserManager::new();
    
    // Add users
    let user1 = User {
        id: 1,
        name: "Alice Johnson".to_string(),
        email: "alice@company.com".to_string(),
        active: true,
    };
    
    let user2 = User {
        id: 2,
        name: "Bob Smith".to_string(),
        email: "bob@company.com".to_string(),
        active: false,
    };
    
    assert!(manager.add_user(user1.clone()).is_ok());
    assert!(manager.add_user(user2.clone()).is_ok());
    
    // Test retrieval
    assert_eq!(manager.get_user(1), Some(&user1));
    assert_eq!(manager.get_user(2), Some(&user2));
    assert_eq!(manager.count(), 2);
    
    // Test filtering
    let active_users = manager.get_active_users();
    assert_eq!(active_users.len(), 1);
    assert_eq!(active_users[0], &user1);
    
    let inactive_users = manager.get_inactive_users();
    assert_eq!(inactive_users.len(), 1);
    assert_eq!(inactive_users[0], &user2);
    
    // Test activation/deactivation
    assert!(manager.deactivate_user(1).is_ok());
    assert!(!manager.get_user(1).unwrap().active);
    
    assert!(manager.activate_user(2).is_ok());
    assert!(manager.get_user(2).unwrap().active);
    
    // Test update
    let updated_user = User {
        id: 1,
        name: "Alice Updated".to_string(),
        email: "alice.updated@company.com".to_string(),
        active: true,
    };
    assert!(manager.update_user(1, updated_user.clone()).is_ok());
    assert_eq!(manager.get_user(1), Some(&updated_user));
    
    // Test deletion
    assert!(manager.delete_user(2).is_ok());
    assert_eq!(manager.count(), 1);
    assert_eq!(manager.get_user(2), None);
}

#[test]
fn test_file_persistence() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = UserManager::new();
    
    let user = User {
        id: 1,
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        active: true,
    };
    
    manager.add_user(user.clone())?;
    
    // Create temporary file
    let mut temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_str().unwrap();
    
    // Save and load
    manager.save_to_file(temp_path)?;
    
    let mut new_manager = UserManager::new();
    new_manager.load_from_file(temp_path)?;
    
    assert_eq!(new_manager.count(), 1);
    assert_eq!(new_manager.get_user(1), Some(&user));
    
    Ok(())
}

#[test]
fn test_mathematical_functions_integration() {
    // Test fibonacci with prime checking
    let fib_10 = calculate_fibonacci(10).unwrap();
    assert_eq!(fib_10, 55);
    assert!(!is_prime(fib_10));
    
    let fib_7 = calculate_fibonacci(7).unwrap();
    assert_eq!(fib_7, 13);
    assert!(is_prime(fib_7));
    
    // Test factorial
    let fact_5 = factorial(5).unwrap();
    assert_eq!(fact_5, 120);
    assert!(!is_prime(fact_5));
}

#[test]
fn test_email_validation_comprehensive() {
    let valid_emails = vec![
        "test@example.com",
        "user.name@domain.co.uk",
        "test+tag@example.org",
        "user123@test-domain.com",
        "a@b.co",
    ];
    
    let invalid_emails = vec![
        "",
        "test",
        "@domain.com",
        "user@",
        "user@@domain.com",
        "user@domain",
        ".user@domain.com",
        "user.@domain.com",
        "user..name@domain.com",
        "user@.domain.com",
        "user@domain.com.",
    ];
    
    for email in valid_emails {
        assert!(validate_email(email), "Expected {} to be valid", email);
    }
    
    for email in invalid_emails {
        assert!(!validate_email(email), "Expected {} to be invalid", email);
    }
}

#[test]
fn test_error_handling_integration() {
    let mut manager = UserManager::new();
    
    // Test adding user with invalid email
    let invalid_user = User {
        id: 1,
        name: "Test User".to_string(),
        email: "invalid-email".to_string(),
        active: true,
    };
    
    let result = manager.add_user(invalid_user);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid email"));
    
    // Test operations on empty manager
    assert!(manager.update_user(1, User {
        id: 1,
        name: "Test".to_string(),
        email: "test@example.com".to_string(),
        active: true,
    }).is_err());
    
    assert!(manager.delete_user(1).is_err());
    assert!(manager.activate_user(1).is_err());
    assert!(manager.deactivate_user(1).is_err());
}

#[test]
fn test_large_dataset_performance() {
    let mut manager = UserManager::new();
    
    // Add 1000 users
    let start = std::time::Instant::now();
    for i in 1..=1000 {
        let user = User {
            id: i,
            name: format!("User {}", i),
            email: format!("user{}@example.com", i),
            active: i % 2 == 0,
        };
        manager.add_user(user).unwrap();
    }
    let add_duration = start.elapsed();
    
    assert_eq!(manager.count(), 1000);
    println!("Added 1000 users in {:?}", add_duration);
    
    // Test retrieval performance
    let start = std::time::Instant::now();
    let active_users = manager.get_active_users();
    let filter_duration = start.elapsed();
    
    assert_eq!(active_users.len(), 500);
    println!("Filtered 1000 users in {:?}", filter_duration);
    
    // Performance should be reasonable
    assert!(add_duration.as_millis() < 1000);
    assert!(filter_duration.as_millis() < 100);
}

#[cfg(test)]
mod async_tests {
    use super::*;
    use tokio_test;
    
    #[tokio_test::block_on]
    async fn test_concurrent_operations() {
        let mut manager = UserManager::new();
        
        let user = User {
            id: 1,
            name: "Async User".to_string(),
            email: "async@example.com".to_string(),
            active: true,
        };
        
        // Simulate async operation
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        manager.add_user(user.clone()).unwrap();
        
        assert_eq!(manager.get_user(1), Some(&user));
    }
}