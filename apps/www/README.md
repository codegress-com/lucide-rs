# Lucide Rust Documentation Website

Beautiful documentation website for the Lucide Rust icon library, built with Leptos and Tailwind CSS.

## Features

- 🎨 **Beautiful Design** - Modern, responsive design with Tailwind CSS 4
- 🔍 **Icon Search** - Search and browse 1000+ Lucide icons
- 📚 **Comprehensive Docs** - Installation guides and framework-specific examples
- ⚡ **Fast & Modern** - Built with Leptos for optimal performance
- 📱 **Mobile Friendly** - Fully responsive design

## Development

### Prerequisites

- Rust (latest stable)
- Node.js 18+ 
- Trunk (`cargo install trunk`)
- Tailwind CSS CLI

### Setup

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Install Rust dependencies**:
   ```bash
   cargo install trunk
   ```

3. **Run development server**:
   ```bash
   # Terminal 1: Start Tailwind CSS watcher
   npm run css
   
   # Terminal 2: Start Trunk dev server
   npm run dev
   ```

4. **Open browser**: http://localhost:3000

### Build for Production

```bash
# Build CSS
npm run css:build

# Build Rust/WASM
npm run build
```

The production build will be in the `dist/` directory.

## Project Structure

```
apps/www/
├── src/
│   ├── components/     # Shared UI components
│   │   ├── navbar.rs
│   │   ├── footer.rs
│   │   ├── layout.rs
│   │   └── ...
│   ├── pages/          # Page components
│   │   ├── home.rs
│   │   ├── icons.rs
│   │   ├── docs.rs
│   │   └── mod.rs
│   ├── lib.rs          # Main app & routing
│   └── main.rs         # Entry point
├── public/
│   ├── style.css       # Tailwind imports
│   └── output.css      # Generated CSS
├── index.html          # HTML template
├── Trunk.toml          # Trunk configuration
├── tailwind.config.js  # Tailwind configuration
└── package.json        # Node.js dependencies
```

## Routes

- `/` - Homepage with hero and quick start
- `/icons` - Icon library with search and filtering  
- `/docs` - Documentation overview
- `/docs/installation` - Installation guide
- `/docs/leptos` - Leptos integration guide
- `/docs/dioxus` - Dioxus integration guide
- `/docs/yew` - Yew integration guide
- `/docs/sycamore` - Sycamore integration guide

## Contributing

See the main [CONTRIBUTING.md](../../CONTRIBUTING.md) for contribution guidelines.

## License

MIT License - see [LICENSE](../../LICENSE) for details.
