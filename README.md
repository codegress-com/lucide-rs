# 🎨 Lucide Icons for Rust

Beautiful, customizable SVG icons for Rust web applications. Built for modern frameworks like Leptos, Yew, Dioxus, and Sycamore.

[![Crates.io](https://img.shields.io/crates/v/lucide.svg)](https://crates.io/crates/lucide)
[![Documentation](https://docs.rs/lucide/badge.svg)](https://docs.rs/lucide)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/vaclavhrach/lucide-rs/workflows/CI/badge.svg)](https://github.com/vaclavhrach/lucide-rs/actions)

## ✨ Features

- 🎯 **1000+ Icons** - Comprehensive icon library from [Lucide](https://lucide.dev)
- 🦀 **Rust Native** - Built specifically for Rust web frameworks  
- 🌳 **Tree Shakeable** - Category-based features for optimal bundle size
- ⚡ **Fast** - Optimized SVG components with minimal overhead
- 🎨 **Customizable** - Easy styling with CSS classes and properties
- 🔧 **Framework Support** - Leptos, Yew, Dioxus, and Sycamore

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
# Choose your framework
lucide = { version = "0.1.0", features = ["leptos"] }
# lucide = { version = "0.1.0", features = ["yew"] }  
# lucide = { version = "0.1.0", features = ["dioxus"] }
# lucide = { version = "0.1.0", features = ["sycamore"] }
```

Use in your components:

```rust
use lucide::*;

// Leptos
view! {
    <lucide::Home class="w-6 h-6 text-gray-600" />
    <lucide::Search class="w-4 h-4" />
    <lucide::Settings class="w-5 h-5 text-blue-500" />
}

// Yew
html! {
    <lucide::Home class="w-6 h-6 text-gray-600" />
    <lucide::Search size=16 />
    <lucide::Settings color="blue" />
}

// Dioxus  
rsx! {
    lucide::Home { class: "w-6 h-6 text-gray-600" }
    lucide::Search { size: 16 }
    lucide::Settings { color: "blue" }
}

// Sycamore
view! {
    lucide::Home(class="w-6 h-6 text-gray-600")
    lucide::Search(size=16)  
    lucide::Settings(color="blue")
}
```

## 📦 Optimized Bundle Size

Include only the icon categories you need:

```toml
[dependencies]
lucide = { 
    version = "0.1.0", 
    features = ["leptos", "navigation", "files", "communication"] 
}
```

### Available Categories

| Category | Icons | Description |
|----------|-------|-------------|
| `navigation` | 50+ | Home, menu, arrows, chevrons |
| `files` | 40+ | Folder, document, download, upload |
| `communication` | 30+ | Mail, phone, message, video |
| `multimedia` | 25+ | Play, pause, volume, camera |
| `arrows` | 20+ | Directional indicators |
| `tools` | 35+ | Settings, edit, search, filter |

[📋 **View All Categories & Icons →**](ICONS.md)

### Meta Features

```toml
# Common combinations
features = ["leptos", "essentials"]     # Core navigation + files  
features = ["leptos", "web-app"]        # Perfect for web applications
features = ["leptos", "mobile-app"]     # Optimized for mobile apps
features = ["leptos", "all-icons"]      # Everything (larger bundle)
```

## 🖼️ Icon Properties

All icon components support these props:

- `size: usize` (default: 24) - Icon size in pixels
- `color: String` (default: "currentColor") - Stroke color
- `fill: String` (default: "none") - Fill color  
- `stroke_width: usize` (default: 2) - Stroke width
- `absolute_stroke_width: bool` (default: false) - Whether to scale stroke with icon
- `class: Option<String>` - CSS class names
- Framework-specific props like `node_ref`, `style`, etc.

## 🛠️ Development

### Generating Icons

To regenerate all icon components from the latest Lucide icons:

```bash
./scripts/generate-icons.sh
```

This script will:
1. 📥 Download the latest SVG icons from Lucide
2. 🦀 Generate Rust components for all frameworks
3. 📝 Update module exports
4. 🧹 Clean up temporary files

### Project Structure

```
lucide-rs/
├── packages/lucide/          # Main crate
│   ├── src/
│   │   ├── dioxus/          # Dioxus components
│   │   ├── leptos/          # Leptos components  
│   │   ├── yew/             # Yew components
│   │   ├── sycamore/        # Sycamore components
│   │   └── bin/             # Code generation binary
│   └── Cargo.toml
└── scripts/                 # Build scripts
    └── generate-icons.sh    # Icon generation script
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Icons from [Lucide](https://lucide.dev/)
- Inspired by [RustForWeb/lucide](https://github.com/RustForWeb/lucide)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
