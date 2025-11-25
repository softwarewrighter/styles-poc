# Wild CSS Patterns Reference

## Gradient Background Patterns

### Animated Gradient
```css
@keyframes gradientShift {
    0% { background-position: 0% 50%; }
    50% { background-position: 100% 50%; }
    100% { background-position: 0% 50%; }
}

body {
    background: linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab);
    background-size: 400% 400%;
    animation: gradientShift 15s ease infinite;
}
```

### Mesh Gradient
```css
body {
    background-color: #ff99ee;
    background-image:
        radial-gradient(at 40% 20%, #ff99ee 0px, transparent 50%),
        radial-gradient(at 80% 0%, #ffd700 0px, transparent 50%),
        radial-gradient(at 0% 50%, #00ffff 0px, transparent 50%),
        radial-gradient(at 80% 50%, #ff6b6b 0px, transparent 50%),
        radial-gradient(at 0% 100%, #7b68ee 0px, transparent 50%),
        radial-gradient(at 80% 100%, #00fa9a 0px, transparent 50%);
}
```

### Diagonal Stripes
```css
body {
    background: repeating-linear-gradient(
        45deg,
        #606dbc,
        #606dbc 10px,
        #465298 10px,
        #465298 20px
    );
}
```

### Conic Gradient Burst
```css
body {
    background: conic-gradient(
        from 0deg at 50% 50%,
        #ff0080, #ff8c00, #40e0d0,
        #ff0080, #ff8c00, #40e0d0,
        #ff0080
    );
}
```

## Text Rotation Effects

### Tilted Heading
```css
h1 {
    transform: rotate(-3deg);
    transform-origin: left center;
}
```

### Dramatic Tilt with Shadow
```css
.wild-heading {
    transform: rotate(-5deg) skewX(-5deg);
    text-shadow:
        3px 3px 0 #ff00ff,
        6px 6px 0 #00ffff;
}
```

### Perspective Text
```css
.perspective-text {
    transform: perspective(500px) rotateX(15deg);
    transform-origin: bottom center;
}
```

### Hover Rotation
```css
h2 {
    transition: transform 0.3s ease;
}
h2:hover {
    transform: rotate(2deg) scale(1.05);
}
```

## Fade-In Animations

### Simple Fade In
```css
@keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
}
.fade-in {
    animation: fadeIn 0.8s ease-out forwards;
}
```

### Fade In Up
```css
@keyframes fadeInUp {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
.fade-in-up {
    animation: fadeInUp 0.6s ease-out forwards;
}
```

### Fade In Scale
```css
@keyframes fadeInScale {
    from {
        opacity: 0;
        transform: scale(0.9);
    }
    to {
        opacity: 1;
        transform: scale(1);
    }
}
.fade-in-scale {
    animation: fadeInScale 0.5s ease-out forwards;
}
```

### Staggered Fade In (use with nth-child)
```css
.stagger-item {
    opacity: 0;
    animation: fadeInUp 0.5s ease-out forwards;
}
.stagger-item:nth-child(1) { animation-delay: 0.1s; }
.stagger-item:nth-child(2) { animation-delay: 0.2s; }
.stagger-item:nth-child(3) { animation-delay: 0.3s; }
.stagger-item:nth-child(4) { animation-delay: 0.4s; }
.stagger-item:nth-child(5) { animation-delay: 0.5s; }
```

## Slide-In Animations

### Slide In From Left
```css
@keyframes slideInLeft {
    from {
        opacity: 0;
        transform: translateX(-100px);
    }
    to {
        opacity: 1;
        transform: translateX(0);
    }
}
.slide-in-left {
    animation: slideInLeft 0.6s ease-out forwards;
}
```

### Slide In From Right
```css
@keyframes slideInRight {
    from {
        opacity: 0;
        transform: translateX(100px);
    }
    to {
        opacity: 1;
        transform: translateX(0);
    }
}
.slide-in-right {
    animation: slideInRight 0.6s ease-out forwards;
}
```

### Slide In From Bottom
```css
@keyframes slideInBottom {
    from {
        opacity: 0;
        transform: translateY(50px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
.slide-in-bottom {
    animation: slideInBottom 0.6s ease-out forwards;
}
```

### Bounce In
```css
@keyframes bounceIn {
    0% {
        opacity: 0;
        transform: scale(0.3);
    }
    50% {
        transform: scale(1.1);
    }
    70% {
        transform: scale(0.9);
    }
    100% {
        opacity: 1;
        transform: scale(1);
    }
}
.bounce-in {
    animation: bounceIn 0.8s cubic-bezier(0.68, -0.55, 0.265, 1.55) forwards;
}
```

