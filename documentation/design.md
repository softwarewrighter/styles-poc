# Design Document: Styles POC

## UI Layout Design

### Page Structure

```
+-------------------------------------------------------------------+
|                         <header>                                   |
|  Logo/Title                              [Theme: v simple-light]   |
+-------------------------------------------------------------------+
+----------+-------------------------------------------+-------------+
|          |              <main>                       |             |
|  <nav>   |  +-------------------------------------+  |  <aside>    |
|          |  |         <section>                   |  |             |
|  - Home  |  |  <article>                          |  |  Related    |
|  - About |  |    Title                            |  |  Links      |
|  - Blog  |  |    Lorem ipsum dolor sit...         |  |             |
|  - Work  |  |  </article>                         |  |  - Link 1   |
|  - Contact| |                                      |  |  - Link 2   |
|          |  |  <article>                          |  |  - Link 3   |
|          |  |    Title                            |  |             |
|          |  |    Lorem ipsum dolor sit...         |  |  Info Box   |
|          |  |  </article>                         |  |  ---------  |
|          |  | </section>                          |  |  Some       |
|          |  |                                      |  |  helpful    |
|          |  | <section>                           |  |  text here  |
|          |  |   ...                               |  |             |
|          |  | </section>                          |  |             |
|          |  +-------------------------------------+  |             |
+----------+-------------------------------------------+-------------+
+-------------------------------------------------------------------+
|                         <footer>                                   |
|  (c) 2024 Styles POC                              Built with Yew   |
+-------------------------------------------------------------------+
```

## Theme Design

### Simple Light Theme

| Element | Color | Notes |
|---------|-------|-------|
| Background | `#ffffff` | Pure white |
| Text | `#333333` | Dark gray for readability |
| Primary | `#0066cc` | Blue for links/accents |
| Secondary | `#f5f5f5` | Light gray for sidebars |
| Border | `#dddddd` | Subtle borders |
| Header/Footer BG | `#f0f0f0` | Slightly off-white |

### Simple Dark Theme

| Element | Color | Notes |
|---------|-------|-------|
| Background | `#1a1a1a` | Near black |
| Text | `#e0e0e0` | Light gray for readability |
| Primary | `#4da6ff` | Lighter blue for contrast |
| Secondary | `#2d2d2d` | Dark gray for sidebars |
| Border | `#404040` | Subtle dark borders |
| Header/Footer BG | `#252525` | Slightly lighter than main |

## CSS Architecture

### Layout Strategy

Using CSS Grid for the main page layout:

```css
.page-layout {
    display: grid;
    grid-template-areas:
        "header header header"
        "nav    main   aside"
        "footer footer footer";
    grid-template-columns: 200px 1fr 250px;
    grid-template-rows: auto 1fr auto;
    min-height: 100vh;
}
```

### CSS Custom Properties

Each theme defines these custom properties:

```css
:root {
    --color-bg: ...;
    --color-text: ...;
    --color-primary: ...;
    --color-secondary: ...;
    --color-border: ...;
    --color-header-bg: ...;
    --color-footer-bg: ...;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 2rem;
    --font-family: system-ui, sans-serif;
    --font-size-base: 16px;
}
```

## Component Design

### Header Component
- Fixed height
- Flexbox layout with space-between
- Contains title and theme selector

### Nav Component
- Fixed width sidebar
- Vertical list of navigation items
- Hover states for links

### Main Content Component
- Flexible width (takes remaining space)
- Contains multiple sections and articles
- Scrollable if content overflows

### Aside Component
- Fixed width sidebar
- Contains related links and info boxes
- Lower visual priority than main content

### Footer Component
- Fixed height
- Copyright and attribution text
- Centered or justified layout

### Theme Selector Component
- Native `<select>` element
- Styled to match current theme
- Immediate visual feedback on change

## Typography

### Font Stack
```css
font-family: system-ui, -apple-system, BlinkMacSystemFont,
             'Segoe UI', Roboto, sans-serif;
```

### Scale
- h1: 2rem
- h2: 1.5rem
- h3: 1.25rem
- body: 1rem
- small: 0.875rem

## Interaction Design

### Theme Switching
1. User clicks dropdown
2. User selects new theme
3. Page instantly updates (no flash)
4. All colors transition smoothly (optional enhancement)

### Navigation
- Visual indication of current page (future)
- Hover states on all clickable elements
- Focus states for keyboard navigation
