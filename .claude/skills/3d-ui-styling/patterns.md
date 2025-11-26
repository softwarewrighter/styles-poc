# 3D CSS Patterns Reference

## Perspective Container Setup

### Basic Perspective
```css
.perspective-container {
    perspective: 1000px;
    perspective-origin: 50% 50%;
}

.perspective-container > * {
    transform-style: preserve-3d;
}
```

### Off-Center Vanishing Point
```css
/* Vanishing point to the left */
.perspective-left {
    perspective: 1200px;
    perspective-origin: 25% 50%;
}

/* Vanishing point to the right */
.perspective-right {
    perspective: 1200px;
    perspective-origin: 75% 50%;
}

/* Vanishing point above */
.perspective-above {
    perspective: 1200px;
    perspective-origin: 50% 25%;
}
```

### Two-Point Perspective (Simulated)
```css
.two-point-container {
    perspective: 1500px;
    perspective-origin: 50% 50%;
}

/* Left-facing panels recede to left */
.panel-left {
    transform: rotateY(30deg);
    transform-origin: left center;
}

/* Right-facing panels recede to right */
.panel-right {
    transform: rotateY(-30deg);
    transform-origin: right center;
}
```

## Depth Layers with translateZ

### Basic Depth Positioning
```css
.layer-far {
    transform: translateZ(-200px);
    opacity: 0.6;
}

.layer-mid {
    transform: translateZ(0);
    opacity: 0.9;
}

.layer-near {
    transform: translateZ(100px);
    opacity: 1;
}

.layer-foreground {
    transform: translateZ(200px);
    opacity: 1;
}
```

### Depth with Scale Compensation
```css
/* Objects appear smaller as they recede, compensate to maintain size */
.depth-card-far {
    transform: translateZ(-300px) scale(1.3);
    opacity: 0.5;
    filter: blur(1px);
}

.depth-card-near {
    transform: translateZ(50px) scale(0.95);
    opacity: 1;
}
```

### Atmospheric Perspective (Fog Effect)
```css
.depth-layer {
    --depth: 0;
    transform: translateZ(calc(var(--depth) * -100px));
    opacity: calc(1 - (var(--depth) * 0.15));
    filter: blur(calc(var(--depth) * 0.5px));
}

/* Usage */
.far { --depth: 3; }
.mid { --depth: 1; }
.near { --depth: 0; }
```

## Font Size for Depth Perception

### Size-Based Depth Scale
```css
:root {
    --text-depth-0: 1.5rem;    /* Foreground */
    --text-depth-1: 1.25rem;   /* Near */
    --text-depth-2: 1rem;      /* Mid */
    --text-depth-3: 0.875rem;  /* Far */
    --text-depth-4: 0.75rem;   /* Very far */
}

.heading-foreground {
    font-size: var(--text-depth-0);
    font-weight: 700;
    color: var(--color-text);
}

.heading-background {
    font-size: var(--text-depth-3);
    font-weight: 400;
    color: var(--color-text-muted);
}
```

### Dynamic Font Scaling with Depth
```css
.depth-text {
    --z-depth: 0;
    font-size: calc(1rem + (0.5rem * (1 - var(--z-depth) / 10)));
    opacity: calc(1 - (var(--z-depth) * 0.08));
    transform: translateZ(calc(var(--z-depth) * -50px));
}
```

## 3D Carousel Effects

