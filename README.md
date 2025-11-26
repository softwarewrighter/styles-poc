# Styles POC

A Yew/Rust/WASM proof-of-concept demonstrating HTML5 semantic layouts with dynamic CSS theme switching.

**[Live Demo](https://softwarewrighter.github.io/styles-poc/)**

## Features

- HTML5 semantic layout elements (header, nav, main, section, article, aside, footer)
- Dynamic CSS theme switching via dropdown selector
- Nine distinct themes ranging from minimal to 3D perspective
- Pure Rust/WASM implementation with minimal JavaScript
- Claude Code skills for AI-assisted theme development

## Theme Gallery

### Simple Light
Clean, minimal light theme with system fonts and subtle styling.

![Simple Light Theme](./images/screenshot1.png?ts=1732579200000)

### Simple Dark
Clean, minimal dark theme with comfortable contrast.

![Simple Dark Theme](./images/screenshot2.png?ts=1764113910793)

### Improved Light
Modern light theme with indigo accents, soft shadows, and smooth animations.

![Improved Light Theme](./images/screenshot3.png?ts=1732579200002)

### Improved Dark
Modern dark theme with deep purple palette and glow effects.

![Improved Dark Theme](./images/screenshot4.png?ts=1732579200003)

### Wild Light
Radical light theme with animated gradients, rotated text, and bounce effects.

![Wild Light Theme](./images/screenshot5.png?ts=1732579200004)

### Wild Dark
Neon cyberpunk theme with glowing effects, scanlines, and glitch animations.

![Wild Dark Theme](./images/screenshot6.png?ts=1732579200005)

### Terminal Dark
Retro CRT terminal theme with green phosphor glow, scanlines, and monospace typography.

![Terminal Dark Theme](./images/screenshot7.png?ts=1764116679558)

### 3D Light
Immersive light theme with CSS perspective, depth layers, and spatial animations.

![3D Light Theme](./images/screenshot8.png?ts=1732588400000)

### 3D Dark
Deep ethereal dark theme with glowing depth effects and 3D transforms.

![3D Dark Theme](./images/screenshot9.png?ts=1732588400001)

## Getting Started

### Prerequisites

- Rust (latest stable)
- Trunk (`cargo install trunk`)

### Running Locally

```bash
# Development server with hot reload
trunk serve

# Production build
trunk build --release
```

Open http://localhost:8080 and use the theme dropdown to switch between themes.

## Project Structure

```
styles-poc/
+-- Cargo.toml              # Rust dependencies
+-- build.rs                # Build info injection
+-- index.html              # Trunk entry point
+-- favicon.ico             # Site favicon
+-- src/
|   +-- main.rs             # App entry point and theme logic
|   +-- lorem.rs            # Lorem ipsum content
|   +-- components/
|       +-- mod.rs          # Component exports
|       +-- header.rs       # Header with theme selector
|       +-- main_content.rs # Main content sections
|       +-- sidebars.rs     # Nav and Aside components
|       +-- footer.rs       # Footer with build info
+-- styles/
|   +-- simple-light.css    # Minimal light theme
|   +-- simple-dark.css     # Minimal dark theme
|   +-- improved-light.css  # Modern light theme
|   +-- improved-dark.css   # Modern dark theme
|   +-- wild-light.css      # Experimental light theme
|   +-- wild-dark.css       # Experimental dark theme
|   +-- terminal-dark.css   # Retro terminal theme
|   +-- 3d-light.css        # 3D perspective light theme
|   +-- 3d-dark.css         # 3D perspective dark theme
+-- images/
|   +-- screenshot{1-9}.png # Theme screenshots
+-- documentation/
|   +-- prd.md              # Product requirements
|   +-- architecture.md     # System architecture
|   +-- design.md           # UI/UX design
|   +-- plan.md             # Implementation plan
|   +-- status.md           # Project status
|   +-- process.md          # Development process
+-- docs/                   # GitHub Pages build output
+-- .claude/
    +-- skills/             # Claude Code skills
```

## Claude Code Skills

This project includes four Claude Code skills for AI-assisted CSS theme development. These skills provide context and patterns that help Claude Code create consistent, high-quality themes.

### ui-styling Skill

**Location:** `.claude/skills/ui-styling/`

**Purpose:** Creates modern, polished CSS themes with professional styling.

**Files:**

#### SKILL.md
The main skill definition file containing:
- Skill metadata (name, description)
- Project context for the Yew/WASM app
- CSS custom properties convention
- Step-by-step theming instructions
- Theme style ideas (modern, bold, minimal, retro, corporate)
- File naming conventions
- Code examples for common patterns

#### patterns.md
A comprehensive CSS patterns reference including:

**Color Palettes:**
- Soft Neutrals (indigo primary)
- Ocean Blues
- Forest Greens
- Warm Sunset
- Deep Purple (dark)
- Midnight (dark)

**Typography Stacks:**
- Modern Sans-Serif (Inter)
- Elegant Serif (Merriweather)
- Technical/Code (JetBrains Mono)
- Friendly Rounded (Nunito)

**Effects:**
- Shadow systems (subtle, colored, inset)
- Animation patterns (fade, slide, pulse)
- Border styles (subtle, gradient, accent)
- Interactive states (hover, focus, active)
- Glassmorphism and Neumorphism
- Gradient text effects

**Accessibility:**
- Reduced motion media queries
- High contrast support
- System theme detection

### wild-ui-styling Skill

**Location:** `.claude/skills/wild-ui-styling/`

**Purpose:** Creates radical, experimental CSS themes with bold animations and unconventional styling.

**Files:**

#### SKILL.md
The skill definition for experimental themes:
- Guidelines for bold, attention-grabbing design
- Animation-first approach
- Transform and rotation techniques
- Performance considerations for animations

#### patterns.md
A reference for wild CSS effects including:

**Gradient Backgrounds:**
- Animated shifting gradients
- Mesh gradients with multiple radial stops
- Diagonal stripe patterns
- Conic gradient bursts

**Text Rotations:**
- Tilted headings (-3deg to -5deg)
- Dramatic tilt with layered shadows
- Perspective transforms
- Hover rotation effects

**Animations:**
- Fade-in variations (up, scale, staggered)
- Slide-in from all directions
- Bounce effects with cubic-bezier
- Floating/hovering elements
- Pulse and glow effects
- Rainbow border cycling
- Spin animations

**Neon Effects:**
- Cyberpunk color palettes
- Vaporwave pastels
- Sunset fire gradients
- Electric ocean blues
- Neon glow shadows
- Glitch/flicker effects

**Wild Hover States:**
- Shake on hover
- Glitch distortion
- Neon flicker

### retro-ui-styling Skill

**Location:** `.claude/skills/retro-ui-styling/`

**Purpose:** Creates vintage/nostalgic CSS themes inspired by different computing eras.

**Files:**

#### SKILL.md
The skill definition for retro themes:
- Guidelines for era-specific design (Terminal, DOS, Windows 95, Early Mac)
- CRT effects and scanlines
- Authentic color palettes (phosphor green, amber, DOS blue)
- Monospace typography patterns

#### patterns.md
A reference for retro CSS effects including:

**Terminal Color Palettes:**
- Classic Green Phosphor (P1)
- Amber Phosphor (P3)
- White Phosphor (P4)

**DOS/Windows Palettes:**
- Classic DOS Blue
- DOS BIOS style
- Windows 95 system colors

**CRT Effects:**
- Scanline overlays
- Phosphor glow and text bloom
- CRT flicker animations
- Screen curvature effects

**3D Effects (Windows 95):**
- Beveled buttons (outset/inset)
- Title bar gradients
- System gray panels

**Retro Typography:**
- Terminal monospace fonts
- DOS/BIOS style fonts
- Windows 95 Tahoma
- Early Mac Chicago/Geneva

### 3d-ui-styling Skill

**Location:** `.claude/skills/3d-ui-styling/`

**Purpose:** Creates immersive CSS themes with 3D perspective effects, vanishing points, depth-based sizing, and interactive carousel-like transitions without WebGL or Three.js.

**Files:**

#### SKILL.md
The skill definition for 3D perspective themes:
- CSS perspective and transform-style guidelines
- Vanishing point positioning techniques
- Depth layer management with translateZ
- Font sizing for depth perception
- Carousel and interactive 3D effects
- SVG integration for 3D objects
- Performance considerations for 3D transforms

#### patterns.md
A reference for CSS 3D effects including:

**Perspective Setup:**
- Basic perspective containers
- Off-center vanishing points
- Two-point perspective simulation
- Transform-style preserve-3d

**Depth Layers:**
- translateZ positioning
- Scale compensation for distance
- Atmospheric perspective (opacity/blur)
- Depth-based shadows

**Font Depth Effects:**
- Size-based depth scaling
- Dynamic font sizing with CSS variables
- Opacity gradients for receding text

**3D Carousel:**
- Circular item distribution with rotateY
- Vertical carousel with rotateX
- Size/opacity changes based on position
- Pause-on-hover interactions

**Mouse-Driven Effects:**
- Tilt cards on mouse move
- Parallax depth layers
- Scroll-based parallax
- Mouse position tracking

**SVG 3D Integration:**
- Perspective grid (converging lines)
- Isometric cubes and shapes
- 3D extruded text shadows
- Animated isometric grids

**Card Effects:**
- 3D flip on hover
- Spring effect transitions
- Depth-based layered shadows

**3D Color Palettes:**
- Cool Depth (blues)
- Warm Depth (oranges)
- Ethereal (purples)
- Matrix (greens)

**Accessibility:**
- Reduced motion support
- High contrast mode adjustments

### Using the Skills

When working with Claude Code in this project:

1. **For modern themes:** Ask Claude to "create a new theme" or "improve the styling" - it will automatically use the ui-styling skill patterns.

2. **For experimental themes:** Ask for "wild", "radical", "cyberpunk", or "animated" themes - Claude will use the wild-ui-styling skill.

3. **For retro themes:** Ask for "terminal", "retro", "DOS", "Windows 95", or "vintage" themes - Claude will use the retro-ui-styling skill.

4. **For 3D themes:** Ask for "3D", "perspective", "depth", "carousel", or "spatial" themes - Claude will use the 3d-ui-styling skill.

5. **Adding new themes:**
   - Create CSS file in `styles/` directory
   - Add option to dropdown in `src/components/header.rs`
   - Follow the CSS custom properties convention from the skills

## Technology Stack

- **Yew 0.21** - Rust framework for WebAssembly
- **wasm-bindgen** - Rust/JavaScript interop
- **web-sys** - Web API bindings
- **Trunk** - WASM build tool
- **CSS Custom Properties** - Theming system

## License

Copyright (c) 2025 Michael A Wright

MIT License - see [LICENSE](https://github.com/softwarewrighter/styles-poc/blob/main/LICENSE) for details.

## Links

- [Repository](https://github.com/softwarewrighter/styles-poc)
- [License](https://github.com/softwarewrighter/styles-poc/blob/main/LICENSE)
