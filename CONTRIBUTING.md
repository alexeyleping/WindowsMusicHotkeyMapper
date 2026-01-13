# Contributing to Windows Music Hotkey Mapper

Thank you for your interest in contributing! Here's how you can help.

## Development Setup

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/WindowsMusicHotkeyMapper.git
   cd WindowsMusicHotkeyMapper
   ```

3. Install platform-specific dependencies:

   **Linux:**
   ```bash
   sudo apt install playerctl libx11-dev libxdo-dev
   ```

   **Windows:**
   No additional dependencies needed.

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=info cargo run
```

### Testing

```bash
# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Making Changes

1. Fork the repository
2. Create a new branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Test your changes thoroughly
5. Commit with a clear message
6. Push to your fork
7. Create a Pull Request

## Code Style

- Use `cargo fmt` to format your code
- Run `cargo clippy` and fix any warnings
- Write clear, descriptive commit messages
- Add comments for complex logic

## Commit Message Format

```
<type>: <subject>

<body>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

## Pull Request Process

1. Update README.md if needed
2. Ensure all tests pass
3. Update documentation for new features
4. Request review from maintainers

## Questions?

Open an issue for discussion before making major changes.
