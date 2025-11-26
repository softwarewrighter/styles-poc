---
name: retro-ui-styling
description: Creates vintage/nostalgic CSS themes inspired by different computing eras. Use when asked to create retro, vintage, terminal, DOS, Windows 95, or classic computing-inspired themes. Specializes in CRT effects, phosphor glow, scanlines, monospace typography, and era-specific color palettes.
---

# Retro UI Styling Skill

## Overview

This skill helps create nostalgic CSS themes that evoke the look and feel of classic computing eras. It focuses on authentic retro aesthetics while maintaining usability in modern browsers.

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

When creating retro CSS themes:

### 1. Choose an Era

Identify the specific computing era to emulate:

- **Terminal Era (1970s-1980s)**: Green/amber phosphor monitors, CRT scanlines, monospace text
- **DOS Era (1980s)**: Blue backgrounds, white/cyan text, text-mode interfaces
- **Early GUI Era (1984-1994)**: Mac System 1-7, early Windows, bitmap fonts
- **Windows 95 Era (1995-2000)**: 3D beveled buttons, system gray, pixel-perfect borders
- **Early Web Era (1995-2000)**: Tiled backgrounds, rainbow dividers, "Under Construction" vibes

### 2. Select Era-Appropriate Colors

Each era has distinctive color characteristics:

- **Terminal**: Black background with green (#33ff33) or amber (#ffb000) text
- **DOS**: Blue (#0000aa) background with white/cyan text
- **Early Mac**: Black and white with 1-bit precision
- **Windows 95**: System gray (#c0c0c0), navy title bars (#000080)
- **Early Web**: Bright colors, often clashing, gray backgrounds (#808080)

### 3. Choose Authentic Typography

- Use monospace fonts for terminal/DOS themes
- Consider web-safe fonts that existed in the era
- Bitmap-style fonts for extra authenticity
- Larger text sizes (older displays had lower resolution)

### 4. Apply Era-Specific Effects

- **CRT Effects**: Scanlines, phosphor glow, screen curvature (subtle)
- **3D Effects**: Beveled borders, inset/outset shadows
- **Pixel Borders**: Sharp 1-2px borders, no border-radius
- **Color Banding**: Limited color palettes

### 5. Consider User Experience

While being authentic, ensure:
- Text remains readable
- Interactive elements are clearly identifiable
- Accessibility isn't completely sacrificed
- The theme is fun but usable

## Retro Theme Ideas

### Terminal Dark
- Pure black background
- Bright green or amber text (phosphor color)
- CRT scanline overlay
- Subtle text glow/bloom
- 100% monospace typography
- Blinking cursor effects

### DOS Blue
- Classic DOS blue (#0000aa) background
- White and cyan text
- Box-drawing character aesthetics
- Text-mode UI simulation
- F-key style navigation hints

### Windows 95
- System gray (#c0c0c0) backgrounds
- 3D beveled buttons (outset normal, inset pressed)
- Navy blue title bars
- Tahoma or MS Sans Serif fonts
- 2px 3D borders everywhere

### Mac Classic
- High contrast black and white
- Chicago or Geneva fonts
- 1px black borders
- Simple iconography
- Menu bar styling

### GeoCities
- Tiled backgrounds
- Bright, saturated colors
- Comic Sans or Times New Roman
- Rainbow horizontal rules
- Excessive text effects

## File Naming Convention

Retro theme files should be named descriptively:
- `terminal-dark.css` - Green terminal theme
- `terminal-amber.css` - Amber terminal theme
- `dos-blue.css` - DOS-style theme
- `win95-light.css` - Windows 95 theme
- `mac-classic.css` - Early Mac theme

## Adding Retro Themes

1. Create new CSS file in `/styles/` directory
2. Copy structure from existing theme
3. Apply era-specific colors and effects
4. Add scanlines or other effects using pseudo-elements
5. Test with theme switcher
6. Add option to theme selector in `src/components/header.rs`

## Performance Considerations

- CRT effects use pseudo-elements and are performant
- Avoid excessive box-shadows (use sparingly)
- Scanline overlays should use `pointer-events: none`
- Animations should be subtle and respect `prefers-reduced-motion`
