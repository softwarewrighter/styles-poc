# Retro CSS Patterns Reference

## Terminal Color Palettes

### Classic Green Phosphor (P1)
```css
:root {
    --color-bg: #0a0a0a;
    --color-bg-secondary: #0f0f0f;
    --color-text: #33ff33;
    --color-text-muted: #22aa22;
    --color-primary: #00ff00;
    --color-primary-hover: #66ff66;
    --color-border: #1a3a1a;
    --color-glow: rgba(51, 255, 51, 0.5);
}
```

### Amber Phosphor (P3)
```css
:root {
    --color-bg: #0a0800;
    --color-bg-secondary: #141000;
    --color-text: #ffb000;
    --color-text-muted: #cc8800;
    --color-primary: #ffcc00;
    --color-primary-hover: #ffdd44;
    --color-border: #3a2a00;
    --color-glow: rgba(255, 176, 0, 0.5);
}
```

### White Phosphor (P4)
```css
:root {
    --color-bg: #0a0a0a;
    --color-bg-secondary: #141414;
    --color-text: #e0e0e0;
    --color-text-muted: #a0a0a0;
    --color-primary: #ffffff;
    --color-primary-hover: #ffffff;
    --color-border: #2a2a2a;
    --color-glow: rgba(255, 255, 255, 0.4);
}
```

## DOS Color Palettes

### Classic DOS Blue
```css
:root {
    --color-bg: #0000aa;
    --color-bg-secondary: #000080;
    --color-text: #ffffff;
    --color-text-muted: #aaaaaa;
    --color-primary: #55ffff;
    --color-primary-hover: #ffffff;
    --color-border: #5555ff;
    --color-highlight-bg: #00aaaa;
}
```

### DOS BIOS
```css
:root {
    --color-bg: #000000;
    --color-bg-secondary: #0000aa;
    --color-text: #aaaaaa;
    --color-text-muted: #555555;
    --color-primary: #ffff55;
    --color-primary-hover: #ffffff;
    --color-border: #555555;
}
```

## Windows 95 Color Palette

```css
:root {
    /* Main colors */
    --win95-gray: #c0c0c0;
    --win95-dark-gray: #808080;
    --win95-white: #ffffff;
    --win95-black: #000000;
    --win95-navy: #000080;
    --win95-teal: #008080;

    /* 3D effect colors */
    --win95-highlight: #ffffff;
    --win95-shadow: #808080;
    --win95-dark-shadow: #404040;

    /* Mapped to theme variables */
    --color-bg: #c0c0c0;
    --color-bg-secondary: #d4d0c8;
    --color-text: #000000;
    --color-text-muted: #404040;
    --color-primary: #000080;
    --color-primary-hover: #0000ff;
    --color-border: #808080;
}
```

## Typography Stacks

### Terminal Monospace
```css
--font-family: 'IBM Plex Mono', 'Fira Code', 'Source Code Pro',
               'Courier New', Courier, monospace;
--font-family-mono: var(--font-family);
```

### DOS/BIOS Style
```css
--font-family: 'Perfect DOS VGA 437', 'Px437 IBM VGA8',
               'Fixedsys', 'Terminal', monospace;
```

### Windows 95
```css
--font-family: 'Tahoma', 'MS Sans Serif', 'Segoe UI',
               Arial, sans-serif;
```

### Early Mac
```css
--font-family: 'Chicago', 'Geneva', 'Monaco',
               'Lucida Grande', sans-serif;
```

## CRT Effects

### Scanline Overlay
```css
/* Apply to body or main container */
.crt-scanlines::after {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: repeating-linear-gradient(
        0deg,
        rgba(0, 0, 0, 0.15),
        rgba(0, 0, 0, 0.15) 1px,
        transparent 1px,
        transparent 2px
    );
    pointer-events: none;
    z-index: 9999;
}
```

### Subtle Scanlines (Less Intense)
```css
.crt-scanlines-subtle::after {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: repeating-linear-gradient(
        0deg,
        rgba(0, 0, 0, 0.05),
        rgba(0, 0, 0, 0.05) 1px,
        transparent 1px,
        transparent 3px
    );
    pointer-events: none;
    z-index: 9999;
}
```

