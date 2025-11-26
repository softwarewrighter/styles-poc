---
name: 3d-ui-styling
description: Creates immersive CSS themes with 3D perspective effects, vanishing points, depth-based sizing, and interactive carousel-like transitions. Use when asked to create 3D, perspective, depth, carousel, or spatial UI effects. Specializes in CSS 3D transforms, perspective, SVG depth effects, and mouse-driven interactions without Three.js.
---

# 3D UI Styling Skill

## Overview

This skill helps create immersive CSS themes that simulate 3D space using pure CSS, SVG, and vanilla JavaScript. It achieves depth perception through perspective transforms, vanishing points, scale-based depth cues, and interactive carousel effects—all without WebGL or Three.js.

## Project Context

This is a Yew/Rust/WASM project demonstrating HTML5 semantic layouts with dynamic theme switching. CSS files are located in `/styles/` and loaded dynamically.

## 3D Styling Principles

### 1. Perspective is Everything
- Use CSS `perspective` on containers to enable 3D transforms
- Choose appropriate perspective values (500px-2000px depending on effect)
- Set `perspective-origin` to control vanishing point position
- Layer elements at different Z-depths using `translateZ()`

### 2. Scale Implies Depth
- Larger elements appear closer, smaller appear farther
- Use scale transforms combined with opacity for depth fade
- Typography sizing creates visual hierarchy through perceived distance
- Background elements should be smaller and more muted

### 3. Multiple Vanishing Points
- Single vanishing point: classic perspective, objects converge to center
- Two-point perspective: edges recede to left and right horizon points
- Elements can reference different vanishing points for complex compositions

### 4. Motion Enhances Depth
- Parallax scrolling with different speeds for depth layers
- Mouse movement can shift perspective subtly
- Carousel rotations reveal spatial relationships
- Transition timing should feel physical (ease-out for approaching, ease-in for receding)

## CSS Custom Properties for 3D Themes

```css
:root {
    /* Perspective */
    --perspective-distance: 1000px;
    --perspective-origin-x: 50%;
    --perspective-origin-y: 50%;

    /* Depth layers (Z-axis positions) */
    --depth-far: -200px;
    --depth-mid: 0px;
    --depth-near: 100px;
    --depth-foreground: 200px;

    /* Scale for depth (larger = closer) */
    --scale-far: 0.7;
    --scale-mid: 1;
    --scale-near: 1.2;
    --scale-foreground: 1.5;

    /* Opacity for atmospheric perspective */
    --opacity-far: 0.5;
    --opacity-mid: 0.8;
    --opacity-near: 1;

    /* Font sizes for depth */
    --font-size-far: 0.75rem;
    --font-size-mid: 1rem;
    --font-size-near: 1.25rem;
    --font-size-foreground: 1.5rem;

    /* Blur for depth of field */
    --blur-far: 2px;
    --blur-mid: 0px;
    --blur-near: 0px;

    /* 3D rotation speeds */
    --carousel-duration: 20s;
    --hover-transition: 0.5s;
}
```

## Instructions

When creating 3D CSS themes:

### 1. Establish the Perspective Container
- Apply `perspective` to a parent container (often `body` or `.page-layout`)
- Set `transform-style: preserve-3d` to enable nested 3D
- Choose `perspective-origin` for desired vanishing point

### 2. Layer Content at Different Depths
- Use `translateZ()` to position elements in 3D space
- Combine with scale adjustments for consistent apparent size
- Apply opacity/blur for atmospheric perspective effect

### 3. Create Depth Through Typography
- Headings (foreground): Larger, bolder, higher contrast
- Body text (mid-ground): Standard sizing
- Secondary content (background): Smaller, lighter, optionally blurred

### 4. Add Interactive 3D Elements
- Carousel: Rotate items around Y-axis in a circle
- Card flip: `rotateY(180deg)` on hover
- Tilt effects: Subtle `rotateX/Y` based on mouse position
- Parallax: Different `translateZ` creates scroll speed differences

### 5. Use SVG for 3D Objects
- Isometric shapes using path transforms
- Layered SVG elements at different depths
- Animated SVG with 3D-like rotations
- Grid patterns that converge to vanishing points

### 6. Implement Carousel Effects
- Distribute items in a circle using `rotateY` + `translateZ`
- Use `animation` for continuous rotation
- Allow click/hover to bring items to front
- Transition font size and opacity with position

## Theme Ideas

### Infinite Corridor (Single Vanishing Point)
- Content panels recede toward center
- Text size decreases toward vanishing point
- Converging lines in background
- Scroll brings content forward

### Floating Cards (Multi-Layer)
- Cards at different Z-depths
- Subtle shadow beneath each card
- Parallax on mouse move
- Cards scale up on hover/focus

### Rotating Showcase (Carousel)
- Sections arranged in 3D carousel
- Active section is front and center
- Inactive sections smaller and dimmer
- Smooth rotation transitions

### Isometric Dashboard
- SVG isometric grid background
- Content boxes with 3D extrusion effect
- Hover lifts elements from surface
- Consistent isometric angles (30°)

## File Naming Convention

3D theme files should be named descriptively:
- `3d-light.css` - Light 3D perspective theme
- `3d-dark.css` - Dark 3D perspective theme
- `carousel-light.css` - Carousel-focused theme
- `isometric-dark.css` - Isometric 3D style

## Adding 3D Themes

1. Create new CSS file in `/styles/` directory
2. Set up perspective container
3. Define depth layers with `translateZ`
4. Add carousel or interactive elements
5. Include mouse-driven JavaScript if needed (inline or in theme)
6. Test with theme switcher
7. Add option to theme selector in `src/components/header.rs`

## Performance Considerations

- Use `transform` and `opacity` (GPU accelerated)
- Avoid excessive `filter: blur()` on large elements
- Use `will-change: transform` sparingly for heavy 3D
- Test on lower-end devices
- Respect `prefers-reduced-motion` for accessibility

## SVG 3D Integration

SVG elements can enhance 3D themes:
- Isometric grid backgrounds
- Perspective grid lines converging to vanishing point
- 3D-extruded text and shapes
- Animated depth indicators

See patterns.md for SVG examples and techniques.
