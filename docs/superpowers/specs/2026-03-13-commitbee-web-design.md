# CommitBee Web — Design Specification

**Version**: 1.0
**Date**: 2026-03-13
**Status**: Approved

## 1. Overview

CommitBee Web is the official website for [CommitBee](https://github.com/sephyi/commitbee), a Rust-native CLI tool that uses tree-sitter semantic analysis and LLMs to generate high-quality conventional commit messages.

The site serves two purposes in a unified experience:

1. **Cinematic landing page** — scroll-driven visual storytelling that demonstrates how commitbee understands code, with smooth animations and a pre-baked pipeline walkthrough
2. **Documentation wiki** — the full commitbee documentation rendered from markdown source files, with sidebar navigation, table of contents, fuzzy search, and syntax-highlighted code blocks

## 2. Design Decisions

| Decision | Choice | Rationale |
| --- | --- | --- |
| Framework | Leptos 0.7+ (Rust) | Full-stack Rust, aligns with commitbee identity, islands architecture for surgical hydration |
| Rendering | SSR with islands + static pre-render | SEO-friendly pre-rendered HTML, interactive elements hydrate via WASM |
| Styling | Tailwind CSS + small custom CSS | Utility-first, built-in dark mode, cargo-leptos has native Tailwind support |
| Markdown | pulldown-cmark at build time | Zero runtime parsing, content baked into binary as const statics |
| Syntax highlighting | syntect at build time | Rust-native, bee-themed color scheme, no client-side JS needed |
| Deployment | GitHub Pages via GitHub Actions | Free, CI-driven, static output from pre-render step |
| Design identity | Warm/organic bee theme | Honey amber palette, hexagonal motifs, unique in the dev tool space |
| Dark/light mode | Auto (prefers-color-scheme) + manual toggle | Toggle persists to localStorage, island component |
| i18n | English only, architecture ready for expansion | Deferred — content structure supports future locale directories |

## 3. Architecture

### 3.1 Stack

- **Leptos 0.7+** with `islands` feature
- **Axum** as the HTTP server (build-time only — not deployed)
- **cargo-leptos** as the build tool
- **pulldown-cmark** for markdown to HTML conversion
- **syntect** for syntax highlighting
- **Tailwind CSS v4** for styling
- **GitHub Actions** for CI/CD
- **GitHub Pages** for hosting

### 3.2 How It Works

1. At build time, `build.rs` processes all markdown content into static Rust data
2. `cargo-leptos` compiles the server (native) and client (WASM) in parallel
3. In CI, the server runs locally and all routes are pre-rendered to static HTML
4. The static HTML + WASM bundle + CSS deploys to GitHub Pages
5. Islands hydrate client-side for interactive elements (theme toggle, pipeline demo, search, code copy buttons)

The Axum server exists only as a build-time rendering tool. In production, everything is static files.

### 3.3 Project Structure

```
commitbee-web/
├── Cargo.toml
├── mise.toml                 # Task orchestration
├── tailwind.config.js        # Tailwind configuration with bee theme
├── src/
│   ├── main.rs               # Axum server entry
│   ├── lib.rs                # Leptos app root + hydrate entry
│   ├── app.rs                # Router + HTML shell
│   ├── pages/
│   │   ├── landing.rs        # Hero + all landing sections
│   │   ├── docs.rs           # Doc page renderer
│   │   └── not_found.rs      # 404 page
│   ├── components/
│   │   ├── nav.rs            # Sticky navigation header
│   │   ├── footer.rs         # Site footer
│   │   ├── theme_toggle.rs   # Dark/light mode island
│   │   ├── pipeline_demo.rs  # Animated pipeline walkthrough island
│   │   ├── code_block.rs     # Syntax-highlighted code with copy button island
│   │   ├── doc_search.rs     # Fuzzy search island
│   │   ├── doc_sidebar.rs    # Docs sidebar navigation
│   │   ├── doc_toc.rs        # Right-side table of contents
│   │   └── scroll_reveal.rs  # Scroll animation wrapper
│   └── content/
│       ├── loader.rs         # Build-time markdown loader + frontmatter parser
│       └── generated.rs      # Auto-generated: doc tree as const statics
├── content/
│   └── docs/
│       ├── getting-started.md
│       ├── how-it-works.md
│       ├── configuration.md
│       ├── commands-and-flags.md
│       ├── llm-providers.md
│       ├── commit-splitting.md
│       ├── validation-pipeline.md
│       ├── security-and-safety.md
│       ├── supported-languages.md
│       ├── git-hooks.md
│       ├── troubleshooting.md
│       └── architecture.md
├── style/
│   ├── tailwind.css          # Tailwind directives + bee theme CSS custom properties
│   └── animations.css        # Scroll-driven keyframes + complex effects
├── public/
│   ├── fonts/                # Inter + JetBrains Mono (self-hosted)
│   └── images/               # OG images, favicon, bee assets
├── build.rs                  # Markdown processing pipeline
└── .github/
    └── workflows/
        └── deploy.yml        # Build + pre-render + deploy to GitHub Pages
```

### 3.4 Key Dependencies

| Crate | Purpose |
| --- | --- |
| `leptos` (features: `islands`) | Framework with islands architecture |
| `leptos_meta` | `<title>`, `<meta>` tag management |
| `leptos_router` | Client-side routing |
| `leptos_axum` | Axum integration for SSR |
| `axum` | HTTP server (build-time only) |
| `tokio` | Async runtime |
| `pulldown-cmark` | Markdown to HTML |
| `syntect` | Syntax highlighting |
| `serde` + `serde_yaml` | Frontmatter parsing |
| `tower-http` | Static file serving, compression |

### 3.5 Build Pipeline

```
build.rs                          cargo-leptos
   │                                   │
   ├─ Walk content/docs/*.md           ├─ Compile server (native)
   ├─ Parse YAML frontmatter           ├─ Compile client WASM (islands only)
   ├─ Markdown → HTML (pulldown-cmark) ├─ Process Tailwind CSS
   ├─ Syntax highlight (syntect)       └─ wasm-opt optimization
   ├─ Extract heading tree for TOC
   ├─ Build search index
   └─ Generate src/content/generated.rs

CI pre-render step:
   ├─ Start server locally
   ├─ Crawl all routes → static HTML
   ├─ Collect HTML + WASM + CSS + fonts + images
   └─ Deploy to GitHub Pages
```

## 4. Landing Page

The landing page is a cinematic scroll experience. Each section is revealed by scroll-driven CSS animations. The page flows top-to-bottom through these sections:

### 4.1 Hero

- Tagline: "The commit message generator that actually understands your code."
- Animated honeycomb motif in background (CSS `clip-path` / SVG pattern, gentle pulse)
- Terminal mockup showing commitbee producing a commit message
- CTAs: "Get Started" (scrolls to install section) and "See How It Works" (scrolls to pipeline)
- On scroll: terminal mockup fades and scales down

### 4.2 The Problem

- Split view: left shows raw `git diff`, right shows a vague commit message from a typical tool
- Headline: "Every other tool just pipes your diff to an LLM and hopes for the best."
- Scroll triggers visual transformation — raw diff morphs into structured semantic context

### 4.3 Pipeline Demo (showpiece)

Pre-baked animated walkthrough of commitbee's 7-stage pipeline using a real example diff. Each stage appears as a card that slides in on scroll:

1. **Git Service** — staged files appear
2. **Tree-sitter** — symbols light up in the code, functions/structs highlighted
3. **Splitter** — files visually group into clusters
4. **Context Builder** — evidence flags appear, budget meter fills
5. **LLM** — streaming text animation of commit message generation
6. **Validator** — checkmarks appear next to each of the 7 rules
7. **Sanitizer** — final clean commit message appears

Implemented as an `#[island]` component with play/pause and step-through controls.

### 4.4 Differentiators

Three feature cards with hexagonal accent borders:

- **Tree-sitter Semantic Analysis** — "It reads your code, not just your diffs"
- **Commit Splitting** — "It detects mixed concerns and splits them"
- **25-Pattern Secret Scanning** — "It catches leaked credentials before they reach any LLM"

Each card has a mini-animation showing the feature in action.

### 4.5 Competitive Edge

Stylized comparison (layered cards, not a flat table). CommitBee vs. the field on key features. CommitBee column accented with amber glow.

### 4.6 Install + Quick Start

Terminal-style code blocks with copy buttons. Shows `cargo install commitbee`, `brew install`, and the zero-config first-run experience with an animated terminal.

### 4.7 Transition to Docs

Visual gateway where the landing page aesthetic gradually shifts into the documentation layout. Cards linking to key doc sections. Smooth transition into docs navigation.

### 4.8 Footer

GitHub repo, crates.io, license links. "Made with Rust" badge. Subtle honeycomb pattern.

## 5. Documentation Wiki

### 5.1 Layout

- **Sticky top nav** — shared with landing page, doc search bar added
- **Left sidebar** — collapsible section tree, current page highlighted with amber accent, sticky on desktop, slide-out drawer on mobile
- **Main content** — rendered markdown, max-width ~75ch for readability
- **Right sidebar (desktop)** — auto-generated table of contents from headings, highlights current section on scroll

### 5.2 Content Pipeline

Markdown files in `content/docs/` with YAML frontmatter:

```yaml
---
title: "Getting Started"
order: 1
section: "Basics"
description: "Install commitbee and generate your first commit message"
---
```

At build time (`build.rs`):

1. Walk `content/docs/`, parse frontmatter
2. Convert markdown to HTML via `pulldown-cmark`
3. Apply syntax highlighting via `syntect`
4. Extract heading structure for TOC generation
5. Build search index (title + headings + first 200 words per page)
6. Generate `src/content/generated.rs` with doc tree as `const` statics

### 5.3 Section Structure

```
Basics/
  getting-started.md
  how-it-works.md

Usage/
  commands-and-flags.md
  configuration.md
  llm-providers.md
  commit-splitting.md

Internals/
  validation-pipeline.md
  security-and-safety.md
  supported-languages.md

Integration/
  git-hooks.md
  troubleshooting.md

Reference/
  architecture.md
```

### 5.4 Interactive Elements (Islands)

- **Doc search** — client-side fuzzy search over pre-built index, `Cmd+K` / `Ctrl+K` keyboard shortcut
- **Code blocks** — copy-to-clipboard button, language label, syntax highlighting
- **Mobile nav** — hamburger toggle for sidebar drawer

### 5.5 Navigation

- Client-side routing via `leptos_router` — no full page reloads
- URL structure: `/docs/getting-started`, `/docs/configuration`, etc.
- Prev/next links at bottom of each page
- Breadcrumbs: Docs > Section > Page

## 6. Design System

### 6.1 Color Palette

**Light mode:**

| Token | Value | Usage |
| --- | --- | --- |
| `--honey` | `#F59E0B` | Primary accent, CTAs, active states |
| `--honey-light` | `#FCD34D` | Hover states, subtle highlights |
| `--honey-dark` | `#D97706` | Pressed states, borders |
| `--nectar` | `#FFFBEB` | Background tints, card backgrounds |
| `--comb` | `#78716C` | Secondary text |
| `--bark` | `#1C1917` | Primary text |
| `--pollen` | `#FEF3C7` | Code block backgrounds, callouts |
| `--surface` | `#FFFFFF` | Page background |
| `--surface-raised` | `#FAFAF9` | Cards, sidebar |

**Dark mode:**

| Token | Value | Usage |
| --- | --- | --- |
| `--honey` | `#FBBF24` | Primary accent (brighter for contrast) |
| `--honey-light` | `#FCD34D` | Hover states |
| `--honey-dark` | `#F59E0B` | Pressed states |
| `--nectar` | `#1C1917` | Background tints |
| `--comb` | `#A8A29E` | Secondary text |
| `--bark` | `#FAFAF9` | Primary text |
| `--pollen` | `#292524` | Code block backgrounds |
| `--surface` | `#0C0A09` | Page background |
| `--surface-raised` | `#1C1917` | Cards, sidebar |

### 6.2 Typography

- **Headings + Body:** Inter (self-hosted)
- **Code:** JetBrains Mono (self-hosted)
- No external font dependencies (no Google Fonts)

### 6.3 Hexagonal Motifs

- Subtle hex grid pattern in hero background (CSS `clip-path` or SVG pattern)
- Feature card borders with hex-inspired angled corners (hint, not literal hexagons)
- Section dividers use gentle honeycomb wave instead of straight lines
- Restrained — enhances, never distracts

### 6.4 Scroll Animations

- CSS-first using `animation-timeline: scroll()` and `animation-range`
- Fallback: `IntersectionObserver` via `#[island]` for unsupported browsers
- Motion style: fade-up and slide-in, smooth and organic ("floating" not "snapping")
- Respects `prefers-reduced-motion` — all animations disabled, content appears statically

### 6.5 Dark/Light Mode

- Auto-detects `prefers-color-scheme` by default
- Manual toggle via `#[island]` component in navigation
- User preference persisted to `localStorage`
- Tailwind `dark:` variants for all color tokens

## 7. Deployment

### 7.1 GitHub Pages

- Static HTML + WASM + CSS + assets deployed to `gh-pages` branch
- Custom domain support via CNAME file in `public/`

### 7.2 CI Pipeline (GitHub Actions)

```
On push to development:
  1. cargo leptos build --release
  2. Start server locally
  3. Pre-render all routes to static HTML
  4. Collect output (HTML + WASM + CSS + fonts + images)
  5. Deploy to GitHub Pages
```

### 7.3 mise Tasks

```toml
[tasks.dev]
run = "cargo leptos watch"
description = "Start dev server with hot reload"

[tasks.build]
run = "cargo leptos build --release"
description = "Production build"

[tasks.content]
run = "cargo test --test content_check"
description = "Validate markdown frontmatter and links"
```

## 8. Non-Goals

- **WASM playground** — No live compilation of commitbee internals in the browser. Pipeline demo is pre-baked animation.
- **Blog / changelog** — Not in scope for v1. Documentation wiki only.
- **User accounts / analytics dashboard** — Static site, no server-side state.
- **Comments / discussion** — Use GitHub Discussions for community interaction.
- **i18n content translation** — English only for v1. Architecture supports future expansion.