### Basic Carousel (8 Items)
```css
.carousel {
    perspective: 1000px;
    width: 300px;
    height: 200px;
    position: relative;
}

.carousel-container {
    width: 100%;
    height: 100%;
    position: relative;
    transform-style: preserve-3d;
    animation: carousel-rotate 20s linear infinite;
}

.carousel-item {
    position: absolute;
    width: 200px;
    height: 150px;
    left: 50px;
    top: 25px;
    backface-visibility: hidden;
}

/* Position items in a circle */
.carousel-item:nth-child(1) { transform: rotateY(0deg) translateZ(250px); }
.carousel-item:nth-child(2) { transform: rotateY(45deg) translateZ(250px); }
.carousel-item:nth-child(3) { transform: rotateY(90deg) translateZ(250px); }
.carousel-item:nth-child(4) { transform: rotateY(135deg) translateZ(250px); }
.carousel-item:nth-child(5) { transform: rotateY(180deg) translateZ(250px); }
.carousel-item:nth-child(6) { transform: rotateY(225deg) translateZ(250px); }
.carousel-item:nth-child(7) { transform: rotateY(270deg) translateZ(250px); }
.carousel-item:nth-child(8) { transform: rotateY(315deg) translateZ(250px); }

@keyframes carousel-rotate {
    from { transform: rotateY(0deg); }
    to { transform: rotateY(360deg); }
}
```

### Carousel with Size/Opacity Changes
```css
.carousel-item {
    transition: transform 0.5s, opacity 0.5s, font-size 0.5s;
}

/* Items in back are smaller and dimmer */
.carousel-container:not(:hover) .carousel-item {
    opacity: 0.5;
}

/* Pause on hover and highlight front item */
.carousel-container:hover {
    animation-play-state: paused;
}

.carousel-container:hover .carousel-item:hover {
    transform: rotateY(var(--rotation)) translateZ(300px) scale(1.2);
    opacity: 1;
    font-size: 1.25em;
}
```

### Vertical Carousel
```css
.carousel-vertical {
    perspective: 800px;
}

.carousel-vertical .carousel-container {
    animation: carousel-rotate-x 15s linear infinite;
}

.carousel-vertical .carousel-item:nth-child(1) { transform: rotateX(0deg) translateZ(200px); }
.carousel-vertical .carousel-item:nth-child(2) { transform: rotateX(60deg) translateZ(200px); }
.carousel-vertical .carousel-item:nth-child(3) { transform: rotateX(120deg) translateZ(200px); }
.carousel-vertical .carousel-item:nth-child(4) { transform: rotateX(180deg) translateZ(200px); }
.carousel-vertical .carousel-item:nth-child(5) { transform: rotateX(240deg) translateZ(200px); }
.carousel-vertical .carousel-item:nth-child(6) { transform: rotateX(300deg) translateZ(200px); }

@keyframes carousel-rotate-x {
    from { transform: rotateX(0deg); }
    to { transform: rotateX(360deg); }
}
```

## Mouse-Driven 3D Effects

### Tilt on Mouse Move (CSS + JS)
```css
.tilt-card {
    transform-style: preserve-3d;
    transition: transform 0.1s ease-out;
}

/* Default state */
.tilt-card {
    transform: perspective(1000px) rotateX(0deg) rotateY(0deg);
}
```

```javascript
// Add to theme or inline
document.querySelectorAll('.tilt-card').forEach(card => {
    card.addEventListener('mousemove', (e) => {
        const rect = card.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        const centerX = rect.width / 2;
        const centerY = rect.height / 2;
        const rotateX = (y - centerY) / 10;
        const rotateY = (centerX - x) / 10;
        card.style.transform = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
    });
    card.addEventListener('mouseleave', () => {
        card.style.transform = 'perspective(1000px) rotateX(0deg) rotateY(0deg)';
    });
});
```

### Parallax Depth Layers (CSS + JS)
```css
.parallax-container {
    perspective: 1px;
    height: 100vh;
    overflow-x: hidden;
    overflow-y: auto;
}

.parallax-layer {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
}

.parallax-back {
    transform: translateZ(-2px) scale(3);
}

.parallax-mid {
    transform: translateZ(-1px) scale(2);
}

.parallax-front {
    transform: translateZ(0);
}
```

### Mouse Parallax (No Scroll)
```css
.mouse-parallax-layer {
    transition: transform 0.1s ease-out;
}
```

```javascript
document.addEventListener('mousemove', (e) => {
    const x = (e.clientX / window.innerWidth - 0.5) * 2;
    const y = (e.clientY / window.innerHeight - 0.5) * 2;

    document.querySelectorAll('.mouse-parallax-layer').forEach(layer => {
        const depth = layer.dataset.depth || 1;
        const moveX = x * depth * 20;
        const moveY = y * depth * 20;
        layer.style.transform = `translate(${moveX}px, ${moveY}px)`;
    });
});
```

