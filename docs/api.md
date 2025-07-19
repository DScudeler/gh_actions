# API Documentation

## Programmatic Interface

This document describes the programmatic API for the `gh_actions` crate, which can be used as a library in other Rust projects.

## Adding as Dependency

```toml
[dependencies]
gh_actions = { git = "https://github.com/DScudeler/gh_actions.git" }
```

## Core Types

### User

```rust
use gh_actions::User;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}
```

#### Example Usage

```rust
use gh_actions::User;

let user = User {
    id: 1,
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    active: true,
};

println!("User: {} ({})", user.name, user.email);
```

### UserManager

The `UserManager` provides CRUD operations for managing users.

```rust
use gh_actions::{UserManager, User};

let mut manager = UserManager::new();
```

## UserManager API

### Creation and Basic Operations

#### `new() -> Self`
Creates a new empty UserManager.

```rust
let mut manager = UserManager::new();
```

#### `default() -> Self`
Same as `new()`, implements the Default trait.

#### `count() -> usize`
Returns the number of users in the manager.

```rust
let count = manager.count();
println!("Total users: {}", count);
```

#### `clear(&mut self)`
Removes all users from the manager.

```rust
manager.clear();
assert_eq!(manager.count(), 0);
```

### User Operations

#### `add_user(&mut self, user: User) -> Result<(), String>`
Adds a new user. Returns error if ID already exists or validation fails.

```rust
let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
    active: true,
};

match manager.add_user(user) {
    Ok(_) => println!("User added successfully"),
    Err(e) => eprintln!("Failed to add user: {}", e),
}
```

#### `get_user(&self, id: u32) -> Option<&User>`
Retrieves a user by ID.

```rust
match manager.get_user(1) {
    Some(user) => println!("Found user: {}", user.name),
    None => println!("User not found"),
}
```

#### `get_users(&self) -> &Vec<User>`
Returns all users.

```rust
for user in manager.get_users() {
    println!("{}: {}", user.id, user.name);
}
```

#### `update_user(&mut self, id: u32, updated_user: User) -> Result<(), String>`
Updates an existing user.

```rust
let updated_user = User {
    id: 1,
    name: "Alice Smith".to_string(),
    email: "alice.smith@example.com".to_string(),
    active: true,
};

manager.update_user(1, updated_user)?;
```

#### `delete_user(&mut self, id: u32) -> Result<(), String>`
Removes a user by ID.

```rust
match manager.delete_user(1) {
    Ok(_) => println!("User deleted"),
    Err(e) => eprintln!("Failed to delete user: {}", e),
}
```

### Status Management

#### `get_active_users(&self) -> Vec<&User>`
Returns only active users.

```rust
let active_users = manager.get_active_users();
println!("Active users: {}", active_users.len());
```

#### `get_inactive_users(&self) -> Vec<&User>`
Returns only inactive users.

```rust
let inactive_users = manager.get_inactive_users();
```

#### `activate_user(&mut self, id: u32) -> Result<(), String>`
Activates a user by ID.

```rust
manager.activate_user(1)?;
```

#### `deactivate_user(&mut self, id: u32) -> Result<(), String>`
Deactivates a user by ID.

```rust
manager.deactivate_user(1)?;
```

### File Operations

#### `save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>`
Saves users to a JSON file.

```rust
manager.save_to_file("users.json")?;
```

#### `load_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>`
Loads users from a JSON file.

```rust
manager.load_from_file("users.json")?;
```

## Utility Functions

### Mathematical Functions

#### `calculate_fibonacci(n: u32) -> Result<u64, String>`
Calculates the nth Fibonacci number iteratively.

```rust
use gh_actions::calculate_fibonacci;

match calculate_fibonacci(10) {
    Ok(result) => println!("Fibonacci(10) = {}", result), // 55
    Err(e) => eprintln!("Error: {}", e),
}
```

#### `calculate_fibonacci_recursive(n: u32) -> Result<u64, String>`
Calculates the nth Fibonacci number recursively (slower for large n).

