# Contributing to Lucide Rust

Thank you for your interest in contributing to Lucide Rust! 🎨

## Quick Start

1. **Fork** the repository
2. **Clone** your fork locally
3. **Create** a feature branch (`git checkout -b feat/amazing-feature`)
4. **Make** your changes
5. **Test** your changes (`cargo test`)
6. **Commit** and **push** to your fork
7. **Open** a pull request

## Development Setup

### Prerequisites

- **Rust** 1.70+ with Cargo
- **Git** for version control

### Installation

```bash
git clone https://github.com/vaclavhrach/lucide-rs.git
cd lucide-rs

# Install dependencies and build
cargo build

# Run tests
cargo test

# Run examples
cd apps/examples/leptos-csr
cargo run
```

## Project Structure

```
lucide-rs/
├── packages/lucide/                # Main library
│   ├── src/
│   │   ├── bin/generate-icons.rs   # Icon generation script
│   │   ├── dioxus/                 # Dioxus components
│   │   ├── leptos/                 # Leptos components  
│   │   ├── sycamore/               # Sycamore components
│   │   └── yew/                    # Yew components
│   └── icons/                      # SVG source files
└── apps/examples/                  # Example applications
    ├── dioxus/
    ├── leptos/
    ├── sycamore/
    └── yew/
```

## How to Contribute

### 🐛 Reporting Bugs

Use our [bug report template](.github/ISSUE_TEMPLATE/bug_report.yml) and include:

- **Framework** (Leptos, Yew, Dioxus, Sycamore)
- **Version** you're using
- **Steps to reproduce** the issue
- **Expected vs actual** behavior

### ✨ Requesting Features

Use our [feature request template](.github/ISSUE_TEMPLATE/feature_request.yml) for:

- **New icons** or icon categories
- **Framework integration** improvements
- **Developer experience** enhancements

### 🔧 Code Contributions

#### Adding New Icons

1. Add SVG files to `packages/lucide/icons/`
2. Add corresponding JSON metadata with categories
3. Run `cargo run --bin generate-icons --features codegen`
4. Test in example applications

#### Framework Integration

Each framework has its own module in `src/`. Follow the existing patterns:

- **Dioxus**: RSX syntax with props
- **Leptos**: Signals and reactive properties
- **Yew**: Html! macro with Properties
- **Sycamore**: View! macro with Props

#### Example Applications

Enhance examples in `apps/examples/` to showcase new features or fix issues.

## Code Standards

### Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --all-features -- -W warnings
```

### Testing

```bash
# Test core library
cargo test -p lucide --all-features

# Build examples
cargo build --workspace
```

## Icon Categories

Icons are organized into categories for tree-shaking:

- `navigation` - Navigation and UI controls
- `files` - File and folder operations
- `communication` - Mail, phone, messaging
- `multimedia` - Media playback and editing
- `arrows` - Directional indicators
- And many more... (see [ICONS.md](ICONS.md))

## Pull Request Guidelines

### Before Submitting

- [ ] Code follows project formatting (`cargo fmt`)
- [ ] All tests pass (`cargo test`)
- [ ] Examples build successfully
- [ ] Changes are documented

### PR Description

Use our [PR template](.github/pull_request_template.md) and include:

- **Clear description** of changes
- **Type of change** (bug fix, feature, docs, etc.)
- **Testing** performed
- **Breaking changes** (if any)

## Getting Help

- 💬 [GitHub Discussions](https://github.com/vaclavhrach/lucide-rs/discussions) for questions
- 🐛 [Issues](https://github.com/vaclavhrach/lucide-rs/issues) for bugs and features
- 📚 [Examples](apps/examples/) for implementation patterns

## Recognition

Contributors are automatically recognized in:

- GitHub contributor stats
- Release notes for significant contributions
- Special thanks for first-time contributors

---

**Happy contributing!** 🚀