## SVG 3D Effects

### Perspective Grid (Converging Lines)
```svg
<svg viewBox="0 0 400 300" class="perspective-grid">
    <defs>
        <linearGradient id="gridFade" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stop-color="currentColor" stop-opacity="0.1"/>
            <stop offset="100%" stop-color="currentColor" stop-opacity="0.5"/>
        </linearGradient>
    </defs>

    <!-- Horizontal lines -->
    <line x1="0" y1="250" x2="400" y2="250" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="20" y1="220" x2="380" y2="220" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="50" y1="190" x2="350" y2="190" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="80" y1="170" x2="320" y2="170" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="120" y1="155" x2="280" y2="155" stroke="url(#gridFade)" stroke-width="1"/>

    <!-- Converging vertical lines to vanishing point at (200, 100) -->
    <line x1="0" y1="300" x2="200" y2="100" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="100" y1="300" x2="200" y2="100" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="200" y1="300" x2="200" y2="100" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="300" y1="300" x2="200" y2="100" stroke="url(#gridFade)" stroke-width="1"/>
    <line x1="400" y1="300" x2="200" y2="100" stroke="url(#gridFade)" stroke-width="1"/>
</svg>
```

### Isometric Cube
```svg
<svg viewBox="0 0 100 100" class="iso-cube">
    <!-- Top face -->
    <polygon points="50,10 90,30 50,50 10,30" fill="var(--iso-top, #6366f1)"/>
    <!-- Left face -->
    <polygon points="10,30 50,50 50,90 10,70" fill="var(--iso-left, #4f46e5)"/>
    <!-- Right face -->
    <polygon points="50,50 90,30 90,70 50,90" fill="var(--iso-right, #3730a3)"/>
</svg>
```

### 3D Extruded Text Effect
```css
.extruded-text {
    color: var(--color-primary);
    text-shadow:
        1px 1px 0 var(--shadow-1, #4338ca),
        2px 2px 0 var(--shadow-2, #3730a3),
        3px 3px 0 var(--shadow-3, #312e81),
        4px 4px 0 var(--shadow-4, #1e1b4b),
        5px 5px 10px rgba(0, 0, 0, 0.4);
}
```

### Animated Isometric Grid
```css
.iso-grid {
    background-image:
        linear-gradient(30deg, var(--grid-color) 12%, transparent 12.5%, transparent 87%, var(--grid-color) 87.5%),
        linear-gradient(150deg, var(--grid-color) 12%, transparent 12.5%, transparent 87%, var(--grid-color) 87.5%),
        linear-gradient(30deg, var(--grid-color) 12%, transparent 12.5%, transparent 87%, var(--grid-color) 87.5%),
        linear-gradient(150deg, var(--grid-color) 12%, transparent 12.5%, transparent 87%, var(--grid-color) 87.5%);
    background-size: 40px 70px;
    background-position: 0 0, 0 0, 20px 35px, 20px 35px;
    animation: iso-scroll 10s linear infinite;
}

@keyframes iso-scroll {
    from { background-position: 0 0, 0 0, 20px 35px, 20px 35px; }
    to { background-position: 40px 70px, 40px 70px, 60px 105px, 60px 105px; }
}
```

## Card Flip Effects

### 3D Card Flip on Hover
```css
.flip-card {
    perspective: 1000px;
    width: 200px;
    height: 300px;
}

.flip-card-inner {
    width: 100%;
    height: 100%;
    transition: transform 0.6s;
    transform-style: preserve-3d;
}

.flip-card:hover .flip-card-inner {
    transform: rotateY(180deg);
}

.flip-card-front,
.flip-card-back {
    position: absolute;
    width: 100%;
    height: 100%;
    backface-visibility: hidden;
}

.flip-card-back {
    transform: rotateY(180deg);
}
```

