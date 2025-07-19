# Usage Guide

## CLI Commands

The `gh_actions` CLI provides several commands for user management and mathematical operations.

## Running the Application

```bash
# Development build
cargo run

# Release build
cargo run --release

# With arguments
cargo run -- [COMMAND] [ARGS]

# Built binary (after cargo build --release)
./target/release/gh_actions [COMMAND] [ARGS]
```

## Command Overview

### Help Command

```bash
gh_actions --help
```

Shows available commands and options.

### User Management

#### Add User

```bash
gh_actions user add <ID> <NAME> <EMAIL>
```

**Examples:**
```bash
gh_actions user add 1 "John Doe" "john@example.com"
gh_actions user add 2 "Alice Smith" "alice@company.org"
```

**Validation:**
- ID must be a positive integer
- Name cannot be empty
- Email must be valid format

#### List Users

```bash
gh_actions user list
```

**Output format:**
```
Users:
  ID: 1, Name: John Doe, Email: john@example.com, Active: true
  ID: 2, Name: Alice Smith, Email: alice@company.org, Active: true
```

### Mathematical Operations

#### Fibonacci Calculation

```bash
gh_actions fib <NUMBER>
```

**Examples:**
```bash
gh_actions fib 10        # Output: Fibonacci of 10 is: 55
gh_actions fib 20        # Output: Fibonacci of 20 is: 6765
gh_actions fib 0         # Output: Fibonacci of 0 is: 0
gh_actions fib 1         # Output: Fibonacci of 1 is: 1
```

**Limitations:**
- Maximum input: 93 (due to u64 overflow protection)
- Numbers beyond 93 will return an error

## Error Handling

The application provides clear error messages for common issues:

### User Management Errors

```bash
# Duplicate ID
gh_actions user add 1 "Test" "test@example.com"
# Error: User with ID 1 already exists

# Invalid email
gh_actions user add 3 "Test" "invalid-email"
# Error: Invalid email format

# Empty name
gh_actions user add 4 "" "test@example.com"
# Error: User name cannot be empty

# Invalid ID format
gh_actions user add abc "Test" "test@example.com"
# Error: Invalid user ID
```

### Mathematical Errors

```bash
# Number too large
gh_actions fib 100
# Error: Number too large for u64

# Invalid number format
gh_actions fib abc
# Error: Invalid number
```

## Exit Codes

- **0**: Success
- **1**: Error occurred (invalid input, operation failed, etc.)

## Integration Examples

### Shell Scripts

```bash
#!/bin/bash
# Add multiple users
users=(
    "1 'John Doe' john@example.com"
    "2 'Alice Smith' alice@company.org"
    "3 'Bob Wilson' bob@test.net"
)

for user in "${users[@]}"; do
    gh_actions user add $user
    if [ $? -eq 0 ]; then
        echo "Added user: $user"
    else
        echo "Failed to add user: $user"
    fi
done

# List all users
echo "All users:"
gh_actions user list
```

### JSON Processing

The UserManager supports JSON file I/O (programmatic access only):

```rust
// Save users to file
user_manager.save_to_file("users.json")?;

// Load users from file
user_manager.load_from_file("users.json")?;
```

## Performance Notes

- User operations are O(n) for search/update/delete
- Fibonacci calculation is O(n) iterative implementation
- JSON serialization scales with number of users
- For large datasets, consider the programmatic API instead of CLI

## Next Steps

- See [Development Guide](development.md) for extending functionality
- Check [API Documentation](api.md) for programmatic usage
- Review [todos.md](todos.md) for planned features