### Phosphor Glow Effect
```css
.phosphor-glow {
    text-shadow:
        0 0 2px var(--color-glow),
        0 0 5px var(--color-glow),
        0 0 10px var(--color-glow);
}
```

### Text Bloom/Blur
```css
.text-bloom {
    text-shadow:
        0 0 1px var(--color-text),
        0 0 2px var(--color-glow);
}
```

### CRT Flicker Animation
```css
@keyframes crt-flicker {
    0% { opacity: 1; }
    5% { opacity: 0.98; }
    10% { opacity: 1; }
    15% { opacity: 0.97; }
    20% { opacity: 1; }
    100% { opacity: 1; }
}

.crt-flicker {
    animation: crt-flicker 0.15s infinite;
}
```

### Screen Curvature Effect
```css
.crt-curve {
    border-radius: 20px / 10px;
    box-shadow:
        inset 0 0 50px rgba(0, 0, 0, 0.5),
        inset 0 0 100px rgba(0, 0, 0, 0.3);
}
```

## Windows 95 3D Effects

### Outset Button (Normal)
```css
.win95-button {
    background: var(--win95-gray);
    border: 2px solid;
    border-color: var(--win95-white) var(--win95-dark-shadow)
                  var(--win95-dark-shadow) var(--win95-white);
    padding: 4px 12px;
}
```

### Inset Button (Pressed)
```css
.win95-button:active,
.win95-button-pressed {
    border-color: var(--win95-dark-shadow) var(--win95-white)
                  var(--win95-white) var(--win95-dark-shadow);
}
```

### Inset Panel
```css
.win95-inset {
    background: var(--win95-white);
    border: 2px solid;
    border-color: var(--win95-shadow) var(--win95-white)
                  var(--win95-white) var(--win95-shadow);
}
```

### Title Bar
```css
.win95-titlebar {
    background: linear-gradient(90deg,
        var(--win95-navy) 0%,
        var(--win95-teal) 100%);
    color: white;
    font-weight: bold;
    padding: 2px 4px;
}
```

## Blinking Cursor

### Block Cursor
```css
@keyframes blink-cursor {
    0%, 50% { opacity: 1; }
    51%, 100% { opacity: 0; }
}

.cursor-block::after {
    content: '\2588'; /* Full block character */
    animation: blink-cursor 1s step-end infinite;
}
```

### Underscore Cursor
```css
.cursor-underscore::after {
    content: '_';
    animation: blink-cursor 1s step-end infinite;
}
```

## Box Drawing Characters

For authentic DOS/terminal UI borders:

```css
/* These can be used in content or as background patterns */
.box-top-left::before { content: '\250C'; }    /* ┌ */
.box-top-right::before { content: '\2510'; }   /* ┐ */
.box-bottom-left::before { content: '\2514'; } /* └ */
.box-bottom-right::before { content: '\2518'; } /* ┘ */
.box-horizontal::before { content: '\2500'; }  /* ─ */
.box-vertical::before { content: '\2502'; }    /* │ */
```

## Retro Borders

### Pixel-Perfect Border
```css
.pixel-border {
    border: 2px solid var(--color-border);
    border-radius: 0;
    image-rendering: pixelated;
}
```

### Double Line Border
```css
.double-border {
    border: 3px double var(--color-border);
    border-radius: 0;
}
```

## Retro Animations

### Terminal Boot Sequence
```css
@keyframes terminal-boot {
    0% { opacity: 0; }
    10% { opacity: 1; }
    11% { opacity: 0; }
    20% { opacity: 1; }
    100% { opacity: 1; }
}
```

### Text Type-In Effect
```css
@keyframes type-in {
    from { width: 0; }
    to { width: 100%; }
}

.type-in {
    overflow: hidden;
    white-space: nowrap;
    animation: type-in 2s steps(30) forwards;
}
```

## Accessibility Considerations

### Reduce CRT Effects for Motion Sensitivity
```css
@media (prefers-reduced-motion: reduce) {
    .crt-flicker,
    .cursor-block::after,
    .cursor-underscore::after {
        animation: none;
    }

    .crt-scanlines::after {
        background: rgba(0, 0, 0, 0.05);
    }
}
```

### High Contrast Override
```css
@media (prefers-contrast: high) {
    :root {
        --color-text: #00ff00;
        --color-bg: #000000;
    }

    .phosphor-glow {
        text-shadow: none;
    }
}
```
