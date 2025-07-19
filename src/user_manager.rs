use serde::{Deserialize, Serialize};
use std::fs;

/// Represents a user in the system.
/// 
/// # Examples
/// 
/// ```
/// use gh_actions::User;
/// 
/// let user = User {
///     id: 1,
///     name: "John Doe".to_string(),
///     email: "john@example.com".to_string(),
///     active: true,
/// };
/// 
/// assert_eq!(user.id, 1);
/// assert_eq!(user.name, "John Doe");
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

/// Manages a collection of users with CRUD operations.
/// 
/// The UserManager provides functionality to add, retrieve, update, and delete users,
/// as well as filter users by their active status and persist data to/from files.
/// 
/// # Examples
/// 
/// ```
/// use gh_actions::{UserManager, User};
/// 
/// let mut manager = UserManager::new();
/// 
/// let user = User {
///     id: 1,
///     name: "Alice".to_string(),
///     email: "alice@example.com".to_string(),
///     active: true,
/// };
/// 
/// manager.add_user(user.clone()).unwrap();
/// assert_eq!(manager.get_user(1), Some(&user));
/// assert_eq!(manager.count(), 1);
/// ```
#[derive(Debug)]
pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.users.iter().any(|u| u.id == user.id) {
            return Err(format!("User with ID {} already exists", user.id));
        }
        if user.name.trim().is_empty() {
            return Err("User name cannot be empty".to_string());
        }
        if !crate::utils::validate_email(&user.email) {
            return Err("Invalid email format".to_string());
        }
        self.users.push(user);
        Ok(())
    }

    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn update_user(&mut self, id: u32, updated_user: User) -> Result<(), String> {
        if updated_user.name.trim().is_empty() {
            return Err("User name cannot be empty".to_string());
        }
        if !crate::utils::validate_email(&updated_user.email) {
            return Err("Invalid email format".to_string());
        }

        match self.users.iter_mut().find(|u| u.id == id) {
            Some(user) => {
                *user = updated_user;
                Ok(())
            }
            None => Err(format!("User with ID {} not found", id)),
        }
    }

    pub fn delete_user(&mut self, id: u32) -> Result<(), String> {
        let initial_len = self.users.len();
        self.users.retain(|u| u.id != id);
        
        if self.users.len() == initial_len {
            Err(format!("User with ID {} not found", id))
        } else {
            Ok(())
        }
    }

    pub fn get_active_users(&self) -> Vec<&User> {
        self.users.iter().filter(|u| u.active).collect()
    }

    pub fn get_inactive_users(&self) -> Vec<&User> {
        self.users.iter().filter(|u| !u.active).collect()
    }

    pub fn activate_user(&mut self, id: u32) -> Result<(), String> {
        match self.users.iter_mut().find(|u| u.id == id) {
            Some(user) => {
                user.active = true;
                Ok(())
            }
            None => Err(format!("User with ID {} not found", id)),
        }
    }

    pub fn deactivate_user(&mut self, id: u32) -> Result<(), String> {
        match self.users.iter_mut().find(|u| u.id == id) {
            Some(user) => {
                user.active = false;
                Ok(())
            }
            None => Err(format!("User with ID {} not found", id)),
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.users)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        self.users = serde_json::from_str(&content)?;
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.users.len()
    }

    pub fn clear(&mut self) {
        self.users.clear();
    }
}

