# Development Guide

## Contributing to gh_actions

This guide covers the development workflow, coding standards, and contribution process for the gh_actions project.

## Development Workflow

### Git Flow

The project uses a **light git flow** approach:

1. **Main Branch**: `main` - production-ready code
2. **Feature Branches**: `feature/description` - new features
3. **Bug Fix Branches**: `fix/description` - bug fixes
4. **Claude Branches**: `claude/issue-X-YYYYMMDD-HHMM` - AI-assisted development

### Branch Naming Conventions

- Features: `feature/user-authentication`
- Bug fixes: `fix/email-validation-regex`  
- Improvements: `improvement/performance-optimization`
- Documentation: `docs/api-documentation`

## Code Standards

### Rust Conventions

- **Formatting**: Use `cargo fmt` before committing
- **Linting**: Address all `cargo clippy` warnings
- **Testing**: Maintain 100% test coverage for new features
- **Documentation**: Document all public APIs with doc comments

### Code Style

```rust
// Good: Clear function signature with documentation
/// Calculates the nth Fibonacci number iteratively.
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// Returns `Ok(fibonacci_number)` or `Err(error_message)`
pub fn calculate_fibonacci(n: u32) -> Result<u64, String> {
    // Implementation
}

// Good: Comprehensive error handling
match user_manager.add_user(user) {
    Ok(_) => println!("User added successfully!"),
    Err(e) => {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### Testing Strategy

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user_success() {
        let mut manager = UserManager::new();
        let user = create_test_user(1);
        
        assert!(manager.add_user(user.clone()).is_ok());
        assert_eq!(manager.count(), 1);
    }
}
```

#### Integration Tests
Located in `tests/` directory:
- `integration_test.rs`: Cross-module functionality
- `cli_tests.rs`: Command-line interface testing
- `property_tests.rs`: Property-based testing

#### Benchmarks
Located in `benches/` directory for performance-critical code:
```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| calculate_fibonacci(20))
    });
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
```

## Project Structure

### Core Modules

```
src/
├── main.rs           # CLI entry point and command handling
├── lib.rs            # Public API exports and integration tests
├── user_manager.rs   # User CRUD operations and persistence
└── utils.rs          # Mathematical and validation utilities
```

### Key Design Patterns

1. **Result-based Error Handling**: All fallible operations return `Result<T, E>`
2. **Builder Pattern**: For complex configurations
3. **RAII**: Automatic resource management
4. **Composition over Inheritance**: Favor traits and composition

## Adding New Features

### 1. Design Phase

- Create GitHub issue describing the feature
- Design the API and data structures
- Consider backward compatibility
- Plan test scenarios

### 2. Implementation Phase

```bash
# Create feature branch
git checkout -b feature/new-feature

# Implement with tests
cargo test

# Run benchmarks if performance-critical
cargo bench

# Format and lint
cargo fmt
cargo clippy
```

### 3. Documentation Phase

- Update relevant documentation files
- Add doc comments for public APIs
- Update examples in usage.md

### 4. Pull Request

- Create PR with descriptive title and body
- Include test results and benchmark data
- Reference related issues
- Request review from maintainers

## Quality Assurance

### Pre-commit Checklist

- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation updated
- [ ] Benchmarks run (if applicable): `cargo bench`

### CI/CD Pipeline

The project uses GitHub Actions for:

1. **Build Verification**: Multi-platform builds
2. **Test Execution**: Unit, integration, and property tests
3. **Code Quality**: Formatting and linting checks
4. **Security Scanning**: Dependency vulnerability checks
5. **Performance Monitoring**: Benchmark regression detection

## Debugging

### Local Development

```bash
# Debug build with symbols
cargo build

# Run with debug output
RUST_LOG=debug cargo run

# Use rust-gdb for debugging
rust-gdb target/debug/gh_actions
```

### Performance Profiling

```bash
# Profile with perf (Linux)
cargo build --release
perf record target/release/gh_actions [args]
perf report

# Memory profiling with valgrind
valgrind --tool=massif target/release/gh_actions [args]
```

## Release Process

1. **Version Bump**: Update `Cargo.toml` version
2. **Changelog**: Update `CHANGELOG.md` with changes
3. **Tag Release**: `git tag v1.x.x && git push --tags`
4. **Build Release**: `cargo build --release`
5. **GitHub Release**: Create release with binaries

## Future Architecture

### Planned Refactoring

- **Web Interface**: Actix-web or Axum backend
- **WASM Frontend**: Yew or Leptos framework
- **Database Layer**: SQLite with migrations
- **Task Management**: Priority queues and scheduling
- **KPI Dashboard**: Real-time metrics and charts

### Migration Strategy

1. Maintain CLI compatibility
2. Add web API alongside CLI
3. Implement WASM frontend
4. Migrate data storage to SQLite
5. Add advanced task management features

## Getting Help

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Architecture and design questions
- **Code Reviews**: Ask maintainers for guidance
- **Rust Community**: [users.rust-lang.org](https://users.rust-lang.org/)