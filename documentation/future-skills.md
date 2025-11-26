# Future Skills and Themes

This document outlines proposed additional Claude Code skills and CSS themes for the Styles POC project.

## Current Skills

| Skill | Purpose |
|-------|---------|
| **ui-styling** | Creates modern, polished CSS themes with professional styling, accessibility focus, and clean design patterns |
| **wild-ui-styling** | Creates radical, experimental CSS themes with bold animations, rotated text, neon effects, and dramatic visual impact |

## Current Themes

| Theme | Style |
|-------|-------|
| simple-light/dark | Minimal, clean styling |
| improved-light/dark | Modern with shadows, animations |
| wild-light/dark | Experimental with gradients, glitch effects |

---

## Proposed Additional Skills

### 1. retro-ui-styling (Implemented)

Creates vintage/nostalgic themes inspired by different computing eras.

**Specialty Areas:**
- 80s terminal (green/amber phosphor, CRT effects, scanlines)
- 90s web aesthetic (beveled buttons, tiled backgrounds)
- Windows 95/98 styling (system gray, pixel borders)
- Early Mac OS aesthetic
- Pixel art inspired themes

**Key Techniques:**
- CRT phosphor glow effects
- Scanline overlays
- Monospace typography
- Retro color palettes (amber, green, gray)
- Beveled and embossed borders

### 2. print-ui-styling (Proposed)

Creates themes optimized for reading and print-like experiences.

**Specialty Areas:**
- Newspaper/editorial layouts with columns
- Book/e-reader styling with optimal line lengths
- Academic paper formatting
- Magazine layouts with pull quotes

**Key Techniques:**
- Serif typography for readability
- Multi-column layouts
- Drop caps and hanging punctuation
- Print-friendly color schemes
- Optimal reading line lengths (45-75 characters)

### 3. brand-ui-styling (Proposed)

Creates themes based on major design systems and brand guidelines.

**Specialty Areas:**
- Material Design inspired (cards, elevation, ripples)
- Apple Human Interface inspired (frosted glass, subtle animations)
- Bootstrap-style utility patterns
- Tailwind-inspired design tokens

**Key Techniques:**
- Design token systems
- Component-based elevation
- Brand-specific color science
- Consistent spacing scales

### 4. accessibility-ui-styling (Proposed)

Creates themes with maximum accessibility for users with various needs.

**Specialty Areas:**
- High contrast modes (WCAG AAA)
- Dyslexia-friendly typography (OpenDyslexic, increased letter spacing)
- Large touch targets for motor impairments
- Screen reader optimizations

**Key Techniques:**
- WCAG AAA contrast ratios (7:1 minimum)
- Focus indicators that exceed WCAG requirements
- Reduced motion alternatives
- Semantic color usage (not relying on color alone)

---

## Proposed New Themes

### Using retro-ui-styling Skill

| Theme Name | Description |
|------------|-------------|
| **terminal-dark** (Implemented) | Green/amber text on black, CRT scanlines, monospace fonts, phosphor glow |
| **win95-light** | Classic beveled buttons, system gray, pixel borders, 3D inset/outset effects |
| **dos-dark** | DOS/BIOS style, white/cyan text on blue, block characters |
| **apple2-dark** | Apple II era green phosphor, high contrast, lo-fi aesthetic |

### Using print-ui-styling Skill

| Theme Name | Description |
|------------|-------------|
| **newspaper-light** | Serif fonts, column layouts, editorial feel, black and white with accent |
| **ebook-light** | Warm paper background, comfortable serif, optimal line length |
| **academic-light** | Traditional academic paper styling, Times New Roman, justified text |

### Using brand-ui-styling Skill

| Theme Name | Description |
|------------|-------------|
| **material-light/dark** | Google Material Design inspired cards, elevation shadows, Material palette |
| **cupertino-light/dark** | Apple-inspired frosted glass, SF-style typography, subtle gradients |

### Using accessibility-ui-styling Skill

| Theme Name | Description |
|------------|-------------|
| **high-contrast-dark** | Maximum contrast, large text, clear focus states, simplified styling |
| **dyslexia-friendly-light** | OpenDyslexic font, increased spacing, clear visual hierarchy |

### Using wild-ui-styling Skill

| Theme Name | Description |
|------------|-------------|
| **vaporwave-dark** | Pink/cyan gradients, Greek statue aesthetic, 80s nostalgia |
| **brutalist-light** | Raw, stark design with heavy typography, unconventional layouts |

---

## Implementation Priority

1. **retro-ui-styling** - Implemented with terminal-dark theme
2. **accessibility-ui-styling** - High impact for inclusivity
3. **print-ui-styling** - Useful for content-heavy applications
4. **brand-ui-styling** - Reference implementations for common patterns

---

## Adding a New Skill

1. Create directory: `.claude/skills/{skill-name}/`
2. Create `SKILL.md` with:
   - YAML frontmatter (name, description)
   - Overview and purpose
   - Project context
   - Detailed instructions
   - Theme ideas
3. Create `patterns.md` with reusable CSS patterns
4. Test by asking Claude to use the skill
5. Document in this file and README

## Adding a New Theme

1. Create CSS file in `styles/` directory
2. Follow CSS custom properties convention
3. Add option to `src/components/header.rs`
4. Take screenshot and add to `images/`
5. Update README with screenshot link
