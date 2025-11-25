---
name: ui-styling
description: Creates modern, stylistic CSS themes for web UIs. Use when asked to create new themes, improve CSS styling, add visual polish, implement design systems, or make UIs more modern/attractive. Specializes in CSS custom properties, responsive layouts, typography, color schemes, animations, and accessibility.
---

# UI Styling Skill

## Overview

This skill helps create modern, visually appealing CSS themes for the styles-poc project. It follows best practices for CSS architecture, theming, and accessibility.

## Project Context

This is a Yew/Rust/WASM project demonstrating HTML5 semantic layouts with dynamic theme switching. CSS files are located in `/styles/` and loaded dynamically via JavaScript.

### Current Theme Structure

Each theme CSS file should define:
- CSS custom properties in `:root` for colors, spacing, typography
- Base reset and typography styles
- Layout styles using CSS Grid (`.page-layout`)
- Component styles for: `.site-header`, `.site-nav`, `.site-main`, `.site-aside`, `.site-footer`
- Interactive states (hover, focus, active)

### CSS Custom Properties Convention

```css
:root {
    /* Colors */
    --color-bg: ...;
    --color-bg-secondary: ...;
    --color-text: ...;
    --color-text-muted: ...;
    --color-primary: ...;
    --color-primary-hover: ...;
    --color-border: ...;
    --color-header-bg: ...;
    --color-footer-bg: ...;
    --color-code-bg: ...;
    --color-tag-bg: ...;
    --color-tag-text: ...;

    /* Spacing */
    --spacing-xs: 0.25rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 1.5rem;
    --spacing-xl: 2rem;

    /* Typography */
    --font-family: ...;
    --font-family-mono: ...;
    --font-size-base: 16px;
    --font-size-sm: 0.875rem;
    --font-size-lg: 1.125rem;
    --line-height: 1.6;

    /* Layout */
    --nav-width: 200px;
    --aside-width: 250px;
    --border-radius: 4px;
}
```

## Instructions

When creating or improving CSS themes:

### 1. Analyze Requirements
- Understand the visual style requested (e.g., "modern", "minimal", "bold", "soft")
- Consider accessibility (contrast ratios, focus states)
- Plan the color palette with primary, secondary, and accent colors

### 2. Create Color Palette
- Use a coherent color system (complementary, analogous, or triadic)
- Ensure sufficient contrast for readability (WCAG AA minimum)
- Define both light and dark variants if appropriate
- Include semantic colors (success, warning, error, info)

### 3. Define Typography
- Choose appropriate font stacks (system fonts for performance, or web fonts)
- Establish a clear type scale (h1 through h6, body, small)
- Set appropriate line heights for readability (1.4-1.6 for body text)

### 4. Implement Layout Styles
- Use CSS Grid for page layout
- Ensure responsive behavior
- Add appropriate spacing using the spacing scale

### 5. Add Visual Polish
- Subtle shadows for depth
- Smooth transitions for interactions (0.2s-0.3s)
- Border radius for softer edges
- Hover and focus states for all interactive elements

### 6. Consider Modern CSS Features
- CSS custom properties for theming
- CSS Grid and Flexbox for layouts
- `clamp()` for fluid typography
- `@media (prefers-color-scheme)` for system theme detection
- `@media (prefers-reduced-motion)` for accessibility

## Theme Ideas

### Modern/Clean
- Generous whitespace
- Subtle shadows
- Rounded corners
- Soft color palette
- Sans-serif typography

### Bold/Vibrant
- High contrast colors
- Strong accent colors
- Larger typography
- Minimal shadows
- Clear visual hierarchy

### Minimal/Elegant
- Limited color palette (2-3 colors)
- Lots of whitespace
- Thin borders or no borders
- Refined typography
- Subtle hover states

### Retro/Vintage
- Warm color tones
- Textured backgrounds
- Serif or display fonts
- Decorative borders
- Nostalgic color combinations

### Corporate/Professional
- Conservative color palette (blues, grays)
- Clear hierarchy
- Traditional layout
- Professional typography
- Subtle interactions

## File Naming Convention

Theme files should be named descriptively:
- `simple-light.css` - Basic light theme
- `simple-dark.css` - Basic dark theme
- `modern-minimal.css` - Modern minimalist style
- `vibrant-bold.css` - Bold, colorful style

## Adding New Themes

1. Create new CSS file in `/styles/` directory
2. Copy structure from existing theme
3. Update all CSS custom properties
4. Test with theme switcher
5. Add option to theme selector in `src/components/header.rs`

## Examples

### Soft Gradient Background
```css
body {
    background: linear-gradient(135deg, var(--color-bg) 0%, var(--color-bg-secondary) 100%);
}
```

### Subtle Card Shadow
```css
.content-article {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
    transition: box-shadow 0.2s ease;
}
.content-article:hover {
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
}
```

### Smooth Color Transitions
```css
a {
    color: var(--color-primary);
    transition: color 0.2s ease;
}
a:hover {
    color: var(--color-primary-hover);
}
```

### Focus Ring for Accessibility
```css
:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
}
```
