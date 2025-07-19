# gh_actions Documentation

## Overview

`gh_actions` is a Rust CLI application designed for testing GitHub Actions integration and workflows. The project serves as both a functional CLI tool and a testbed for automated CI/CD processes.

## Project Vision

This project is evolving from a simple GitHub Actions testing tool into a comprehensive task management web application featuring:

- 🦀 **Rust backend** with CLI interface
- 🌐 **WebAssembly (WASM)** frontend integration  
- 📊 **SQLite database** for task persistence
- 📈 **KPI tracking** (time to completion, estimates, remaining tasks)
- 🔄 **Light Git Flow** development workflow

## Current Features

### User Management System
- Complete CRUD operations for users
- Email validation and data persistence
- User activation/deactivation
- JSON file-based storage

### Mathematical Utilities
- Fibonacci number calculation (iterative and recursive)
- Prime number checking
- String manipulation utilities
- Email format validation

### CLI Interface
- User management commands (`user add`, `user list`)
- Mathematical operations (`fib`)
- Comprehensive help system

## Architecture

```
gh_actions/
├── src/
│   ├── main.rs           # CLI interface and command handling
│   ├── lib.rs            # Library exports and integration tests
│   ├── user_manager.rs   # User management system
│   └── utils.rs          # Mathematical and validation utilities
├── tests/                # Integration and property tests
├── benches/             # Performance benchmarks
└── docs/                # Project documentation
```

## Development Philosophy

- **Quality First**: State-of-the-art quality standards are maintained
- **Test-Driven**: Comprehensive unit, integration, and property tests
- **Performance Aware**: Benchmarking for critical operations
- **Git Flow**: Lightweight branching strategy with PR reviews

## Quick Start

See [Setup Guide](setup.md) for installation instructions.
See [Usage Guide](usage.md) for CLI command examples.
See [Development Guide](development.md) for contributing guidelines.

## Roadmap

Current project status and planned features can be found in [todos.md](todos.md).

## Links

- [GitHub Repository](https://github.com/DScudeler/gh_actions)
- [Issue Tracker](https://github.com/DScudeler/gh_actions/issues)