# 🎨 Lucide Icons for Rust

Beautiful, customizable SVG icons for Rust web applications. Built for modern frameworks like Leptos, Yew, Dioxus, and Sycamore.

[![Crates.io](https://img.shields.io/crates/v/lucide-rust.svg)](https://crates.io/crates/lucide-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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
lucide-rust = { version = "0.1.0", features = ["leptos"] }
# lucide-rust = { version = "0.1.0", features = ["yew"] }  
# lucide-rust = { version = "0.1.0", features = ["dioxus"] }
# lucide-rust = { version = "0.1.0", features = ["sycamore"] }
```

Use in your components:

```rust
use lucide_rust::*;

// Leptos
view! {
    <Home class="w-6 h-6 text-gray-600" />
    <Search class="w-4 h-4" />
    <Settings class="w-5 h-5 text-blue-500" />
}

// Yew
html! {
    <Home class="w-6 h-6 text-gray-600" />
    <Search size=16 />
    <Settings color="blue" />
}

// Dioxus  
rsx! {
    Home { class: "w-6 h-6 text-gray-600" }
    Search { size: 16 }
    Settings { color: "blue" }
}

// Sycamore
view! {
    Home(class="w-6 h-6 text-gray-600")
    Search(size=16)  
    Settings(color="blue")
}
```

## 📦 Optimized Bundle Size

Include only the icon categories you need:

```toml
[dependencies]
lucide-rust = { 
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

[📋 **View All Categories & Icons →**](https://github.com/codegress-com/lucide-rust/main/ICONS.md)

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

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/codegress-com/lucide-rust/main/LICENSE) file for details.

## 🙏 Acknowledgments

- Icons from [Lucide](https://lucide.dev/)
