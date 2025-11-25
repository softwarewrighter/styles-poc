# CSS Patterns Reference

## Modern Color Palettes

### Soft Neutrals
```css
--color-bg: #fafafa;
--color-bg-secondary: #f5f5f5;
--color-text: #374151;
--color-text-muted: #6b7280;
--color-primary: #6366f1;
--color-primary-hover: #4f46e5;
--color-border: #e5e7eb;
```

### Ocean Blues
```css
--color-bg: #f0f9ff;
--color-bg-secondary: #e0f2fe;
--color-text: #0c4a6e;
--color-text-muted: #0369a1;
--color-primary: #0284c7;
--color-primary-hover: #0369a1;
--color-border: #bae6fd;
```

### Forest Greens
```css
--color-bg: #f0fdf4;
--color-bg-secondary: #dcfce7;
--color-text: #14532d;
--color-text-muted: #166534;
--color-primary: #16a34a;
--color-primary-hover: #15803d;
--color-border: #bbf7d0;
```

### Warm Sunset
```css
--color-bg: #fffbeb;
--color-bg-secondary: #fef3c7;
--color-text: #78350f;
--color-text-muted: #92400e;
--color-primary: #f59e0b;
--color-primary-hover: #d97706;
--color-border: #fde68a;
```

### Deep Purple (Dark)
```css
--color-bg: #1e1b4b;
--color-bg-secondary: #312e81;
--color-text: #e0e7ff;
--color-text-muted: #a5b4fc;
--color-primary: #818cf8;
--color-primary-hover: #a5b4fc;
--color-border: #4338ca;
```

### Midnight (Dark)
```css
--color-bg: #0f172a;
--color-bg-secondary: #1e293b;
--color-text: #e2e8f0;
--color-text-muted: #94a3b8;
--color-primary: #38bdf8;
--color-primary-hover: #7dd3fc;
--color-border: #334155;
```

## Typography Stacks

### Modern Sans-Serif
```css
--font-family: 'Inter', system-ui, -apple-system, BlinkMacSystemFont,
               'Segoe UI', Roboto, 'Helvetica Neue', sans-serif;
```

### Elegant Serif
```css
--font-family: 'Merriweather', Georgia, 'Times New Roman', serif;
```

### Technical/Code
```css
--font-family-mono: 'JetBrains Mono', 'Fira Code', 'SF Mono',
                    Consolas, 'Liberation Mono', monospace;
```

### Friendly Rounded
```css
--font-family: 'Nunito', 'Poppins', system-ui, sans-serif;
```

## Shadow Systems

### Subtle Elevation
```css
--shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
--shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1);
--shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1);
--shadow-xl: 0 20px 25px -5px rgb(0 0 0 / 0.1);
```

### Colored Shadows
```css
.primary-shadow {
    box-shadow: 0 4px 14px 0 rgba(99, 102, 241, 0.25);
}
```

### Inset Shadows
```css
.inset-card {
    box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);
}
```

## Animation Patterns

### Fade In
```css
@keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
}
.fade-in { animation: fadeIn 0.3s ease-out; }
```

### Slide Up
```css
@keyframes slideUp {
    from {
        opacity: 0;
        transform: translateY(10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
.slide-up { animation: slideUp 0.3s ease-out; }
```

### Pulse
```css
@keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
}
.pulse { animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
```

### Smooth Hover Lift
```css
.hover-lift {
    transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.hover-lift:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
```

## Border Styles

### Subtle Border
```css
border: 1px solid var(--color-border);
```

### Gradient Border
```css
.gradient-border {
    border: 2px solid transparent;
    background: linear-gradient(white, white) padding-box,
                linear-gradient(135deg, #6366f1, #8b5cf6) border-box;
}
```

### Accent Border
```css
.accent-left {
    border-left: 4px solid var(--color-primary);
}
```

## Interactive States

### Button Hover
```css
button {
    background: var(--color-primary);
    transition: background 0.2s ease, transform 0.1s ease;
}
button:hover {
    background: var(--color-primary-hover);
}
button:active {
    transform: scale(0.98);
}
```

### Link Underline Animation
```css
a {
    text-decoration: none;
    background-image: linear-gradient(currentColor, currentColor);
    background-position: 0% 100%;
    background-repeat: no-repeat;
    background-size: 0% 2px;
    transition: background-size 0.3s ease;
}
a:hover {
    background-size: 100% 2px;
}
```

### Focus Ring
```css
:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
    border-radius: var(--border-radius);
}
```

## Responsive Breakpoints

```css
/* Mobile first approach */
@media (min-width: 640px) { /* sm */ }
@media (min-width: 768px) { /* md */ }
@media (min-width: 1024px) { /* lg */ }
@media (min-width: 1280px) { /* xl */ }
```

## Accessibility Patterns

### Reduced Motion
```css
@media (prefers-reduced-motion: reduce) {
    *, *::before, *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
    }
}
```

### High Contrast
```css
@media (prefers-contrast: high) {
    :root {
        --color-border: currentColor;
    }
    * {
        border-color: currentColor !important;
    }
}
```

### System Theme Detection
```css
@media (prefers-color-scheme: dark) {
    :root {
        /* Dark mode overrides */
    }
}
```

## Glassmorphism Effect

```css
.glass {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
}
```

## Neumorphism Effect

```css
.neumorphic {
    background: #e0e0e0;
    box-shadow: 8px 8px 16px #bebebe,
                -8px -8px 16px #ffffff;
    border-radius: 12px;
}
```

## Gradient Text

```css
.gradient-text {
    background: linear-gradient(135deg, #6366f1, #8b5cf6);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}
```
