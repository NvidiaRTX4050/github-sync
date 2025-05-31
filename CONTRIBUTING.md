# 🤝 Contributing to GitHub Sync

First off, thank you for considering contributing to GitHub Sync! It's people like you that make GitHub Sync such a great tool.

## 📝 Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## 🚀 How Can I Contribute?

### 🐛 Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps to reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed after following the steps
* Explain which behavior you expected to see instead and why
* Include details about your configuration and environment

### 💡 Suggesting Enhancements

If you have a suggestion for the project, we'd love to hear it! Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* A clear and descriptive title
* A detailed description of the proposed functionality
* Any possible drawbacks or challenges you foresee
* If possible, a rough proposal of how to implement it

### 🔧 Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code follows the existing style
6. Issue that pull request!

## 🛠️ Development Setup

1. Clone the repository
```bash
git clone https://github.com/yourusername/github-sync
cd github-sync
```

2. Install dependencies
```bash
cargo build
```

3. Run tests
```bash
cargo test
```

## 📋 Style Guide

* Use the built-in Rust formatter: `cargo fmt`
* Follow Rust naming conventions
* Write descriptive commit messages
* Comment your code when necessary
* Add tests for new functionality

## 🎯 Project Structure

```
github-sync/
├── src/
│   ├── commands/     # CLI command implementations
│   ├── git.rs       # Git operations
│   ├── github.rs    # GitHub API interactions
│   ├── watcher.rs   # File system monitoring
│   └── main.rs      # Entry point
├── tests/           # Integration tests
└── examples/        # Usage examples
```

## 📚 Documentation

* Comment your code using rustdoc conventions
* Update README.md if you change functionality
* Add examples for new features
* Keep API documentation up to date

## ❓ Questions?

Feel free to open an issue with your question or reach out to the maintainers directly.

---

<div align="center">

Thank you for contributing to GitHub Sync! 🎉

</div> 