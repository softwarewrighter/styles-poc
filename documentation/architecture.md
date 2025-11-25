# Architecture Document: Styles POC

## System Overview

```
+-------------------------------------------------------------+
|                        Browser                               |
|  +--------------------------------------------------------+ |
|  |                    index.html                          | |
|  |  +--------------+  +---------------------------------+ | |
|  |  | CSS Themes   |  |         WASM Module             | | |
|  |  | - light.css  |  |  +-------------------------+    | | |
|  |  | - dark.css   |  |  |      Yew App            |    | | |
|  |  +--------------+  |  |  +-----------------+    |    | | |
|  |                    |  |  |   Components    |    |    | | |
|  |                    |  |  |  - Layout       |    |    | | |
|  |                    |  |  |  - ThemeSelect  |    |    | | |
|  |                    |  |  |  - Content      |    |    | | |
|  |                    |  |  +-----------------+    |    | | |
|  |                    |  +-------------------------+    | | |
|  |                    +---------------------------------+ | |
|  +--------------------------------------------------------+ |
+-------------------------------------------------------------+
```

## Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| UI Framework | Yew | Component-based UI in Rust |
| Compilation | wasm-bindgen | Rust to WASM compilation |
| Build Tool | Trunk | Asset bundling and dev server |
| Styling | Vanilla CSS | Theme stylesheets |
| Browser API | web-sys | DOM manipulation for theme switching |

## Component Architecture

### App (Root Component)
- Manages global state (current theme)
- Renders the overall page structure
- Handles theme change events

### Layout Components
- **Header** - Contains logo/title and theme selector
- **Nav** - Left sidebar navigation
- **Main** - Primary content wrapper
- **Aside** - Right sidebar for supplementary info
- **Footer** - Page footer

### Functional Components
- **ThemeSelector** - Dropdown for theme selection
- **Section** - Reusable content section
- **Article** - Reusable article component

## Theme System

### How Theme Switching Works

1. CSS files are loaded via `<link>` tags in the HTML
2. Theme selector triggers a callback to the App component
3. App component updates state with new theme name
4. A Rust function using `web-sys` updates the CSS link href
5. Browser loads new stylesheet and re-renders

### CSS File Structure

```
styles/
+-- simple-light.css
+-- simple-dark.css
```

Each theme CSS file contains:
- CSS custom properties (variables) for colors
- Layout styles using CSS Grid/Flexbox
- Typography styles
- Component-specific styles

## Data Flow

```
User Action (Theme Select)
        |
        v
  ThemeSelector Component
        |
        v
  Callback to App Component
        |
        v
  Update App State
        |
        v
  DOM Update (web-sys)
        |
        v
  Browser Loads New CSS
```

## File Structure

```
styles-poc/
+-- Cargo.toml
+-- index.html
+-- documentation/
|   +-- architecture.md
|   +-- design.md
|   +-- plan.md
|   +-- prd.md
|   +-- status.md
+-- src/
|   +-- main.rs
|   +-- app.rs
|   +-- components/
|   |   +-- mod.rs
|   |   +-- header.rs
|   |   +-- nav.rs
|   |   +-- main_content.rs
|   |   +-- aside.rs
|   |   +-- footer.rs
|   |   +-- theme_selector.rs
|   +-- lorem.rs
+-- styles/
    +-- simple-light.css
    +-- simple-dark.css
```

## Build Process

1. `trunk serve` for development
2. `trunk build --release` for production
3. Trunk handles:
   - Rust compilation to WASM
   - Asset copying (CSS files)
   - HTML processing
   - Dev server with hot reload