## Continuous Animations

### Floating Effect
```css
@keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
}
.floating {
    animation: float 3s ease-in-out infinite;
}
```

### Pulse Glow
```css
@keyframes pulseGlow {
    0%, 100% {
        box-shadow: 0 0 5px #ff00ff, 0 0 10px #ff00ff;
    }
    50% {
        box-shadow: 0 0 20px #ff00ff, 0 0 40px #ff00ff;
    }
}
.pulse-glow {
    animation: pulseGlow 2s ease-in-out infinite;
}
```

### Rainbow Border
```css
@keyframes rainbowBorder {
    0% { border-color: #ff0000; }
    17% { border-color: #ff8000; }
    33% { border-color: #ffff00; }
    50% { border-color: #00ff00; }
    67% { border-color: #0080ff; }
    83% { border-color: #8000ff; }
    100% { border-color: #ff0000; }
}
.rainbow-border {
    border: 3px solid;
    animation: rainbowBorder 5s linear infinite;
}
```

### Spin
```css
@keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}
.spin {
    animation: spin 10s linear infinite;
}
```

## Wild Color Palettes

### Neon Cyberpunk
```css
--color-neon-pink: #ff00ff;
--color-neon-cyan: #00ffff;
--color-neon-yellow: #ffff00;
--color-dark-bg: #0a0a0a;
--color-dark-surface: #1a1a2e;
```

### Vaporwave
```css
--color-vapor-pink: #ff71ce;
--color-vapor-blue: #01cdfe;
--color-vapor-green: #05ffa1;
--color-vapor-purple: #b967ff;
--color-vapor-yellow: #fffb96;
```

### Sunset Fire
```css
--color-sunset-red: #ff4e50;
--color-sunset-orange: #fc913a;
--color-sunset-yellow: #f9d423;
--color-sunset-pink: #e94057;
--color-sunset-purple: #8a2387;
```

### Electric Ocean
```css
--color-ocean-deep: #0c1445;
--color-ocean-mid: #1a3a6e;
--color-ocean-light: #3d7ea6;
--color-electric-cyan: #00f5ff;
--color-electric-green: #00ff88;
```

## Wild Hover Effects

### Shake on Hover
```css
@keyframes shake {
    0%, 100% { transform: translateX(0); }
    25% { transform: translateX(-5px); }
    75% { transform: translateX(5px); }
}
.shake-hover:hover {
    animation: shake 0.3s ease-in-out;
}
```

### Glitch Effect
```css
@keyframes glitch {
    0% { transform: translate(0); }
    20% { transform: translate(-2px, 2px); }
    40% { transform: translate(-2px, -2px); }
    60% { transform: translate(2px, 2px); }
    80% { transform: translate(2px, -2px); }
    100% { transform: translate(0); }
}
.glitch-hover:hover {
    animation: glitch 0.3s linear;
    text-shadow:
        2px 0 #ff00ff,
        -2px 0 #00ffff;
}
```

### Neon Flicker
```css
@keyframes neonFlicker {
    0%, 19%, 21%, 23%, 25%, 54%, 56%, 100% {
        text-shadow:
            0 0 5px #fff,
            0 0 10px #fff,
            0 0 20px #ff00ff,
            0 0 40px #ff00ff;
    }
    20%, 24%, 55% {
        text-shadow: none;
    }
}
.neon-flicker:hover {
    animation: neonFlicker 1.5s infinite alternate;
}
```

## Decorative Elements

### Floating Shapes (Pseudo-elements)
```css
.wild-container::before {
    content: '';
    position: fixed;
    width: 300px;
    height: 300px;
    background: radial-gradient(circle, rgba(255,0,255,0.3) 0%, transparent 70%);
    top: -100px;
    right: -100px;
    border-radius: 50%;
    animation: float 8s ease-in-out infinite;
    pointer-events: none;
    z-index: -1;
}
```

### Animated Border
```css
.animated-border {
    position: relative;
    background: linear-gradient(90deg, #ff00ff, #00ffff, #ff00ff);
    background-size: 200% 100%;
    animation: borderMove 3s linear infinite;
    padding: 3px;
}
.animated-border > * {
    background: #000;
}
@keyframes borderMove {
    0% { background-position: 0% 50%; }
    100% { background-position: 200% 50%; }
}
```

## Accessibility

### Reduced Motion
```css
@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
        animation-delay: 0ms !important;
    }
}
```
