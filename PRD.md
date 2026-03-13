# CommitBee Web — Product Requirements Document

**Version**: 1.1
**Date**: 2026-03-13
**Status**: Phase 1 Implemented
**Author**: [Sephyi](https://github.com/Sephyi) + [Claude Opus 4.6](https://www.anthropic.com/news/claude-opus-4-6)

## Changelog

<details>
<summary>Revision history</summary>

| Version | Date | Summary |
| --- | --- | --- |
| 1.1 | 2026-03-13 | Phase 1 implemented — all FRs built, Leptos 0.8 (was 0.7+), cross-platform Rust prerender binary, web-sys non-optional for island compatibility |
| 1.0 | 2026-03-13 | Initial PRD — landing page, documentation wiki, Leptos architecture, bee-themed design system, GitHub Pages deployment |

</details>

## 1. Vision

> **"The website that makes you want to try the CLI."**

CommitBee Web is the official website for [CommitBee](https://github.com/sephyi/commitbee), serving as both a cinematic landing experience and the canonical documentation wiki. Built entirely in Rust with Leptos, it demonstrates the same engineering philosophy as the CLI itself: semantic understanding, correctness, and craftsmanship.

The site tells the story of how CommitBee understands code through scroll-driven visual storytelling, then provides deep, searchable documentation for developers who are ready to dive in. The warm, organic bee-themed design creates a unique identity in the developer tools space.

### Core Principles

1. **Cinematic first** — The landing page is a visual experience, not a wall of text
2. **Content as code** — Documentation lives as markdown in the repo, rendered at build time
3. **Rust all the way** — Frontend, backend, build tooling — all Rust via Leptos
4. **Zero runtime cost** — Pre-rendered static HTML with surgical WASM hydration for interactive islands only
5. **Accessible by default** — Dark/light mode, reduced motion, semantic HTML, keyboard navigation

## 2. Competitive Landscape

### 2.1 Market Position

| Category | Examples | CommitBee Web Advantage |
| --- | --- | --- |
| CLI tool docs sites | mdBook, Docusaurus, VitePress | Full-stack Rust, cinematic landing, not just docs |
| Dev tool landing pages | Linear, Vercel, Stripe | Warm bee identity vs. generic tech aesthetic |
| Rust WASM sites | Leptos examples, Yew demos | Production-quality design, not a framework demo |

### 2.2 Unique Differentiators

1. **Scroll-driven pipeline walkthrough** — No competing CLI tool visually demonstrates its internals
2. **Bee-themed design system** — Distinctive warm amber palette, hexagonal motifs, immediately recognizable
3. **Full-stack Rust** — Landing page, docs wiki, build pipeline — all one language
4. **Docs from markdown** — Single source of truth between repo and website, zero content drift

## 3. Architecture

### 3.1 Stack

| Component | Technology | Purpose |
| --- | --- | --- |
| Framework | Leptos 0.8 (islands) | SSR with surgical WASM hydration |
| Server | Axum | Build-time HTML rendering (not deployed) |
| Build tool | cargo-leptos | Parallel server/client compilation |
| Styling | Tailwind CSS | Utility-first with dark mode support |
| Markdown | pulldown-cmark | Build-time markdown to HTML |
| Syntax highlighting | syntect | Rust-native, bee-themed color scheme |
| Task runner | mise | Dev/build/content task orchestration |
| CI/CD | GitHub Actions | Build, pre-render, deploy |
| Hosting | GitHub Pages | Static HTML + WASM + CSS |

### 3.2 Rendering Strategy

The site uses Leptos islands architecture with static pre-rendering:

1. `build.rs` processes all markdown content into static Rust data at compile time
2. `cargo-leptos` compiles the Axum server (native) and client WASM (islands only) in parallel
3. In CI, the server runs locally and all routes are pre-rendered to static HTML via the Rust `prerender` binary
4. The static HTML + WASM bundle + CSS deploys to GitHub Pages
5. Islands hydrate client-side only for interactive elements

The Axum server exists solely as a build-time rendering tool. In production, everything is static files served from GitHub Pages.

### 3.3 Target Architecture

```
commitbee-web/
├── src/
│   ├── main.rs               # Axum server entry
│   ├── bin/
│   │   └── prerender.rs      # Cross-platform static HTML pre-renderer
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
│   │   ├── code_block.rs     # Syntax-highlighted code with copy island
│   │   ├── doc_search.rs     # Fuzzy search island
│   │   ├── doc_sidebar.rs    # Docs sidebar navigation
│   │   ├── doc_toc.rs        # Right-side table of contents
│   │   └── scroll_reveal.rs  # Scroll animation wrapper
│   └── content/
│       └── loader.rs         # Build-time markdown loader + frontmatter
├── content/
│   └── docs/                 # Markdown source files with YAML frontmatter
├── style/
│   ├── tailwind.css          # Tailwind directives + bee theme tokens
│   └── animations.css        # Scroll-driven keyframes
├── public/
│   ├── fonts/                # Inter + JetBrains Mono (self-hosted)
│   └── images/               # OG images, favicon, bee assets
├── build.rs                  # Markdown processing pipeline
├── mise.toml                 # Task orchestration
├── Cargo.toml
└── .github/
    └── workflows/
        └── deploy.yml        # Build + pre-render + deploy
```

### 3.4 Build Pipeline

```
build.rs                          cargo-leptos
   |                                   |
   +- Walk content/docs/*.md           +- Compile server (native)
   +- Parse YAML frontmatter           +- Compile client WASM (islands only)
   +- Markdown -> HTML (pulldown-cmark) +- Process Tailwind CSS
   +- Syntax highlight (syntect)       +- wasm-opt optimization
   +- Extract heading tree for TOC
   +- Build search index
   +- Generate $OUT_DIR/content_generated.rs

CI pre-render (cross-platform Rust binary, zero shell deps):
   +- Build release binary + WASM bundle
   +- Start Axum server locally (background)
   +- Pre-render all known routes via HTTP/1.0 GET -> static HTML
   +- Add 404.html for GitHub Pages SPA fallback
   +- Deploy dist/ to GitHub Pages
```

### 3.5 Key Dependencies

| Crate | Purpose |
| --- | --- |
| `leptos` 0.8 (features: `islands`) | Framework with islands architecture |
| `leptos_meta` | `<title>`, `<meta>`, OG tags per page |
| `leptos_router` | Client-side routing |
| `leptos_axum` | Axum integration for SSR |
| `axum` | HTTP server (build-time only) |
| `tokio` | Async runtime |
| `pulldown-cmark` | Markdown to HTML |
| `syntect` | Syntax highlighting |
| `serde` + `serde_yaml` | Frontmatter parsing |
| `tower-http` | Static file serving, compression |
| `wasm-bindgen` + `web-sys` | Browser API access (localStorage, IntersectionObserver, Clipboard) — non-optional because `#[island]` macro does not cfg-gate the body |

## 4. Feature Requirements

### 4.1 Phase 1 — Landing Page

#### FR-001: Hero Section

Fullscreen hero with tagline "The commit message generator that actually understands your code." Animated honeycomb motif in background (CSS `clip-path` or SVG pattern, gentle pulse animation). Terminal mockup showing commitbee producing a commit message. Two CTAs: "Get Started" (scrolls to install) and "See How It Works" (scrolls to pipeline). Terminal fades and scales down on scroll.

#### FR-002: Problem Statement Section

Split view layout: left side shows raw `git diff` dump, right side shows a vague commit message from a typical tool. Headline: "Every other tool just pipes your diff to an LLM and hopes for the best." Scroll triggers visual transformation where raw diff morphs into structured semantic context.

#### FR-003: Pipeline Demo Animation

Pre-baked animated walkthrough of commitbee's 7-stage pipeline using a real example diff. Each stage appears as a card that slides in on scroll:

1. Git Service — staged files appear
2. Tree-sitter — symbols highlighted in code
3. Splitter — files visually group into clusters
4. Context Builder — evidence flags appear, budget meter fills
5. LLM — streaming text animation of commit message generation
6. Validator — checkmarks appear next to each of the 7 rules
7. Sanitizer — final clean commit message appears

Implemented as an `#[island]` component with play/pause and step-through controls. All animation data is pre-baked, not computed live.

#### FR-004: Differentiator Cards

Three feature cards with hexagonal accent borders:

- Tree-sitter Semantic Analysis — "It reads your code, not just your diffs"
- Commit Splitting — "It detects mixed concerns and splits them"
- 25-Pattern Secret Scanning — "It catches leaked credentials before they reach any LLM"

Each card has a mini-animation showing the feature in action.

#### FR-005: Competitive Comparison

Stylized comparison of commitbee vs. the field on key features. Layered card design (not a flat table). CommitBee column accented with amber glow. Features compared: tree-sitter AST, commit splitting, secret scanning, token budget, streaming, local LLM, cloud providers, git hooks.

#### FR-006: Install and Quick Start

Terminal-style code blocks with copy-to-clipboard buttons. Shows `cargo install commitbee`, `brew install sephyi/tap/commitbee`, and the zero-config first-run experience. Animated terminal showing the first-run flow.

#### FR-007: Docs Transition Gateway

Visual transition where landing page aesthetic gradually shifts into documentation layout. Cards linking to key doc sections: Getting Started, Configuration, Providers, Architecture. Smooth visual transition into docs navigation.

#### FR-008: Footer

GitHub repo, crates.io, license links. "Made with Rust" badge. Subtle honeycomb pattern background.

### 4.2 Phase 1 — Documentation Wiki

#### FR-010: Doc Page Renderer

Renders markdown content as HTML with sidebar navigation and table of contents. Three-column layout on desktop: left sidebar (doc tree), main content (~75ch max-width), right sidebar (page TOC). Responsive: sidebar becomes slide-out drawer on mobile, TOC collapses.

#### FR-011: Markdown Content Pipeline

Markdown files in `content/docs/` with YAML frontmatter (`title`, `order`, `section`, `description`). Processed at build time by `build.rs`:

1. Walk directory, parse frontmatter
2. Convert markdown to HTML via `pulldown-cmark`
3. Apply syntax highlighting via `syntect`
4. Extract heading tree for TOC generation
5. Build search index (title + headings + first 200 words per page)
6. Generate Rust module with doc tree as `const` statics in `$OUT_DIR`

#### FR-012: Doc Navigation

Left sidebar with collapsible section tree. Current page highlighted with amber accent. Sticky on desktop, slide-out drawer on mobile. Prev/next links at bottom of each page based on ordering. Breadcrumbs: Docs > Section > Page.

Section structure:

| Section | Pages |
| --- | --- |
| Basics | Getting Started, How It Works |
| Usage | Commands & Flags, Configuration, LLM Providers, Commit Splitting |
| Internals | Validation Pipeline, Security & Safety, Supported Languages |
| Integration | Git Hooks, Troubleshooting |
| Reference | Architecture |

#### FR-013: Doc Search

Client-side fuzzy search over pre-built search index. `Cmd+K` / `Ctrl+K` keyboard shortcut to open search overlay. Searches titles, headings, and content excerpts. Results ranked by relevance. Implemented as an `#[island]` component. Search index emitted as a separate JSON file by `build.rs`, loaded lazily when the search modal opens (not baked into WASM to keep bundle small). Fuzzy matching via `sublime_fuzzy` or equivalent compiled into the search island.

#### FR-014: Code Blocks

Syntax-highlighted code blocks via `syntect` (rendered at build time). Copy-to-clipboard button on each block. Language label displayed. Bee-themed syntax color scheme for both light and dark modes.

#### FR-015: Table of Contents

Right sidebar (desktop only) auto-generated from page headings. Highlights current section on scroll via `IntersectionObserver`. Smooth-scrolls to heading on click.

### 4.3 Phase 1 — Design System

#### FR-020: Bee-Themed Color Palette

Warm amber/honey color tokens:

**Light mode:** `--honey` (#F59E0B), `--honey-light` (#FCD34D), `--honey-dark` (#D97706), `--nectar` (#FFFBEB), `--comb` (#78716C), `--bark` (#1C1917), `--pollen` (#FEF3C7), `--surface` (#FFFFFF), `--surface-raised` (#FAFAF9).

**Dark mode:** `--honey` (#FBBF24), `--nectar` (#1C1917), `--comb` (#A8A29E), `--bark` (#FAFAF9), `--pollen` (#292524), `--surface` (#0C0A09), `--surface-raised` (#1C1917).

#### FR-021: Dark/Light Mode

Auto-detects `prefers-color-scheme` by default. Manual toggle via `#[island]` component in navigation bar. User preference persisted to `localStorage`. Implemented with Tailwind `dark:` variants. Smooth transition between modes.

#### FR-022: Typography

Inter for headings and body text. JetBrains Mono for code. Self-hosted fonts in `public/fonts/` (no external font dependencies).

#### FR-023: Scroll Animations

`IntersectionObserver` via `#[island]` as the baseline implementation (works in all browsers). CSS `animation-timeline: scroll()` as progressive enhancement for Chromium browsers. Motion style: fade-up and slide-in, smooth and organic. Respects `prefers-reduced-motion` — all animations disabled when set.

#### FR-024: Hexagonal Motifs

Subtle hex grid pattern in hero background. Feature card borders with hex-inspired angled corners (hints, not literal hexagons). Section dividers use gentle honeycomb wave instead of straight lines. Restrained — enhances the theme without distracting from content.

#### FR-025: Sticky Navigation

Fixed navigation bar at top of page. Shows commitbee logo/name, primary nav links (Home, Docs, GitHub). On docs pages, adds search bar. Transparent background on hero, solid background after scroll. Mobile: hamburger menu with slide-out drawer.

### 4.4 Phase 1 — Infrastructure

#### FR-030: GitHub Actions CI/CD

GitHub Actions workflow triggered on push to `development` branch:

1. `cargo leptos build --release`
2. Start Axum server locally in background
3. Pre-render all routes to static HTML via wget
4. Add `404.html` for GitHub Pages SPA fallback
5. Deploy `dist/` to GitHub Pages via `actions/deploy-pages`

#### FR-031: mise Task Orchestration

`mise.toml` with tasks: `dev` (cargo leptos watch), `build` (cargo leptos build --release), `content` (validate markdown frontmatter and links), `prerender` (pre-render all routes to static HTML via Rust binary), `fmt` (cargo fmt), `check` (clippy for both SSR and WASM).

#### FR-032: Client-Side Routing

`leptos_router` for navigation between pages without full reloads. URL structure: `/` (landing), `/docs/:slug` (doc pages). Fallback route for 404.

## 5. Security Requirements

### SR-001: No Secrets in Source

No API keys, credentials, or secrets in the codebase. The site is entirely static with no server-side secrets needed.

### SR-002: Content Security

Self-hosted fonts and assets only. No external script dependencies. No third-party analytics or tracking by default. Content-Security-Policy headers via `<meta>` tag in HTML shell.

### SR-003: Dependency Auditing

`cargo audit` in CI. Minimize dependency tree. Pin WASM toolchain versions.

## 6. Performance Requirements

### PR-001: First Contentful Paint

Target: < 1.5s on 3G connection. Pre-rendered HTML ensures content is visible before WASM loads. Critical CSS inlined or loaded with high priority.

### PR-002: WASM Bundle Size

Islands-only WASM: target < 200KB gzipped. Only interactive components compiled to WASM. Static content is pure HTML with zero JS overhead.

### PR-003: Lighthouse Score

Target: > 95 on all four Lighthouse categories (Performance, Accessibility, Best Practices, SEO) for both landing page and doc pages.

### PR-004: Font Loading

Self-hosted fonts with `font-display: swap`. Preload critical font weights. System font stack as fallback.

### PR-005: Image Optimization

OG images and assets served in modern formats (WebP with fallback). Lazy loading for below-the-fold images.

## 7. UX Requirements

### UX-001: Accessibility

WCAG 2.1 AA compliance. Semantic HTML throughout. Keyboard navigation for all interactive elements. Focus indicators on all interactive elements. Screen reader compatible navigation and content structure. `prefers-reduced-motion` respected for all animations.

### UX-002: Responsive Design

Three breakpoints: mobile (< 768px), tablet (768-1024px), desktop (> 1024px). Doc sidebar collapses to drawer on mobile. TOC hidden on mobile and tablet. Landing page sections stack vertically on mobile with simplified animations.

### UX-003: Dark/Light Mode

Automatic detection of system preference. Manual override with persistent toggle. No flash of wrong theme on page load (theme applied before first paint via inline script or server-rendered class).

### UX-004: Navigation

Sticky top nav visible at all times. Doc sidebar shows full page tree with current page highlighted. Breadcrumbs on doc pages for orientation. Prev/next links for sequential reading. `Cmd+K` / `Ctrl+K` search shortcut discoverable via nav hint.

## 8. Testing Requirements

### TR-001: Build Verification

All markdown files parse without errors. Frontmatter validates against schema. No broken internal links between doc pages. Search index generates successfully.

### TR-002: Component Tests

Leptos component tests for interactive islands (theme toggle, search, code copy). Verify hydration works correctly after pre-rendering.

### TR-003: Visual Regression

Lighthouse CI checks on every PR. Screenshot comparison for landing page and key doc pages (optional, Phase 2).

### TR-004: Accessibility Testing

`axe-core` or equivalent automated accessibility checks in CI. Verify keyboard navigation flow. Verify screen reader landmarks.

### TR-005: Cross-Browser

Test on: Chrome (latest), Firefox (latest), Safari (latest). WASM hydration must work on all three. CSS scroll-timeline fallback verified on Firefox/Safari.

## 9. Deployment Requirements

### DR-001: GitHub Pages

Static HTML + WASM + CSS + assets deployed to `gh-pages` branch. Custom domain support via CNAME file in `public/`.

### DR-002: Pre-Render Pipeline

All routes pre-rendered at build time. Route list auto-generated by `build.rs` into a `routes.txt` manifest (doc slugs from content pipeline + landing page). A cross-platform Rust binary (`src/bin/prerender.rs`) starts the Axum server locally, iterates the manifest, and fetches each route via HTTP/1.0 GET to `path/index.html` (e.g., `docs/getting-started/index.html`). Zero shell dependencies — works on Linux, macOS, and Windows. `404.html` generated for unmatched routes.

### DR-003: Cache Strategy

Hashed asset filenames for long-term caching. HTML files served with short cache TTL for quick content updates. WASM bundle and CSS fingerprinted.

## 10. Roadmap Summary

| Phase | Version | Status | Focus |
| --- | --- | --- | --- |
| 1 | v1.0 | Implemented | Landing page, docs wiki, design system, deployment pipeline |
| 2 | v1.1 | Future | i18n support, visual regression tests, enhanced pipeline demo |
| 3 | v2.0 | Future | WASM playground (run commitbee internals in browser), blog/changelog |

## 11. Success Metrics

| Metric | Target | Measurement |
| --- | --- | --- |
| Lighthouse Performance | > 95 | CI Lighthouse audit |
| Lighthouse Accessibility | > 95 | CI Lighthouse audit |
| WASM bundle size | < 200KB gzipped | CI artifact size |
| First Contentful Paint | < 1.5s (3G) | Lighthouse |
| Build time | < 5 minutes | GitHub Actions duration |
| Doc page count | >= 12 (all DOCS.md sections) | Build output |
| Broken links | 0 | Build-time validation |

## 12. Non-Goals

- **WASM playground** — No live compilation of commitbee internals in the browser for v1. Pipeline demo uses pre-baked animation.
- **Blog / changelog** — Not in scope for v1. Documentation wiki only.
- **User accounts** — Static site with no server-side state or authentication.
- **Analytics dashboard** — No built-in analytics. Optional privacy-respecting analytics (e.g., Plausible) can be added later.
- **Comments / discussion** — Use GitHub Discussions for community interaction.
- **i18n content translation** — English only for v1. Architecture supports future locale directories.
- **Server-side runtime** — No persistent server. Everything pre-rendered to static files.

## Appendix A: Design System Reference

See `docs/superpowers/specs/2026-03-13-commitbee-web-design.md` for the full design specification including:

- Detailed color palette with hex values for both light and dark modes
- Typography specifications
- Hexagonal motif guidelines
- Scroll animation specifications
- Component inventory

## Appendix B: Content Source Mapping

Documentation content is derived from the commitbee repository's `DOCS.md`, split into individual markdown files:

| Doc Page | Source Section |
| --- | --- |
| getting-started.md | DOCS.md: Getting Started |
| how-it-works.md | DOCS.md: How It Works |
| configuration.md | DOCS.md: Configuration |
| commands-and-flags.md | DOCS.md: Commands & Flags |
| llm-providers.md | DOCS.md: LLM Providers |
| commit-splitting.md | DOCS.md: Commit Splitting |
| validation-pipeline.md | DOCS.md: The Validation Pipeline |
| security-and-safety.md | DOCS.md: Security & Safety |
| git-hooks.md | DOCS.md: Git Hook Integration |
| supported-languages.md | DOCS.md: Supported Languages |
| troubleshooting.md | DOCS.md: Troubleshooting |
| architecture.md | DOCS.md: Architecture Deep Dive |
