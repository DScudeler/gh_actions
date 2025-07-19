use gh_actions::{User, UserManager};
use std::collections::HashMap;

pub fn create_test_user(id: u32) -> User {
    User {
        id,
        name: format!("Test User {}", id),
        email: format!("test{}@example.com", id),
        active: true,
    }
}

pub fn create_inactive_user(id: u32) -> User {
    User {
        id,
        name: format!("Inactive User {}", id),
        email: format!("inactive{}@example.com", id),
        active: false,
    }
}

pub fn create_test_manager_with_users(count: u32) -> UserManager {
    let mut manager = UserManager::new();
    
    for i in 1..=count {
        let user = if i % 2 == 0 {
            create_inactive_user(i)
        } else {
            create_test_user(i)
        };
        manager.add_user(user).unwrap();
    }
    
    manager
}

pub fn assert_user_counts(manager: &UserManager, expected_total: usize, expected_active: usize) {
    assert_eq!(manager.count(), expected_total);
    assert_eq!(manager.get_active_users().len(), expected_active);
    assert_eq!(manager.get_inactive_users().len(), expected_total - expected_active);
}

pub struct TestMetrics {
    pub execution_times: HashMap<String, std::time::Duration>,
    pub memory_usage: HashMap<String, usize>,
}

impl TestMetrics {
    pub fn new() -> Self {
        Self {
            execution_times: HashMap::new(),
            memory_usage: HashMap::new(),
        }
    }
    
    pub fn time_operation<F, R>(&mut self, name: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = operation();
        let duration = start.elapsed();
        self.execution_times.insert(name.to_string(), duration);
        result
    }
    
    pub fn print_metrics(&self) {
        println!("=== Test Metrics ===");
        for (name, duration) in &self.execution_times {
            println!("{}: {:?}", name, duration);
        }
        for (name, memory) in &self.memory_usage {
            println!("{} memory: {} bytes", name, memory);
        }
    }
}

#[macro_export]
macro_rules! assert_duration_less_than {
    ($duration:expr, $max_ms:expr) => {
        assert!(
            $duration.as_millis() < $max_ms,
            "Operation took {}ms, expected less than {}ms",
            $duration.as_millis(),
            $max_ms
        );
    };
}

pub fn generate_test_data(size: usize) -> Vec<User> {
    (1..=size)
        .map(|i| User {
            id: i as u32,
            name: format!("Generated User {}", i),
            email: format!("generated{}@testdomain.com", i),
            active: i % 3 != 0, // ~66% active users
        })
        .collect()
}