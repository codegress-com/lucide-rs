# Lucide Rust - Leptos CSR Example

This example demonstrates how to use the `lucide-rust` icon library with Leptos in a Client-Side Rendered (CSR) application.

## Features

- **Comprehensive Icon Testing**: Displays a wide variety of Lucide icons organized by category
- **Rust Keyword Compatibility**: Specifically tests icons that were previously problematic due to Rust keyword conflicts (Box, Move, Type)
- **Responsive Design**: Icons are displayed in a responsive grid layout using Tailwind CSS
- **Clean Component Usage**: Shows how to import and use lucide-rust components in Leptos

## Key Test Areas

### 🚨 Rust Keyword Icons Test
The example prominently features a test section for icons whose names conflict with Rust keywords:
- `Box` (from `box_icon` module)
- `Move` (from `move_icon` module) 
- `Type` (from `type_icon` module)

These icons previously caused compilation errors but now work perfectly thanks to the `_icon` suffix naming strategy in the code generator.

### Icon Categories Tested
- Basic Icons (Home, Search, Heart, Star, User, etc.)
- Media Icons (Camera, Music, Play, Pause, Volume, etc.)
- System Icons (WiFi, Battery, Bluetooth, Settings, etc.)
- File & Folder Icons (File, Folder, Download, Upload, Save, etc.)
- Navigation Icons (Arrows, Chevrons in all directions)
- Action Icons (Check, X, Plus, Minus, Edit, Trash, Eye, etc.)
- Security Icons (Lock, Shield, Globe)
- Tech Icons (Code, Database, Server)

## Running the Example

```bash
# Install dependencies (if needed)
npm install

# Run the development server
trunk serve

# Or build for production
trunk build --release
```

## Usage Pattern

```rust
use lucide_rust::leptos::{
    search_icon::Search,
    heart_icon::Heart,
    // ... other icons
};

#[component]
fn MyComponent() -> impl IntoView {
    view! {
        <div>
            <Search size=32 />
            <Heart size=24 />
        </div>
    }
}
```

## Key Learnings

1. **Module Naming**: All icon modules use the `_icon` suffix (e.g., `box_icon`, `move_icon`) to avoid Rust keyword conflicts
2. **Component Names**: Component names remain clean and intuitive (e.g., `Box`, `Move`, `Type`)
3. **Import Pattern**: Import icons from their specific modules: `use lucide_rust::leptos::box_icon::Box;`
4. **Props**: All icons support standard props like `size`, `color`, `fill`, `stroke_width`, etc.

This example serves as both a comprehensive test suite and a reference implementation for using lucide-rust icons in Leptos applications.
