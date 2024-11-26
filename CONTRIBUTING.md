# Contributing to Search in Terminal

Thank you for your interest in contributing to Search in Terminal! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/search-in-terminal
   cd search-in-terminal
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/zykowal/search-in-terminal
   ```
4. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Process

1. Install development dependencies:
   ```bash
   cargo install cargo-watch
   cargo install cargo-edit
   ```

2. Run tests:
   ```bash
   cargo test
   ```

3. Run the development server with auto-reload:
   ```bash
   cargo watch -x run
   ```

## Pull Request Process

1. Update the README.md with details of changes to the interface
2. Update the documentation in the `docs` directory if needed
3. Run all tests and ensure they pass
4. Update the version numbers following [Semantic Versioning](https://semver.org/)
5. Create a Pull Request with a clear title and description

### PR Title Format

Use one of these prefixes:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `test:` for test-related changes
- `refactor:` for code refactoring
- `style:` for formatting changes
- `chore:` for maintenance tasks

Example: `feat: add support for Baidu search engine`

## Coding Standards

### Rust Style Guidelines

1. Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
2. Use `rustfmt` to format your code:
   ```bash
   cargo fmt
   ```
3. Use `clippy` to catch common mistakes:
   ```bash
   cargo clippy
   ```

### Code Organization

- Keep functions small and focused
- Use meaningful variable and function names
- Add comments for complex logic
- Use type annotations when type inference isn't clear

### Error Handling

- Use the `thiserror` crate for error definitions
- Provide meaningful error messages
- Handle all potential error cases
- Use `Result` types appropriately

## Testing Guidelines

1. Write unit tests for new functionality
2. Include integration tests when adding features
3. Test edge cases and error conditions
4. Use meaningful test names that describe the scenario

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_with_valid_query() {
        // Test implementation
    }

    #[test]
    fn test_search_with_empty_query() {
        // Test implementation
    }
}
```

## Documentation

1. Add rustdoc comments for public APIs
2. Include examples in documentation
3. Update relevant documentation files
4. Add inline comments for complex logic

### Documentation Structure

- API documentation in `docs/API.md`
- Configuration guide in `docs/CONFIG.md`
- User guide in README.md
- Code documentation using rustdoc

### Example Documentation

```rust
/// Performs a search using the specified search engine
///
/// # Arguments
///
/// * `query` - The search query string
/// * `start` - The starting position for pagination
///
/// # Returns
///
/// A Result containing a vector of SearchResult or an error
///
/// # Examples
///
/// ```
/// let engine = SearchEngine::Google(Google);
/// let results = engine.search("rust programming", 0).await?;
/// ```
pub async fn search(&self, query: &str, start: u16) -> Result<Vec<SearchResult>>
```

## Questions or Problems?

Feel free to:
- Open an issue for bugs
- Start a discussion for features
- Ask questions in discussions
- Join our community chat