### Flip with Spring Effect
```css
.flip-card-inner {
    transition: transform 0.8s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
```

## Depth-Based Shadows

### Layered 3D Shadow
```css
.depth-shadow {
    box-shadow:
        0 1px 1px rgba(0,0,0,0.08),
        0 2px 2px rgba(0,0,0,0.08),
        0 4px 4px rgba(0,0,0,0.08),
        0 8px 8px rgba(0,0,0,0.08),
        0 16px 16px rgba(0,0,0,0.08);
}
```

### Shadow Based on Z-Depth
```css
.card {
    --z-offset: 20px;
    transform: translateZ(var(--z-offset));
    box-shadow:
        0 calc(var(--z-offset) * 0.5) calc(var(--z-offset) * 1.5) rgba(0,0,0,0.2);
}

.card:hover {
    --z-offset: 40px;
}
```

## Vanishing Point Text

### Text Receding to Center
```css
.vanishing-text {
    perspective: 500px;
    perspective-origin: 50% 100%;
}

.vanishing-text > * {
    transform-origin: center bottom;
}

.vanishing-text > *:nth-child(1) { transform: translateZ(0) scale(1); opacity: 1; }
.vanishing-text > *:nth-child(2) { transform: translateZ(-50px) scale(0.9); opacity: 0.9; }
.vanishing-text > *:nth-child(3) { transform: translateZ(-100px) scale(0.8); opacity: 0.8; }
.vanishing-text > *:nth-child(4) { transform: translateZ(-150px) scale(0.7); opacity: 0.7; }
.vanishing-text > *:nth-child(5) { transform: translateZ(-200px) scale(0.6); opacity: 0.6; }
```

### Animated Depth Cycling
```css
@keyframes depth-cycle {
    0% { transform: translateZ(200px); opacity: 1; font-size: 1.5rem; }
    100% { transform: translateZ(-200px); opacity: 0.3; font-size: 0.75rem; }
}

.depth-cycle-item {
    animation: depth-cycle 10s linear infinite;
}

.depth-cycle-item:nth-child(2) { animation-delay: -2s; }
.depth-cycle-item:nth-child(3) { animation-delay: -4s; }
.depth-cycle-item:nth-child(4) { animation-delay: -6s; }
.depth-cycle-item:nth-child(5) { animation-delay: -8s; }
```

## 3D Color Palettes

### Cool Depth (Blues)
```css
--depth-bg-far: #0f172a;
--depth-bg-mid: #1e293b;
--depth-bg-near: #334155;
--depth-accent: #3b82f6;
--depth-highlight: #60a5fa;
```

### Warm Depth (Oranges)
```css
--depth-bg-far: #1c1917;
--depth-bg-mid: #292524;
--depth-bg-near: #44403c;
--depth-accent: #f97316;
--depth-highlight: #fb923c;
```

### Ethereal (Purples)
```css
--depth-bg-far: #0f0720;
--depth-bg-mid: #1a0f30;
--depth-bg-near: #2d1f50;
--depth-accent: #a855f7;
--depth-highlight: #c084fc;
```

### Matrix (Greens)
```css
--depth-bg-far: #000000;
--depth-bg-mid: #0a1f0a;
--depth-bg-near: #143014;
--depth-accent: #22c55e;
--depth-highlight: #4ade80;
```

## Accessibility

### Reduced Motion
```css
@media (prefers-reduced-motion: reduce) {
    .carousel-container,
    .depth-cycle-item,
    .iso-grid {
        animation: none;
    }

    .flip-card-inner,
    .tilt-card,
    .depth-text {
        transition: none;
    }

    /* Remove 3D transforms that may cause disorientation */
    .perspective-container {
        perspective: none;
    }

    .layer-far,
    .layer-mid,
    .layer-near,
    .layer-foreground {
        transform: none;
    }
}
```

### High Contrast Mode
```css
@media (prefers-contrast: high) {
    .depth-text {
        opacity: 1 !important;
        filter: none !important;
    }

    .layer-far,
    .layer-mid {
        opacity: 1;
    }
}
```
