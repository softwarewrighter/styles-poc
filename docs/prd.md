# Product Requirements Document: Styles POC

## Overview

A proof-of-concept Yew/Rust/WASM application demonstrating HTML5 semantic layout elements with dynamic CSS theme switching.

## Goals

1. Demonstrate proper use of HTML5 semantic layout elements
2. Show how CSS can dramatically change the appearance of the same HTML structure
3. Provide a foundation for exploring different styling approaches
4. Keep implementation pure Rust with minimal JavaScript

## Features

### Core Features

1. **HTML5 Semantic Layout**
   - `<header>` - Page header with title and theme selector
   - `<nav>` - Navigation sidebar
   - `<main>` - Primary content area
   - `<aside>` - Secondary sidebar for supplementary content
   - `<section>` - Logical groupings within main content
   - `<article>` - Self-contained content pieces
   - `<footer>` - Page footer

2. **Theme Switching**
   - Dropdown selector to choose between themes
   - Dynamic CSS file loading
   - Persist theme selection (future enhancement)

3. **Filler Content**
   - Lorem ipsum text in various lengths
   - Demonstrate text, headings, lists, and other common elements

### Initial Themes

1. **simple-light** - Clean, light background with dark text
2. **simple-dark** - Dark background with light text

## Technical Requirements

- Rust/Yew for all application logic
- WebAssembly compilation target
- Vanilla CSS (no preprocessors initially)
- Minimal to no JavaScript
- Trunk for build tooling

## Non-Goals

- Production-ready styling
- Mobile-first responsive design (initially)
- Accessibility compliance (initially)
- Server-side rendering

## Success Criteria

1. Application compiles and runs in browser
2. All HTML5 semantic elements are present and visible
3. Theme switching works without page reload
4. Both themes are visually distinct and functional
