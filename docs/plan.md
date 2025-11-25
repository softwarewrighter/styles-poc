# Implementation Plan: Styles POC

## Phase 1: Project Setup

### Tasks
1. [x] Create documentation structure
2. Initialize Cargo.toml with dependencies
3. Create basic file structure
4. Set up index.html with Trunk configuration

### Dependencies
```toml
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlLinkElement", "Window"] }
```

## Phase 2: Core Application

### Tasks
1. Create main.rs entry point
2. Create App component with theme state
3. Implement basic HTML structure with semantic elements

### Deliverables
- Running Yew application
- Basic page layout visible

## Phase 3: Components

### Tasks
1. Create Header component
2. Create Nav component
3. Create MainContent component with Section and Article
4. Create Aside component
5. Create Footer component
6. Create ThemeSelector component

### Component Hierarchy
```
App
+-- Header
|   +-- ThemeSelector
+-- Nav
+-- MainContent
|   +-- Section
|   |   +-- Article (multiple)
|   +-- Section
|       +-- Article (multiple)
+-- Aside
+-- Footer
```

## Phase 4: Lorem Ipsum Content

### Tasks
1. Create lorem.rs module with text constants
2. Populate all content areas with filler text
3. Include various text elements (paragraphs, lists, headings)

## Phase 5: Theme System

### Tasks
1. Create styles/ directory
2. Implement simple-light.css
3. Implement simple-dark.css
4. Implement theme switching logic using web-sys
5. Wire up ThemeSelector to theme switching

### Theme Switching Implementation
```rust
fn set_theme(theme: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let link = document.get_element_by_id("theme-stylesheet").unwrap();
    let link: web_sys::HtmlLinkElement = link.dyn_into().unwrap();
    link.set_href(&format!("styles/{}.css", theme));
}
```

## Phase 6: Polish

### Tasks
1. Ensure both themes look complete
2. Test theme switching
3. Update status document
4. Add any missing lorem ipsum content

## File Creation Order

1. `docs/` - Documentation (this phase)
2. `Cargo.toml` - Dependencies
3. `index.html` - Entry point
4. `src/main.rs` - App entry
5. `src/app.rs` - Root component
6. `src/components/mod.rs` - Module exports
7. `src/components/*.rs` - Individual components
8. `src/lorem.rs` - Filler content
9. `styles/simple-light.css` - Light theme
10. `styles/simple-dark.css` - Dark theme

## Testing Strategy

### Manual Testing
1. Run `trunk serve`
2. Verify page loads
3. Check all semantic elements render
4. Test theme dropdown changes stylesheet
5. Verify both themes apply correctly

### Future: Automated Testing
- Component tests with Yew's testing utilities
- E2E tests with browser automation

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Theme flash on load | Set default theme in HTML |
| WASM load time | Keep bundle small, show loading state |
| CSS not loading | Use Trunk's asset handling correctly |

## Success Metrics

- [ ] Application compiles without errors
- [ ] Page displays all HTML5 semantic elements
- [ ] Theme selector changes the active stylesheet
- [ ] Light theme is readable and clean
- [ ] Dark theme is readable and clean
- [ ] No JavaScript required (except WASM bootstrap)