impl Default for UserManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(id: u32) -> User {
        User {
            id,
            name: format!("Test User {}", id),
            email: format!("test{}@example.com", id),
            active: true,
        }
    }

    #[test]
    fn test_new_user_manager() {
        let manager = UserManager::new();
        assert_eq!(manager.count(), 0);
        assert!(manager.get_users().is_empty());
    }

    #[test]
    fn test_default_user_manager() {
        let manager = UserManager::default();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_add_user_success() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        
        assert!(manager.add_user(user.clone()).is_ok());
        assert_eq!(manager.count(), 1);
        assert_eq!(manager.get_user(1), Some(&user));
    }

    #[test]
    fn test_add_duplicate_user() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        
        manager.add_user(user.clone()).unwrap();
        let result = manager.add_user(user);
        
        assert!(result.is_err());
        assert_eq!(manager.count(), 1);
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_add_user_empty_name() {
        let mut manager = UserManager::new();
        let user = User {
            id: 1,
            name: "".to_string(),
            email: "test@example.com".to_string(),
            active: true,
        };
        
        let result = manager.add_user(user);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("name cannot be empty"));
    }

    #[test]
    fn test_add_user_invalid_email() {
        let mut manager = UserManager::new();
        let user = User {
            id: 1,
            name: "Test User".to_string(),
            email: "invalid-email".to_string(),
            active: true,
        };
        
        let result = manager.add_user(user);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid email"));
    }

    #[test]
    fn test_get_user() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        manager.add_user(user.clone()).unwrap();
        
        assert_eq!(manager.get_user(1), Some(&user));
        assert_eq!(manager.get_user(999), None);
    }

    #[test]
    fn test_update_user() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        manager.add_user(user).unwrap();
        
        let updated_user = User {
            id: 1,
            name: "Updated User".to_string(),
            email: "updated@example.com".to_string(),
            active: false,
        };
        
        assert!(manager.update_user(1, updated_user.clone()).is_ok());
        assert_eq!(manager.get_user(1), Some(&updated_user));
    }

    #[test]
    fn test_update_nonexistent_user() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        
        let result = manager.update_user(999, user);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_delete_user() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        manager.add_user(user).unwrap();
        
        assert!(manager.delete_user(1).is_ok());
        assert_eq!(manager.count(), 0);
        assert_eq!(manager.get_user(1), None);
    }

    #[test]
    fn test_delete_nonexistent_user() {
        let mut manager = UserManager::new();
        let result = manager.delete_user(999);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_get_active_users() {
        let mut manager = UserManager::new();
        
        let active_user = User {
            id: 1,
            name: "Active User".to_string(),
            email: "active@example.com".to_string(),
            active: true,
        };
        
        let inactive_user = User {
            id: 2,
            name: "Inactive User".to_string(),
            email: "inactive@example.com".to_string(),
            active: false,
        };
        
        manager.add_user(active_user.clone()).unwrap();
        manager.add_user(inactive_user).unwrap();
        
        let active_users = manager.get_active_users();
        assert_eq!(active_users.len(), 1);
        assert_eq!(active_users[0], &active_user);
    }

    #[test]
    fn test_get_inactive_users() {
        let mut manager = UserManager::new();
        
        let active_user = User {
            id: 1,
            name: "Active User".to_string(),
            email: "active@example.com".to_string(),
            active: true,
        };
        
        let inactive_user = User {
            id: 2,
            name: "Inactive User".to_string(),
            email: "inactive@example.com".to_string(),
            active: false,
        };
        
        manager.add_user(active_user).unwrap();
        manager.add_user(inactive_user.clone()).unwrap();
        
        let inactive_users = manager.get_inactive_users();
        assert_eq!(inactive_users.len(), 1);
        assert_eq!(inactive_users[0], &inactive_user);
    }

    #[test]
    fn test_activate_user() {
        let mut manager = UserManager::new();
        let mut user = create_test_user(1);
        user.active = false;
        manager.add_user(user).unwrap();
        
        assert!(manager.activate_user(1).is_ok());
        assert!(manager.get_user(1).unwrap().active);
    }

    #[test]
    fn test_deactivate_user() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        manager.add_user(user).unwrap();
        
        assert!(manager.deactivate_user(1).is_ok());
        assert!(!manager.get_user(1).unwrap().active);
    }

    #[test]
    fn test_clear() {
        let mut manager = UserManager::new();
        manager.add_user(create_test_user(1)).unwrap();
        manager.add_user(create_test_user(2)).unwrap();
        
        assert_eq!(manager.count(), 2);
        manager.clear();
        assert_eq!(manager.count(), 0);
    }
}