# WASM Task Manager

A minimal, fast task management web application built with Rust and WebAssembly.

## 🚀 Features

- **Fast WASM Performance**: Core task management logic compiled to WebAssembly
- **Simple Task Management**: Add, complete, and delete tasks
- **Real-time Statistics**: Track total, completed, and remaining tasks
- **Responsive Design**: Modern UI that works on all devices
- **Zero Dependencies**: No external databases - everything runs in the browser

## 🛠️ Development

### Prerequisites
- Rust (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Local Build
```bash
# Build WASM package
./build-wasm.sh

# Serve locally
cd www && python3 -m http.server 8000
```

### Testing
```bash
cargo test --verbose
cargo clippy
cargo fmt
```

## 🏗️ Architecture

- **Rust Backend** (`src/task.rs`): Core task management logic
- **WASM Bindings** (`src/wasm.rs`): JavaScript interoperability layer  
- **Web Frontend** (`www/`): HTML/CSS/JS interface
- **CI/CD Templates**: Ready-to-use GitHub Actions workflow templates

## 🚢 Deployment

### GitHub Actions Setup
This project includes ready-to-use GitHub Actions workflows for:
- **Continuous Integration**: Automated testing, linting, and WASM builds
- **Continuous Deployment**: Auto-deploy to GitHub Pages on main branch updates

**Quick Setup:**
```bash
# Copy workflow templates to .github/workflows/
mkdir -p .github/workflows
cp ci-test-workflow.yml .github/workflows/ci.yml
cp cd-deploy-workflow.yml .github/workflows/deploy.yml
```

For detailed setup instructions, see [`GITHUB_ACTIONS_SETUP.md`](./GITHUB_ACTIONS_SETUP.md).

## 📋 Task Management

The application provides a clean interface for:
- Creating tasks with title and description
- Marking tasks as complete/incomplete
- Deleting tasks
- Viewing completion statistics

Built as an MVP with Rust + WASM for maximum performance and minimal complexity.