```rust
use gh_actions::calculate_fibonacci_recursive;

let result = calculate_fibonacci_recursive(10)?;
```

#### `factorial(n: u32) -> Result<u64, String>`
Calculates factorial of n.

```rust
use gh_actions::factorial;

let result = factorial(5)?; // 120
```

#### `is_prime(n: u32) -> bool`
Checks if a number is prime.

```rust
use gh_actions::is_prime;

assert!(is_prime(17));
assert!(!is_prime(15));
```

#### `gcd(a: u32, b: u32) -> u32`
Calculates greatest common divisor.

```rust
use gh_actions::gcd;

let result = gcd(48, 18); // 6
```

#### `lcm(a: u32, b: u32) -> Result<u32, String>`
Calculates least common multiple.

```rust
use gh_actions::lcm;

let result = lcm(12, 15)?; // 60
```

### String Utilities

#### `validate_email(email: &str) -> bool`
Validates email format.

```rust
use gh_actions::validate_email;

assert!(validate_email("user@example.com"));
assert!(!validate_email("invalid-email"));
```

#### `reverse_string(s: &str) -> String`
Reverses a string.

```rust
use gh_actions::reverse_string;

let reversed = reverse_string("hello"); // "olleh"
```

#### `is_palindrome(s: &str) -> bool`
Checks if a string is a palindrome.

```rust
use gh_actions::is_palindrome;

assert!(is_palindrome("racecar"));
assert!(!is_palindrome("hello"));
```

#### `count_words(s: &str) -> usize`
Counts words in a string.

```rust
use gh_actions::count_words;

let count = count_words("hello world rust"); // 3
```

## Error Handling

All fallible operations return `Result<T, E>` types:

- `Result<(), String>`: Operations that can fail with descriptive error messages
- `Result<T, Box<dyn std::error::Error>>`: File I/O operations with system errors

### Common Error Cases

```rust
// Duplicate user ID
manager.add_user(user)?; // Error: "User with ID 1 already exists"

// Invalid email
let invalid_user = User {
    id: 2,
    name: "Test".to_string(),
    email: "invalid-email".to_string(),
    active: true,
};
manager.add_user(invalid_user)?; // Error: "Invalid email format"

// Number too large for Fibonacci
calculate_fibonacci(100)?; // Error: "Number too large for u64"
```

## Complete Example

```rust
use gh_actions::{UserManager, User, calculate_fibonacci, validate_email};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = UserManager::new();
    
    // Add users
    let users = vec![
        User { id: 1, name: "Alice".to_string(), 
               email: "alice@example.com".to_string(), active: true },
        User { id: 2, name: "Bob".to_string(), 
               email: "bob@example.com".to_string(), active: true },
    ];
    
    for user in users {
        manager.add_user(user)?;
    }
    
    // List users
    for user in manager.get_users() {
        println!("User: {} ({})", user.name, user.email);
        
        // Validate email
        if validate_email(&user.email) {
            println!("  Email is valid");
        }
        
        // Calculate Fibonacci for user ID
        match calculate_fibonacci(user.id) {
            Ok(fib) => println!("  Fibonacci({}) = {}", user.id, fib),
            Err(e) => println!("  Fibonacci error: {}", e),
        }
    }
    
    // Save to file
    manager.save_to_file("users.json")?;
    println!("Users saved to file");
    
    Ok(())
}
```

## Thread Safety

⚠️ **Note**: The current implementation is **not thread-safe**. `UserManager` should not be shared between threads without proper synchronization (e.g., `Mutex` or `RwLock`).

For concurrent usage:

```rust
use std::sync::{Arc, Mutex};
use gh_actions::UserManager;

let manager = Arc::new(Mutex::new(UserManager::new()));

// In different threads
let manager_clone = Arc::clone(&manager);
std::thread::spawn(move || {
    let mut manager = manager_clone.lock().unwrap();
    // Use manager...
});
```