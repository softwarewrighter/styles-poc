---
name: wild-ui-styling
description: Creates radical, eye-catching CSS themes with bold animations, rotated text, gradient backgrounds, and dramatic effects. Use when asked to create wild, creative, experimental, or attention-grabbing UI styles. Specializes in CSS transforms, complex animations, gradient patterns, and unconventional layouts.
---

# Wild UI Styling Skill

## Overview

This skill helps create bold, experimental CSS themes that push visual boundaries. It focuses on dramatic effects, animations, and unconventional styling that makes UIs stand out.

## Project Context

This is a Yew/Rust/WASM project demonstrating HTML5 semantic layouts with dynamic theme switching. CSS files are located in `/styles/` and loaded dynamically.

## Wild Styling Principles

### 1. Go Bold or Go Home
- Use strong color contrasts
- Large, dramatic gradients
- Visible animations and transitions
- Unconventional element positioning

### 2. Animation is Key
- Elements should feel alive
- Use keyframe animations liberally
- Stagger animations for visual interest
- Consider scroll-triggered effects

### 3. Break the Grid (Carefully)
- Rotated text for visual interest
- Overlapping elements
- Asymmetric layouts
- Floating/animated decorative elements

## Instructions

When creating wild CSS themes:

### 1. Plan the Visual Impact
- Choose a bold color scheme (neons, high contrast, or unexpected combinations)
- Plan primary animation effects
- Identify elements for rotation/transformation
- Design the gradient background pattern

### 2. Create Gradient Backgrounds
- Use multi-stop gradients
- Consider animated gradients
- Layer multiple gradients for depth
- Add noise or pattern overlays

### 3. Add Motion
- Fade-in effects for content
- Slide-in animations for sections
- Hover animations that delight
- Subtle continuous animations for atmosphere

### 4. Transform Text
- Rotate headings for impact
- Use text shadows creatively
- Animate text on hover
- Consider perspective transforms

### 5. Layer Effects
- Multiple box shadows
- Pseudo-elements for decoration
- Backdrop filters where supported
- Border animations

## Theme Naming

Wild themes should have descriptive names:
- `wild-light.css` - Light-based wild theme
- `wild-dark.css` - Dark-based wild theme
- `neon-chaos.css` - Neon color explosion
- `retro-wave.css` - 80s synthwave style

## Adding Wild Themes

1. Create new CSS file in `/styles/` directory
2. Start with gradient background
3. Add animation keyframes
4. Apply transforms to headings
5. Add entrance animations to content
6. Include hover effects
7. Test with theme switcher
8. Add option to theme selector in `src/components/header.rs`

## Performance Considerations

- Use `transform` and `opacity` for animations (GPU accelerated)
- Avoid animating `width`, `height`, `top`, `left`
- Use `will-change` sparingly for heavy animations
- Respect `prefers-reduced-motion` for accessibility
