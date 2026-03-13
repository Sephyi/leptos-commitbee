# CommitBee Web Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the official CommitBee website as a Leptos SSR + islands app with a cinematic landing page and documentation wiki, deployed as static HTML to GitHub Pages.

**Architecture:** Leptos 0.7+ islands architecture with Axum as build-time SSR renderer. `build.rs` processes markdown docs into static Rust data at compile time. CI pre-renders all routes to static HTML via wget, deploys to GitHub Pages. Interactive elements (theme toggle, search, pipeline demo, code copy) hydrate as WASM islands.

**Tech Stack:** Leptos 0.7+ (islands), Axum, cargo-leptos, Tailwind CSS, pulldown-cmark, syntect, wasm-bindgen/web-sys, GitHub Actions, GitHub Pages, mise

**Spec:** `docs/superpowers/specs/2026-03-13-commitbee-web-design.md`
**PRD:** `PRD.md`

## Parallelization Map

```
Chunk 1 (Scaffolding) ──────────────────────────────┐
                                                     │
                            ┌────────────────────────┤
                            v                        v
                   Chunk 2 (Design)         Chunk 3 (Content Pipeline)
                            │                        │
                            └───────────┬────────────┘
                                        v
                               Chunk 4 (Components)
                                        │
                                        v
                               Chunk 5 (Pages)
                                        │
                            ┌───────────┤
                            v           v
                   Chunk 6 (Islands)  Chunk 7 (Deployment)
                            │           │
                            └─────┬─────┘
                                  v
                          Chunk 8 (Integration)
```

**Independent pairs (can run in parallel):**
- Chunk 2 + Chunk 3 (after Chunk 1)
- Chunk 6 + Chunk 7 (after Chunk 5)
- Within Chunk 4: nav/footer are independent of sidebar/toc
- Within Chunk 6: all four islands are independent

## File Structure

### Files to Create

| File | Purpose |
| --- | --- |
| `Cargo.toml` | Project manifest with Leptos islands, all dependencies |
| `mise.toml` | Task orchestration (dev, build, content) |
| `tailwind.config.js` | Tailwind theme extending bee color palette |
| `rust-toolchain.toml` | Pin nightly toolchain (required by Leptos WASM) |
| `src/main.rs` | Axum server entry point (SSR only) |
| `src/lib.rs` | App module exports + WASM hydrate entry |
| `src/app.rs` | Router, HTML shell, `<App/>` component |
| `src/pages/mod.rs` | Page module exports |
| `src/pages/landing.rs` | Hero + all landing sections |
| `src/pages/docs.rs` | Doc page renderer with sidebar + TOC |
| `src/pages/not_found.rs` | 404 page |
| `src/components/mod.rs` | Component module exports |
| `src/components/nav.rs` | Sticky navigation header |
| `src/components/footer.rs` | Site footer |
| `src/components/scroll_reveal.rs` | Scroll animation wrapper |
| `src/components/doc_sidebar.rs` | Left sidebar navigation |
| `src/components/doc_toc.rs` | Right-side table of contents |
| `src/components/theme_toggle.rs` | Dark/light mode island |
| `src/components/code_block.rs` | Syntax-highlighted code with copy island |
| `src/components/doc_search.rs` | Fuzzy search island |
| `src/components/pipeline_demo.rs` | Animated pipeline walkthrough island |
| `src/content/mod.rs` | Content module exports |
| `src/content/loader.rs` | Include generated content, expose typed API |
| `build.rs` | Markdown processing pipeline |
| `style/tailwind.css` | Tailwind directives + bee theme CSS custom properties |
| `style/animations.css` | Scroll-driven keyframes + complex effects |
| `public/images/.gitkeep` | Placeholder for images directory |
| `public/fonts/.gitkeep` | Placeholder for fonts directory |
| `content/docs/getting-started.md` | Doc: Getting Started |
| `content/docs/how-it-works.md` | Doc: How It Works |
| `content/docs/configuration.md` | Doc: Configuration |
| `content/docs/commands-and-flags.md` | Doc: Commands & Flags |
| `content/docs/llm-providers.md` | Doc: LLM Providers |
| `content/docs/commit-splitting.md` | Doc: Commit Splitting |
| `content/docs/validation-pipeline.md` | Doc: Validation Pipeline |
| `content/docs/security-and-safety.md` | Doc: Security & Safety |
| `content/docs/supported-languages.md` | Doc: Supported Languages |
| `content/docs/git-hooks.md` | Doc: Git Hooks |
| `content/docs/troubleshooting.md` | Doc: Troubleshooting |
| `content/docs/architecture.md` | Doc: Architecture |
| `tests/content_check.rs` | Validate frontmatter schema, internal links |
| `.github/workflows/deploy.yml` | Build + pre-render + deploy to GitHub Pages |

## Chunk 1: Project Scaffolding

### Task 1: Create Cargo.toml

**Files:**
- Create: `Cargo.toml`

- [ ] **Step 1: Create Cargo.toml with all dependencies and cargo-leptos metadata**

```toml
[package]
name = "commitbee-web"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
leptos = { version = "0.8" }
leptos_meta = { version = "0.8" }
leptos_router = { version = "0.8" }
leptos_axum = { version = "0.7", optional = true }
axum = { version = "0.8", optional = true }
tokio = { version = "1", features = ["full"], optional = true }
tower = { version = "0.5", optional = true }
tower-http = { version = "0.6", features = ["fs", "compression-gzip"], optional = true }
serde = { version = "1", features = ["derive"] }
serde_json = { version = "1" }
wasm-bindgen = { version = "0.2", optional = true }
wasm-bindgen-futures = { version = "0.4", optional = true }
js-sys = { version = "0.3", optional = true }
web-sys = { version = "0.3", optional = true, features = [
    "Storage",
    "Window",
    "Document",
    "HtmlElement",
    "IntersectionObserver",
    "IntersectionObserverEntry",
    "IntersectionObserverInit",
    "Clipboard",
    "Navigator",
    "MediaQueryList",
    "KeyboardEvent",
    "Response",
    "Headers",
] }
console_error_panic_hook = { version = "0.1", optional = true }
sublime_fuzzy = { version = "0.7", optional = true }

[build-dependencies]
pulldown-cmark = { version = "0.13", features = ["html"] }
syntect = { version = "5", default-features = false, features = ["default-fancy"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1"
walkdir = "2"
slug = "0.1"

[dev-dependencies]
serde_yaml = "0.9"
walkdir = "2"

[features]
default = []
hydrate = [
    "leptos/hydrate",
    "leptos/islands",
    "leptos_meta/hydrate",
    "leptos_router/hydrate",
    "dep:wasm-bindgen",
    "dep:wasm-bindgen-futures",
    "dep:js-sys",
    "dep:web-sys",
    "dep:console_error_panic_hook",
    "dep:sublime_fuzzy",
]
ssr = [
    "leptos/ssr",
    "leptos/islands",
    "leptos_meta/ssr",
    "leptos_router/ssr",
    "dep:leptos_axum",
    "dep:axum",
    "dep:tokio",
    "dep:tower",
    "dep:tower-http",
]

[package.metadata.leptos]
output-name = "commitbee-web"
site-root = "target/site"
site-pkg-dir = "pkg"
tailwind-input-file = "style/tailwind.css"
tailwind-config-file = "tailwind.config.js"
assets-dir = "public"
site-addr = "127.0.0.1:3000"
reload-port = 3001
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
hash-files = true

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

- [ ] **Step 2: Create rust-toolchain.toml**

```toml
[toolchain]
channel = "nightly"
targets = ["wasm32-unknown-unknown"]
```

Leptos WASM compilation requires the `wasm32-unknown-unknown` target and nightly for some features.

- [ ] **Step 3: Verify Cargo.toml parses**

Run: `cargo metadata --format-version 1 --no-deps 2>&1 | head -1`

Expected: JSON output beginning with `{` (or a dependency resolution error, which is fine at this stage since we haven't created source files yet).

### Task 2: Create mise.toml

**Files:**
- Create: `mise.toml`

- [ ] **Step 1: Create mise.toml with dev/build/content tasks**

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

[tasks.fmt]
run = "cargo fmt"
description = "Format Rust code"

[tasks.check]
run = "cargo clippy --features ssr -- -D warnings && cargo clippy --features hydrate --target wasm32-unknown-unknown -- -D warnings"
description = "Lint server and client code"
```

- [ ] **Step 2: Commit scaffolding config files**

```bash
git add Cargo.toml rust-toolchain.toml mise.toml
git commit -m "chore: add Cargo.toml, toolchain, and mise task config"
```

### Task 3: Create Minimal App Shell

**Files:**
- Create: `src/main.rs`
- Create: `src/lib.rs`
- Create: `src/app.rs`

- [ ] **Step 1: Create src/app.rs with minimal router and HTML shell**

```rust
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { <p>"Page not found"</p> }>
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Title text="CommitBee - The commit message generator that actually understands your code"/>
        <h1>"CommitBee"</h1>
        <p>"The commit message generator that actually understands your code."</p>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body class="bg-surface text-bark antialiased">
                <App/>
            </body>
        </html>
    }
}
```

- [ ] **Step 2: Create src/lib.rs with module exports and hydrate entry**

```rust
pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
```

- [ ] **Step 3: Create src/main.rs with Axum server**

```rust
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use commitbee_web::app::{shell, App};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}
```

- [ ] **Step 4: Create a stub build.rs so the project compiles**

```rust
fn main() {
    // Content pipeline will be implemented in Task 9.
    // For now, just ensure the build succeeds.
}
```

- [ ] **Step 5: Create stub Tailwind files so cargo-leptos doesn't error**

Create `style/tailwind.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

Create `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
```

Create `public/.gitkeep` (empty file for the assets directory).

### Task 4: Verify Build

- [ ] **Step 1: Install cargo-leptos if not present**

Run: `cargo install cargo-leptos`

- [ ] **Step 2: Build the project**

Run: `cargo leptos build`

Expected: Successful compilation of both server and client targets. If there are dependency resolution issues, resolve them by adjusting version constraints in Cargo.toml.

- [ ] **Step 3: Run the dev server briefly to verify**

Run: `cargo leptos watch` (then Ctrl+C after confirming it starts)

Expected: Server starts on `http://127.0.0.1:3000`, serving the minimal "CommitBee" page.

- [ ] **Step 4: Commit**

```bash
git add src/ build.rs style/ tailwind.config.js public/.gitkeep
git commit -m "feat: add minimal Leptos app shell with Axum SSR and islands hydration"
```

## Chunk 2: Design System Foundation

### Task 5: Tailwind Config with Bee Theme

**Files:**
- Modify: `tailwind.config.js`
- Modify: `style/tailwind.css`

- [ ] **Step 1: Update tailwind.config.js with bee color palette and font families**

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        honey: {
          DEFAULT: "var(--honey)",
          light: "var(--honey-light)",
          dark: "var(--honey-dark)",
        },
        nectar: "var(--nectar)",
        comb: "var(--comb)",
        bark: "var(--bark)",
        pollen: "var(--pollen)",
        surface: {
          DEFAULT: "var(--surface)",
          raised: "var(--surface-raised)",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "ui-monospace", "monospace"],
      },
      maxWidth: {
        prose: "75ch",
      },
    },
  },
  plugins: [],
};
```

- [ ] **Step 2: Update style/tailwind.css with CSS custom properties for light/dark modes**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --honey: #f59e0b;
    --honey-light: #fcd34d;
    --honey-dark: #d97706;
    --nectar: #fffbeb;
    --comb: #78716c;
    --bark: #1c1917;
    --pollen: #fef3c7;
    --surface: #ffffff;
    --surface-raised: #fafaf9;
  }

  .dark {
    --honey: #fbbf24;
    --honey-light: #fcd34d;
    --honey-dark: #f59e0b;
    --nectar: #1c1917;
    --comb: #a8a29e;
    --bark: #fafaf9;
    --pollen: #292524;
    --surface: #0c0a09;
    --surface-raised: #1c1917;
  }

  html {
    scroll-behavior: smooth;
  }

  @media (prefers-reduced-motion: reduce) {
    html {
      scroll-behavior: auto;
    }

    *,
    *::before,
    *::after {
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
      transition-duration: 0.01ms !important;
    }
  }

  body {
    font-family: "Inter", system-ui, sans-serif;
    background-color: var(--surface);
    color: var(--bark);
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  code,
  pre,
  kbd {
    font-family: "JetBrains Mono", ui-monospace, monospace;
  }
}
```

- [ ] **Step 3: Commit**

```bash
git add tailwind.config.js style/tailwind.css
git commit -m "feat(design): add bee-themed Tailwind config with light/dark CSS custom properties"
```

### Task 6: Self-Hosted Fonts

**Files:**
- Create: `public/fonts/` (font files)

- [ ] **Step 1: Download Inter and JetBrains Mono font files**

Download Inter (variable weight, woff2):
- `Inter-Regular.woff2` (400)
- `Inter-Medium.woff2` (500)
- `Inter-SemiBold.woff2` (600)
- `Inter-Bold.woff2` (700)

Download JetBrains Mono (woff2):
- `JetBrainsMono-Regular.woff2` (400)

Place all files in `public/fonts/`.

Alternative: use the Inter variable font (`Inter-Variable.woff2`) for smaller payload.

Run:

```bash
mkdir -p public/fonts
# Download fonts from their official releases
# Inter: https://github.com/rsms/inter/releases
# JetBrains Mono: https://github.com/JetBrains/JetBrainsMono/releases
# Place .woff2 files in public/fonts/
```

- [ ] **Step 2: Add @font-face declarations to tailwind.css**

Add before `@tailwind base;`:

```css
@font-face {
  font-family: "Inter";
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url("/fonts/Inter-Regular.woff2") format("woff2");
}

@font-face {
  font-family: "Inter";
  font-style: normal;
  font-weight: 500;
  font-display: swap;
  src: url("/fonts/Inter-Medium.woff2") format("woff2");
}

@font-face {
  font-family: "Inter";
  font-style: normal;
  font-weight: 600;
  font-display: swap;
  src: url("/fonts/Inter-SemiBold.woff2") format("woff2");
}

@font-face {
  font-family: "Inter";
  font-style: normal;
  font-weight: 700;
  font-display: swap;
  src: url("/fonts/Inter-Bold.woff2") format("woff2");
}

@font-face {
  font-family: "JetBrains Mono";
  font-style: normal;
  font-weight: 400;
  font-display: swap;
  src: url("/fonts/JetBrainsMono-Regular.woff2") format("woff2");
}
```

- [ ] **Step 3: Commit**

```bash
git add public/fonts/ style/tailwind.css
git commit -m "feat(design): add self-hosted Inter and JetBrains Mono fonts"
```

### Task 7: Animation CSS

**Files:**
- Create: `style/animations.css`

- [ ] **Step 1: Create style/animations.css with scroll-driven keyframes**

```css
/* Scroll reveal animations — baseline via IntersectionObserver (JS island),
   progressive enhancement via CSS scroll-timeline for Chromium */

.reveal {
  opacity: 0;
  transform: translateY(24px);
  transition: opacity 0.6s ease-out, transform 0.6s ease-out;
}

.reveal.visible {
  opacity: 1;
  transform: translateY(0);
}

/* Stagger children */
.reveal-stagger > .reveal:nth-child(1) { transition-delay: 0ms; }
.reveal-stagger > .reveal:nth-child(2) { transition-delay: 100ms; }
.reveal-stagger > .reveal:nth-child(3) { transition-delay: 200ms; }
.reveal-stagger > .reveal:nth-child(4) { transition-delay: 400ms; }
.reveal-stagger > .reveal:nth-child(5) { transition-delay: 500ms; }
.reveal-stagger > .reveal:nth-child(6) { transition-delay: 600ms; }
.reveal-stagger > .reveal:nth-child(7) { transition-delay: 700ms; }

/* Slide from left */
.reveal-left {
  opacity: 0;
  transform: translateX(-32px);
  transition: opacity 0.6s ease-out, transform 0.6s ease-out;
}

.reveal-left.visible {
  opacity: 1;
  transform: translateX(0);
}

/* Slide from right */
.reveal-right {
  opacity: 0;
  transform: translateX(32px);
  transition: opacity 0.6s ease-out, transform 0.6s ease-out;
}

.reveal-right.visible {
  opacity: 1;
  transform: translateX(0);
}

/* Scale up */
.reveal-scale {
  opacity: 0;
  transform: scale(0.95);
  transition: opacity 0.6s ease-out, transform 0.6s ease-out;
}

.reveal-scale.visible {
  opacity: 1;
  transform: scale(1);
}

/* Honeycomb pulse background */
@keyframes hex-pulse {
  0%, 100% { opacity: 0.03; }
  50% { opacity: 0.08; }
}

.hex-bg {
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='28' height='49' viewBox='0 0 28 49'%3E%3Cg fill-rule='evenodd'%3E%3Cg fill='%23f59e0b' fill-opacity='0.06'%3E%3Cpath d='M13.99 9.25l13 7.5v15l-13 7.5L1 31.75v-15l12.99-7.5zM3 17.9v12.7l10.99 6.34 11-6.35V17.9l-11-6.34L3 17.9zM0 15l12.98-7.5V0h-2v6.35L0 12.69v2.3zm0 18.5L12.98 41v8h-2v-6.85L0 35.81v-2.3zM15 0v7.5L27.99 15H28v-2.31h-.01L17 6.35V0h-2zm0 49v-8l12.99-7.5H28v2.31h-.01L17 42.15V49h-2z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
  animation: hex-pulse 4s ease-in-out infinite;
}

/* Progressive enhancement: CSS scroll-driven animations (Chromium) */
@supports (animation-timeline: scroll()) {
  .reveal-scroll {
    animation: fade-up linear both;
    animation-timeline: view();
    animation-range: entry 0% entry 100%;
  }

  @keyframes fade-up {
    from {
      opacity: 0;
      transform: translateY(24px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
}

/* Pipeline demo step transitions */
.pipeline-step {
  opacity: 0;
  transform: translateX(-16px);
  transition: opacity 0.4s ease-out, transform 0.4s ease-out;
}

.pipeline-step.active {
  opacity: 1;
  transform: translateX(0);
}

/* Terminal typing animation */
@keyframes blink-caret {
  0%, 100% { border-color: transparent; }
  50% { border-color: var(--honey); }
}

.terminal-cursor {
  border-right: 2px solid var(--honey);
  animation: blink-caret 1s step-end infinite;
}

/* Reduced motion: override all animations */
@media (prefers-reduced-motion: reduce) {
  .reveal,
  .reveal-left,
  .reveal-right,
  .reveal-scale {
    opacity: 1;
    transform: none;
    transition: none;
  }

  .pipeline-step {
    opacity: 1;
    transform: none;
    transition: none;
  }

  .hex-bg {
    animation: none;
    opacity: 0.05;
  }

  .terminal-cursor {
    animation: none;
    border-color: var(--honey);
  }
}
```

- [ ] **Step 2: Import animations.css from tailwind.css**

Add at end of `style/tailwind.css`:

```css
@import "./animations.css";
```

- [ ] **Step 3: Verify build still compiles with new CSS**

Run: `cargo leptos build`

Expected: Successful build. Tailwind processes both files.

- [ ] **Step 4: Commit**

```bash
git add style/animations.css style/tailwind.css
git commit -m "feat(design): add scroll-reveal animations with reduced-motion and hex-bg pattern"
```

## Chunk 3: Content Pipeline

### Task 8: Create Markdown Doc Stubs

**Files:**
- Create: `content/docs/*.md` (12 files)

Each doc file uses YAML frontmatter with `title`, `order`, `section`, `description`. URL slug is derived from filename.

- [ ] **Step 1: Create content/docs/ directory and all 12 markdown files**

Every file follows this pattern:

```markdown
---
title: "Page Title"
order: N
section: "Section Name"
description: "Brief description for meta tags"
---

# Page Title

Content placeholder. This will be populated from commitbee DOCS.md.
```

Create these files:

`content/docs/getting-started.md`:
```markdown
---
title: "Getting Started"
order: 1
section: "Basics"
description: "Install commitbee and generate your first commit message"
---

# Getting Started

## Installation

### From source

```bash
cargo install commitbee
```

### Requirements

- **Rust** 1.94+ (edition 2024)
- **Ollama** running locally (default provider)
- A model pulled in Ollama (recommended: `qwen3.5:4b`)

```bash
ollama pull qwen3.5:4b
```

## Quick Start

```bash
# Stage your changes
git add src/feature.rs

# Generate and commit interactively
commitbee

# Preview without committing
commitbee --dry-run

# Auto-confirm and commit
commitbee --yes
```

That's it. CommitBee works with zero configuration if Ollama is running locally.
```

`content/docs/how-it-works.md`:
```markdown
---
title: "How It Works"
order: 2
section: "Basics"
description: "Understand commitbee's 7-stage pipeline from diff to commit message"
---

# How It Works

CommitBee follows a 7-stage pipeline to generate commit messages.

## The Pipeline

1. **Git Service** - Reads staged changes via `git diff --cached`
2. **Tree-sitter Analyzer** - Parses code, extracts symbols, maps hunks to spans
3. **Split Detector** - Groups files by change shape and content vocabulary
4. **Context Builder** - Assembles evidence flags, applies token budget
5. **LLM Provider** - Generates commit message with streaming output
6. **Validator** - Checks 7 rules, retries up to 3 times with corrections
7. **Sanitizer** - Strips artifacts, wraps body at 72 chars
```

`content/docs/commands-and-flags.md`:
```markdown
---
title: "Commands & Flags"
order: 1
section: "Usage"
description: "Complete reference for commitbee CLI commands and flags"
---

# Commands & Flags

## Options

| Flag | Description |
| --- | --- |
| `--dry-run` | Print message only, don't commit |
| `--yes` | Auto-confirm and commit |
| `-n, --generate` | Generate N candidates (1-5, default 1) |
| `--no-split` | Disable commit split suggestions |
| `--no-scope` | Disable scope in commit messages |
| `--clipboard` | Copy message to clipboard instead of committing |
| `--exclude <GLOB>` | Exclude files matching glob pattern (repeatable) |
| `--allow-secrets` | Allow committing with detected secrets |
| `--verbose` | Show symbol extraction details |
| `--show-prompt` | Debug: display the full LLM prompt |

## Commands

| Command | Description |
| --- | --- |
| `init` | Create a config file |
| `config` | Show current configuration |
| `doctor` | Check configuration and connectivity |
| `completions <shell>` | Generate shell completions |
| `hook install` | Install prepare-commit-msg hook |
| `hook uninstall` | Remove prepare-commit-msg hook |
| `hook status` | Check if hook is installed |
```

`content/docs/configuration.md`:
```markdown
---
title: "Configuration"
order: 2
section: "Usage"
description: "Configure commitbee with TOML files, environment variables, and CLI flags"
---

# Configuration

CommitBee uses a 5-level configuration system:

1. Built-in defaults
2. Project `.commitbee.toml`
3. User config (`~/.config/commitbee/config.toml`)
4. Environment variables
5. CLI flags (highest priority)

Run `commitbee init` to create a config file interactively.
```

`content/docs/llm-providers.md`:
```markdown
---
title: "LLM Providers"
order: 3
section: "Usage"
description: "Configure Ollama, OpenAI, and Anthropic providers for commit message generation"
---

# LLM Providers

CommitBee supports three LLM providers:

## Ollama (Default)

Local-first. Your code never leaves your machine.

```toml
[provider]
name = "ollama"
model = "qwen3.5:4b"
```

## OpenAI

```toml
[provider]
name = "openai"
model = "gpt-4o-mini"
api_key_env = "OPENAI_API_KEY"
```

## Anthropic

```toml
[provider]
name = "anthropic"
model = "claude-sonnet-4-20250514"
api_key_env = "ANTHROPIC_API_KEY"
```
```

`content/docs/commit-splitting.md`:
```markdown
---
title: "Commit Splitting"
order: 4
section: "Usage"
description: "Automatic detection and splitting of multi-concern staged changes"
---

# Commit Splitting

When your staged changes mix independent work, CommitBee detects it and offers to split them into separate commits.

The splitter uses diff-shape fingerprinting combined with Jaccard similarity on content vocabulary.
```

`content/docs/validation-pipeline.md`:
```markdown
---
title: "Validation Pipeline"
order: 1
section: "Internals"
description: "The 7-rule validation pipeline that ensures commit message quality"
---

# Validation Pipeline

Every generated message passes through a 7-rule validation pipeline:

1. **Fix requires evidence** - no bug comments, no `fix` type
2. **Breaking change detection** - removed public APIs must be flagged
3. **Anti-hallucination** - breaking change text can't copy internal field names
4. **Mechanical changes** must use `style`
5. **Dependency-only changes** must use `chore`
6. **Subject specificity** - rejects generic messages like "update code"
7. **Subject length** - enforces the 72-character first line limit
```

`content/docs/security-and-safety.md`:
```markdown
---
title: "Security & Safety"
order: 2
section: "Internals"
description: "Secret scanning with 25 built-in patterns across 13 categories"
---

# Security & Safety

CommitBee scans all content before sending to any LLM provider with 25 built-in patterns across 13 categories.

## Categories

- Cloud providers (AWS, GCP, Azure)
- AI/ML (OpenAI, Anthropic, HuggingFace)
- Source control (GitHub, GitLab)
- Communication (Slack, Discord)
- Payment & SaaS (Stripe, Twilio, SendGrid)
- Database (MongoDB, PostgreSQL, MySQL, Redis)
- Cryptographic (PEM keys, JWT)
- Generic (API key assignments)
- Merge conflict detection
```

`content/docs/supported-languages.md`:
```markdown
---
title: "Supported Languages"
order: 3
section: "Internals"
description: "10 languages supported by tree-sitter semantic analysis"
---

# Supported Languages

CommitBee uses tree-sitter to parse these languages:

**Rust, TypeScript, JavaScript, Python, Go, Java, C, C++, Ruby, C#**

All enabled by default, individually toggleable via Cargo feature flags. Files in other languages still get full diff context, just without symbol extraction.
```

`content/docs/git-hooks.md`:
```markdown
---
title: "Git Hooks"
order: 1
section: "Integration"
description: "Integrate commitbee with git prepare-commit-msg hook"
---

# Git Hooks

CommitBee integrates with git's `prepare-commit-msg` hook.

```bash
# Install the hook
commitbee hook install

# Check status
commitbee hook status

# Remove the hook
commitbee hook uninstall
```

The hook has TTY detection for safe non-interactive fallback.
```

`content/docs/troubleshooting.md`:
```markdown
---
title: "Troubleshooting"
order: 2
section: "Integration"
description: "Common issues and their solutions"
---

# Troubleshooting

## Common Issues

### Ollama not running

Run `commitbee doctor` to check connectivity and model availability.

### Empty commit message

Check that you have staged changes: `git diff --cached --stat`

### Model not found

Pull the model first: `ollama pull qwen3.5:4b`
```

`content/docs/architecture.md`:
```markdown
---
title: "Architecture"
order: 1
section: "Reference"
description: "Deep dive into commitbee's internal architecture and design decisions"
---

# Architecture

CommitBee is ~18K lines of Rust compiled to a single static binary with LTO.

## Core Pipeline

```
Stage -> Git Service -> Tree-sitter Analyzer -> Split Detector
      -> Context Builder -> LLM Provider -> Validator -> Sanitizer
```

## Testing

308 tests: unit, snapshot (insta), property (proptest), integration (wiremock).
```

- [ ] **Step 2: Verify all 12 files exist**

Run: `ls content/docs/*.md | wc -l`

Expected: `12`

- [ ] **Step 3: Commit**

```bash
git add content/docs/
git commit -m "docs: add 12 markdown documentation pages with frontmatter"
```

### Task 9: Build Script (build.rs)

**Files:**
- Modify: `build.rs`

This is the core content pipeline. It reads markdown, renders HTML, builds TOC, generates search index, and emits Rust source.

- [ ] **Step 1: Write the full build.rs content pipeline**

```rust
use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: String,
    order: u32,
    section: String,
    description: String,
}

struct Heading {
    level: u8,
    text: String,
    id: String,
}

struct DocPage {
    slug: String,
    title: String,
    order: u32,
    section: String,
    description: String,
    html_content: String,
    headings: Vec<Heading>,
    word_excerpt: String,
}

fn main() {
    println!("cargo::rerun-if-changed=content/docs/");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let content_dir = Path::new("content/docs");

    if !content_dir.exists() {
        // No content yet, generate empty stubs
        generate_empty_module(&out_dir);
        return;
    }

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let mut pages: Vec<DocPage> = Vec::new();
    let mut routes = vec!["/".to_string()];

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "md")
        })
    {
        let raw = fs::read_to_string(entry.path()).unwrap_or_else(|e| {
            panic!("Failed to read {}: {e}", entry.path().display())
        });

        let (fm, markdown) = parse_frontmatter(&raw);
        let slug = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let headings = extract_headings(&markdown);
        let html_content = render_markdown_with_syntax_highlighting(&markdown, &ss, theme);
        let word_excerpt = extract_excerpt(&markdown, 200);

        routes.push(format!("/docs/{slug}"));

        pages.push(DocPage {
            slug,
            title: fm.title,
            order: fm.order,
            section: fm.section,
            description: fm.description,
            html_content,
            headings,
            word_excerpt,
        });
    }

    // Sort by logical section order (not alphabetical), then by order within section
    let section_order = ["Basics", "Usage", "Internals", "Integration", "Reference"];
    let section_rank = |s: &str| -> usize {
        section_order.iter().position(|&x| x == s).unwrap_or(usize::MAX)
    };
    pages.sort_by(|a, b| {
        section_rank(&a.section)
            .cmp(&section_rank(&b.section))
            .then(a.order.cmp(&b.order))
    });

    generate_rust_module(&pages, &out_dir);
    generate_search_index(&pages, &out_dir);

    // Routes manifest for pre-rendering
    fs::write(
        Path::new(&out_dir).join("routes.txt"),
        routes.join("\n"),
    )
    .unwrap();
}

fn parse_frontmatter(content: &str) -> (Frontmatter, String) {
    let content = content.trim_start();
    if !content.starts_with("---") {
        panic!("Missing YAML frontmatter delimiter");
    }
    let after_first = &content[3..];
    let end = after_first
        .find("---")
        .expect("Missing closing frontmatter delimiter");
    let yaml = &after_first[..end];
    let markdown = &after_first[end + 3..];

    let fm: Frontmatter =
        serde_yaml::from_str(yaml).expect("Invalid frontmatter YAML");

    (fm, markdown.trim().to_string())
}

fn extract_headings(markdown: &str) -> Vec<Heading> {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut headings = Vec::new();
    let mut current_level: Option<u8> = None;
    let mut current_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current_level = Some(level as u8);
                current_text.clear();
            }
            Event::Text(text) if current_level.is_some() => {
                current_text.push_str(&text);
            }
            Event::Code(code) if current_level.is_some() => {
                current_text.push_str(&code);
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(level) = current_level.take() {
                    let id = slug::slugify(&current_text);
                    headings.push(Heading {
                        level,
                        text: current_text.clone(),
                        id,
                    });
                }
            }
            _ => {}
        }
    }

    headings
}

fn render_markdown_with_syntax_highlighting(
    markdown: &str,
    ss: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut events: Vec<Event> = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_content.clear();
                code_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                        lang.to_string()
                    }
                    _ => String::new(),
                };
            }
            Event::Text(text) if in_code_block => {
                code_content.push_str(&text);
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;

                let lang_display = if code_lang.is_empty() {
                    "text"
                } else {
                    &code_lang
                };

                let highlighted = if let Some(syntax) =
                    ss.find_syntax_by_token(&code_lang)
                {
                    highlighted_html_for_string(
                        &code_content,
                        ss,
                        syntax,
                        theme,
                    )
                    .unwrap_or_else(|_| html_escape(&code_content))
                } else {
                    html_escape(&code_content)
                };

                let html = format!(
                    r#"<div class="code-block-wrapper relative group" data-lang="{lang_display}"><div class="code-block-header flex items-center justify-between px-4 py-2 text-xs text-comb bg-pollen rounded-t-lg border-b border-honey/10"><span>{lang_display}</span><button class="copy-btn opacity-0 group-hover:opacity-100 transition-opacity text-comb hover:text-honey" data-code="{escaped}">Copy</button></div><pre class="!rounded-t-none"><code>{highlighted}</code></pre></div>"#,
                    escaped = html_escape(&code_content)
                );

                events.push(Event::Html(html.into()));
                continue;
            }
            Event::Start(Tag::Heading { level, .. }) => {
                // Add id attribute to headings for anchor links
                // We'll collect the text first, then emit with id
                events.push(event);
                continue;
            }
            _ => {}
        }

        if !in_code_block {
            events.push(event);
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());

    // Post-process: add IDs to heading tags for TOC anchor links
    add_heading_ids(&html_output)
}

fn add_heading_ids(html: &str) -> String {
    let mut result = html.to_string();
    for level in 1..=6 {
        let open_tag = format!("<h{level}>");
        let close_tag = format!("</h{level}>");
        let mut search_from = 0;
        let mut new_result = String::new();

        while let Some(start) = result[search_from..].find(&open_tag) {
            let abs_start = search_from + start;
            let content_start = abs_start + open_tag.len();
            if let Some(end) = result[content_start..].find(&close_tag) {
                let abs_end = content_start + end;
                let text = &result[content_start..abs_end];
                let plain = strip_html_tags(text);
                let id = slug::slugify(&plain);

                new_result.push_str(&result[search_from..abs_start]);
                write!(
                    new_result,
                    "<h{level} id=\"{id}\">{text}{close_tag}"
                )
                .unwrap();
                search_from = abs_end + close_tag.len();
            } else {
                break;
            }
        }
        new_result.push_str(&result[search_from..]);
        result = new_result;
    }
    result
}

fn strip_html_tags(s: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn extract_excerpt(markdown: &str, max_words: usize) -> String {
    let parser = Parser::new(markdown);
    let mut words = Vec::new();

    for event in parser {
        if let Event::Text(text) = event {
            words.extend(text.split_whitespace().map(String::from));
            if words.len() >= max_words {
                break;
            }
        }
    }

    words.truncate(max_words);
    words.join(" ")
}

fn generate_rust_module(pages: &[DocPage], out_dir: &str) {
    let mut code = String::new();

    // Section order definition
    writeln!(code, "pub const SECTION_ORDER: &[&str] = &[\"Basics\", \"Usage\", \"Internals\", \"Integration\", \"Reference\"];").unwrap();
    writeln!(code).unwrap();

    // Doc page struct
    writeln!(code, "#[derive(Debug, Clone)]").unwrap();
    writeln!(code, "pub struct DocPageData {{").unwrap();
    writeln!(code, "    pub slug: &'static str,").unwrap();
    writeln!(code, "    pub title: &'static str,").unwrap();
    writeln!(code, "    pub section: &'static str,").unwrap();
    writeln!(code, "    pub description: &'static str,").unwrap();
    writeln!(code, "    pub order: u32,").unwrap();
    writeln!(code, "    pub html_content: &'static str,").unwrap();
    writeln!(code, "    pub headings: &'static [(u8, &'static str, &'static str)],").unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    // Generate static arrays for each page
    for (i, page) in pages.iter().enumerate() {
        // Headings array
        write!(code, "static HEADINGS_{i}: &[(u8, &str, &str)] = &[").unwrap();
        for h in &page.headings {
            write!(
                code,
                "({}, \"{}\", \"{}\"),",
                h.level,
                h.text.replace('"', "\\\""),
                h.id
            )
            .unwrap();
        }
        writeln!(code, "];").unwrap();
    }
    writeln!(code).unwrap();

    // Pages array
    writeln!(code, "pub static PAGES: &[DocPageData] = &[").unwrap();
    for (i, page) in pages.iter().enumerate() {
        writeln!(code, "    DocPageData {{").unwrap();
        writeln!(code, "        slug: \"{}\",", page.slug).unwrap();
        writeln!(
            code,
            "        title: \"{}\",",
            page.title.replace('"', "\\\"")
        )
        .unwrap();
        writeln!(code, "        section: \"{}\",", page.section).unwrap();
        writeln!(
            code,
            "        description: \"{}\",",
            page.description.replace('"', "\\\"")
        )
        .unwrap();
        writeln!(code, "        order: {},", page.order).unwrap();
        writeln!(
            code,
            "        html_content: r##\"{}\"##,",
            page.html_content
        )
        .unwrap();
        writeln!(code, "        headings: HEADINGS_{i},").unwrap();
        writeln!(code, "    }},").unwrap();
    }
    writeln!(code, "];").unwrap();
    writeln!(code).unwrap();

    // Helper functions
    writeln!(code, "pub fn get_page(slug: &str) -> Option<&'static DocPageData> {{").unwrap();
    writeln!(code, "    PAGES.iter().find(|p| p.slug == slug)").unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    writeln!(code, "pub fn get_pages_by_section(section: &str) -> Vec<&'static DocPageData> {{").unwrap();
    writeln!(code, "    PAGES.iter().filter(|p| p.section == section).collect()").unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    writeln!(code, "pub fn get_adjacent(slug: &str) -> (Option<&'static DocPageData>, Option<&'static DocPageData>) {{").unwrap();
    writeln!(code, "    let idx = PAGES.iter().position(|p| p.slug == slug);").unwrap();
    writeln!(code, "    match idx {{").unwrap();
    writeln!(code, "        Some(i) => (").unwrap();
    writeln!(code, "            if i > 0 {{ Some(&PAGES[i - 1]) }} else {{ None }},").unwrap();
    writeln!(code, "            PAGES.get(i + 1),").unwrap();
    writeln!(code, "        ),").unwrap();
    writeln!(code, "        None => (None, None),").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}").unwrap();

    fs::write(
        Path::new(out_dir).join("content_generated.rs"),
        code,
    )
    .unwrap();
}

fn generate_search_index(pages: &[DocPage], out_dir: &str) {
    let entries: Vec<serde_json::Value> = pages
        .iter()
        .map(|p| {
            let heading_texts: Vec<&str> =
                p.headings.iter().map(|h| h.text.as_str()).collect();
            serde_json::json!({
                "slug": p.slug,
                "title": p.title,
                "section": p.section,
                "headings": heading_texts,
                "excerpt": p.word_excerpt,
            })
        })
        .collect();

    let json = serde_json::to_string(&entries).unwrap();

    // Write to OUT_DIR for build artifacts
    fs::write(
        Path::new(out_dir).join("search_index.json"),
        &json,
    )
    .unwrap();

    // Write to target/site/ so cargo-leptos serves it as a static asset.
    // Do NOT write to public/ — that pollutes the source tree and shows as
    // uncommitted changes in git. target/site/ is the served root.
    let site_dir = Path::new("target/site");
    if site_dir.exists() {
        fs::write(site_dir.join("search_index.json"), &json).unwrap();
    }
}

fn generate_empty_module(out_dir: &str) {
    let code = r#"
pub const SECTION_ORDER: &[&str] = &[];

#[derive(Debug, Clone)]
pub struct DocPageData {
    pub slug: &'static str,
    pub title: &'static str,
    pub section: &'static str,
    pub description: &'static str,
    pub order: u32,
    pub html_content: &'static str,
    pub headings: &'static [(u8, &'static str, &'static str)],
}

pub static PAGES: &[DocPageData] = &[];

pub fn get_page(_slug: &str) -> Option<&'static DocPageData> { None }
pub fn get_pages_by_section(_section: &str) -> Vec<&'static DocPageData> { vec![] }
pub fn get_adjacent(_slug: &str) -> (Option<&'static DocPageData>, Option<&'static DocPageData>) { (None, None) }
"#;

    fs::write(
        Path::new(out_dir).join("content_generated.rs"),
        code,
    )
    .unwrap();
    fs::write(
        Path::new(out_dir).join("routes.txt"),
        "/\n",
    )
    .unwrap();
    fs::write(
        Path::new(out_dir).join("search_index.json"),
        "[]",
    )
    .unwrap();
}
```

- [ ] **Step 2: Verify build.rs compiles and generates output**

Run: `cargo leptos build 2>&1 | tail -5`

Expected: Build succeeds. Check that `target/*/build/commitbee-web-*/out/content_generated.rs` exists.

- [ ] **Step 3: Commit**

```bash
git add build.rs
git commit -m "feat(content): add build.rs markdown pipeline with syntax highlighting and search index"
```

### Task 10: Content Loader Module

**Files:**
- Create: `src/content/mod.rs`
- Create: `src/content/loader.rs`

- [ ] **Step 1: Create src/content/mod.rs**

```rust
pub mod loader;
```

- [ ] **Step 2: Create src/content/loader.rs that includes generated content**

```rust
include!(concat!(env!("OUT_DIR"), "/content_generated.rs"));

/// Sections in display order with their pages
pub fn doc_tree() -> Vec<(&'static str, Vec<&'static DocPageData>)> {
    SECTION_ORDER
        .iter()
        .filter_map(|&section| {
            let pages = get_pages_by_section(section);
            if pages.is_empty() {
                None
            } else {
                Some((section, pages))
            }
        })
        .collect()
}
```

- [ ] **Step 3: Add content module to lib.rs**

Add to `src/lib.rs`:

```rust
pub mod content;
```

- [ ] **Step 4: Verify build**

Run: `cargo leptos build`

Expected: Compiles successfully with the content module included.

- [ ] **Step 5: Commit**

```bash
git add src/content/
git commit -m "feat(content): add loader module that exposes generated doc content"
```

### Task 11: Content Validation Test

**Files:**
- Create: `tests/content_check.rs`

- [ ] **Step 1: Write content validation test**

```rust
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct Frontmatter {
    title: String,
    order: u32,
    section: String,
    description: String,
}

fn parse_frontmatter(content: &str) -> Result<Frontmatter, String> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return Err("Missing opening ---".to_string());
    }
    let after_first = &content[3..];
    let end = after_first
        .find("---")
        .ok_or("Missing closing ---")?;
    let yaml = &after_first[..end];
    serde_yaml::from_str(yaml).map_err(|e| e.to_string())
}

#[test]
fn all_docs_have_valid_frontmatter() {
    let content_dir = Path::new("content/docs");
    assert!(
        content_dir.exists(),
        "content/docs/ directory does not exist"
    );

    let mut count = 0;
    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "md")
        })
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        let result = parse_frontmatter(&raw);
        assert!(
            result.is_ok(),
            "Invalid frontmatter in {}: {}",
            entry.path().display(),
            result.unwrap_err()
        );

        let fm = result.unwrap();
        assert!(!fm.title.is_empty(), "Empty title in {}", entry.path().display());
        assert!(!fm.section.is_empty(), "Empty section in {}", entry.path().display());
        assert!(!fm.description.is_empty(), "Empty description in {}", entry.path().display());

        count += 1;
    }

    assert!(count >= 12, "Expected at least 12 doc files, found {count}");
}

#[test]
fn no_duplicate_slugs() {
    let content_dir = Path::new("content/docs");
    let mut slugs = HashSet::new();

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "md")
        })
    {
        let slug = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        assert!(
            slugs.insert(slug.clone()),
            "Duplicate slug: {slug}"
        );
    }
}

#[test]
fn sections_are_recognized() {
    let valid_sections: HashSet<&str> =
        ["Basics", "Usage", "Internals", "Integration", "Reference"]
            .into_iter()
            .collect();

    let content_dir = Path::new("content/docs");

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "md")
        })
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        let fm = parse_frontmatter(&raw).unwrap();
        assert!(
            valid_sections.contains(fm.section.as_str()),
            "Unknown section '{}' in {}",
            fm.section,
            entry.path().display()
        );
    }
}
```

- [ ] **Step 2: Run the content validation tests**

Run: `cargo test --test content_check`

Expected: All 3 tests pass.

- [ ] **Step 3: Commit**

```bash
git add tests/content_check.rs
git commit -m "test: add content validation tests for frontmatter schema and slugs"
```

## Chunk 4: Core Components

### Task 12: Component Module Structure

**Files:**
- Create: `src/components/mod.rs`
- Create: `src/pages/mod.rs`

- [ ] **Step 1: Create src/components/mod.rs**

```rust
pub mod nav;
pub mod footer;
pub mod scroll_reveal;
pub mod doc_sidebar;
pub mod doc_toc;
pub mod theme_toggle;
pub mod code_block;
pub mod doc_search;
pub mod pipeline_demo;
```

- [ ] **Step 2: Create src/pages/mod.rs**

```rust
pub mod landing;
pub mod docs;
pub mod not_found;
```

- [ ] **Step 3: Add modules to lib.rs**

Update `src/lib.rs`:

```rust
pub mod app;
pub mod components;
pub mod content;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
```

### Task 13: Navigation Component

**Files:**
- Create: `src/components/nav.rs`

- [ ] **Step 1: Create nav.rs with sticky navigation header**

```rust
use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 w-full border-b border-honey/10 bg-surface/80 backdrop-blur-lg transition-colors">
            <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
                // Logo
                <a href="/" class="flex items-center gap-2 font-bold text-lg text-bark hover:text-honey transition-colors">
                    <span class="text-2xl">"🐝"</span>
                    <span>"CommitBee"</span>
                </a>

                // Desktop nav links
                <div class="hidden md:flex items-center gap-6">
                    <a href="/" class="text-sm font-medium text-comb hover:text-bark transition-colors">"Home"</a>
                    <a href="/docs/getting-started" class="text-sm font-medium text-comb hover:text-bark transition-colors">"Docs"</a>
                    <a
                        href="https://github.com/sephyi/commitbee"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm font-medium text-comb hover:text-bark transition-colors"
                    >
                        "GitHub"
                    </a>
                    <a
                        href="https://crates.io/crates/commitbee"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm font-medium text-comb hover:text-bark transition-colors"
                    >
                        "crates.io"
                    </a>
                </div>

                // Right side: theme toggle + mobile menu
                <div class="flex items-center gap-3">
                    <super::theme_toggle::ThemeToggle/>

                    // Mobile hamburger button — toggles via island
                    <button
                        class="md:hidden p-2 text-comb hover:text-bark"
                        aria-label="Toggle menu"
                        id="mobile-menu-toggle"
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                        </svg>
                    </button>
                </div>
            </div>
        </nav>
    }
}
```

### Task 14: Footer Component

**Files:**
- Create: `src/components/footer.rs`

- [ ] **Step 1: Create footer.rs with bee-themed footer**

```rust
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="relative border-t border-honey/10 bg-surface-raised">
            // Subtle hex background
            <div class="absolute inset-0 hex-bg pointer-events-none"/>

            <div class="relative mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 gap-8 md:grid-cols-3">
                    // Brand
                    <div>
                        <div class="flex items-center gap-2 font-bold text-lg text-bark">
                            <span class="text-2xl">"🐝"</span>
                            <span>"CommitBee"</span>
                        </div>
                        <p class="mt-2 text-sm text-comb">
                            "The commit message generator that actually understands your code."
                        </p>
                    </div>

                    // Links
                    <div>
                        <h3 class="text-sm font-semibold text-bark">"Links"</h3>
                        <ul class="mt-3 space-y-2">
                            <li>
                                <a href="/docs/getting-started" class="text-sm text-comb hover:text-honey transition-colors">"Documentation"</a>
                            </li>
                            <li>
                                <a href="https://github.com/sephyi/commitbee" target="_blank" rel="noopener noreferrer" class="text-sm text-comb hover:text-honey transition-colors">"GitHub"</a>
                            </li>
                            <li>
                                <a href="https://crates.io/crates/commitbee" target="_blank" rel="noopener noreferrer" class="text-sm text-comb hover:text-honey transition-colors">"crates.io"</a>
                            </li>
                        </ul>
                    </div>

                    // Meta
                    <div>
                        <h3 class="text-sm font-semibold text-bark">"Project"</h3>
                        <ul class="mt-3 space-y-2">
                            <li>
                                <a href="https://github.com/sponsors/Sephyi" target="_blank" rel="noopener noreferrer" class="text-sm text-comb hover:text-honey transition-colors">"Sponsor"</a>
                            </li>
                            <li>
                                <span class="text-sm text-comb">"License: PolyForm Noncommercial"</span>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="mt-8 border-t border-honey/10 pt-6 text-center">
                    <p class="text-xs text-comb">
                        "Made with Rust" " · "
                        "Copyright 2026 " <a href="https://sephy.io" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sephyi"</a>
                    </p>
                </div>
            </div>
        </footer>
    }
}
```

### Task 15: Scroll Reveal Component

**Files:**
- Create: `src/components/scroll_reveal.rs`

This is an `#[island]` that uses IntersectionObserver to add `.visible` class on scroll.

- [ ] **Step 1: Create scroll_reveal.rs**

```rust
use leptos::prelude::*;

/// Wraps children in a div that fades in when scrolled into view.
/// Uses IntersectionObserver as the baseline (all browsers).
/// CSS class variants: "reveal" (fade-up), "reveal-left", "reveal-right", "reveal-scale"
#[island]
pub fn ScrollReveal(
    #[prop(default = "reveal".to_string())] class: String,
    children: Children,
) -> impl IntoView {
    let el_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move || {
        if let Some(el) = el_ref.get() {
            use wasm_bindgen::prelude::*;
            use web_sys::IntersectionObserverInit;

            let callback = Closure::<dyn Fn(js_sys::Array, web_sys::IntersectionObserver)>::new(
                move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
                    for entry in entries.iter() {
                        let entry: web_sys::IntersectionObserverEntry =
                            entry.unchecked_into();
                        if entry.is_intersecting() {
                            let target = entry.target();
                            let _ = target
                                .class_list()
                                .add_1("visible");
                            observer.unobserve(&target);
                        }
                    }
                },
            );

            let mut options = IntersectionObserverInit::new();
            options.threshold(&JsValue::from_f64(0.1));

            if let Ok(observer) =
                web_sys::IntersectionObserver::new_with_options(
                    callback.as_ref().unchecked_ref(),
                    &options,
                )
            {
                observer.observe(&el);
            }

            // Leak the closure so it lives as long as the observer
            callback.forget();
        }
    });

    view! {
        <div node_ref=el_ref class=class>
            {children()}
        </div>
    }
}
```

### Task 16: Doc Sidebar Component

**Files:**
- Create: `src/components/doc_sidebar.rs`

- [ ] **Step 1: Create doc_sidebar.rs with section tree navigation**

```rust
use leptos::prelude::*;
use crate::content::loader;

#[component]
pub fn DocSidebar(
    #[prop(into)] current_slug: String,
) -> impl IntoView {
    let tree = loader::doc_tree();

    view! {
        <aside class="w-64 shrink-0 border-r border-honey/10 bg-surface-raised p-4 overflow-y-auto">
            <nav aria-label="Documentation">
                {tree
                    .into_iter()
                    .map(|(section, pages)| {
                        view! {
                            <div class="mb-6">
                                <h3 class="mb-2 text-xs font-semibold uppercase tracking-wider text-comb">
                                    {section}
                                </h3>
                                <ul class="space-y-1">
                                    {pages
                                        .into_iter()
                                        .map(|page| {
                                            let is_active = page.slug == current_slug;
                                            let link_class = if is_active {
                                                "block rounded-md px-3 py-1.5 text-sm font-medium bg-honey/10 text-honey border-l-2 border-honey"
                                            } else {
                                                "block rounded-md px-3 py-1.5 text-sm text-comb hover:text-bark hover:bg-surface transition-colors"
                                            };
                                            view! {
                                                <li>
                                                    <a href=format!("/docs/{}", page.slug) class=link_class>
                                                        {page.title}
                                                    </a>
                                                </li>
                                            }
                                        })
                                        .collect_view()
                                    }
                                </ul>
                            </div>
                        }
                    })
                    .collect_view()
                }
            </nav>
        </aside>
    }
}
```

### Task 17: Doc TOC Component

**Files:**
- Create: `src/components/doc_toc.rs`

- [ ] **Step 1: Create doc_toc.rs with right-side table of contents**

```rust
use leptos::prelude::*;

#[component]
pub fn DocToc(
    headings: &'static [(u8, &'static str, &'static str)],
) -> impl IntoView {
    // Only show h2 and h3 in the TOC
    let toc_headings: Vec<_> = headings
        .iter()
        .filter(|(level, _, _)| *level == 2 || *level == 3)
        .collect();

    if toc_headings.is_empty() {
        return view! { <div/> }.into_any();
    }

    view! {
        <aside class="hidden xl:block w-56 shrink-0 pl-8">
            <div class="sticky top-20">
                <h4 class="mb-3 text-xs font-semibold uppercase tracking-wider text-comb">"On this page"</h4>
                <ul class="space-y-1 border-l border-honey/10">
                    {toc_headings
                        .into_iter()
                        .map(|(level, text, id)| {
                            let indent = if *level == 3 { "pl-6" } else { "pl-3" };
                            view! {
                                <li>
                                    <a
                                        href=format!("#{id}")
                                        class=format!("{indent} block py-1 text-xs text-comb hover:text-honey transition-colors")
                                    >
                                        {*text}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()
                    }
                </ul>
            </div>
        </aside>
    }
    .into_any()
}
```

- [ ] **Step 2: Verify build with all components**

Run: `cargo leptos build`

Expected: Compiles successfully. Some warnings about unused code are expected at this stage.

- [ ] **Step 3: Commit all components**

```bash
git add src/components/ src/pages/mod.rs
git commit -m "feat(components): add nav, footer, scroll reveal, doc sidebar, and doc TOC"
```

## Chunk 5: Pages

### Task 18: Landing Page

**Files:**
- Create: `src/pages/landing.rs`

- [ ] **Step 1: Create landing.rs with all 8 sections**

```rust
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <Title text="CommitBee - The commit message generator that actually understands your code"/>
        <Meta name="description" content="CommitBee uses tree-sitter semantic analysis and LLMs to generate high-quality conventional commit messages. Rust CLI tool."/>
        <Meta property="og:title" content="CommitBee"/>
        <Meta property="og:description" content="The commit message generator that actually understands your code."/>
        <Meta property="og:image" content="/images/og.png"/>

        <div>
            <HeroSection/>
            <ProblemSection/>
            <PipelineSection/>
            <DifferentiatorsSection/>
            <ComparisonSection/>
            <InstallSection/>
            <DocsTransitionSection/>
        </div>
    }
}

// --- Section 1: Hero ---

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section class="relative min-h-screen flex items-center justify-center overflow-hidden">
            // Hex background
            <div class="absolute inset-0 hex-bg"/>

            <div class="relative mx-auto max-w-4xl px-4 text-center">
                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <h1 class="text-4xl font-bold tracking-tight text-bark sm:text-6xl lg:text-7xl">
                        "The commit message generator that "
                        <span class="text-honey">"actually understands"</span>
                        " your code."
                    </h1>
                </crate::components::scroll_reveal::ScrollReveal>

                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <p class="mt-6 text-lg text-comb sm:text-xl max-w-2xl mx-auto">
                        "CommitBee parses your code with tree-sitter, maps diff hunks to symbol spans, and gives the LLM structured semantic context."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <div class="mt-10 flex items-center justify-center gap-4">
                        <a
                            href="#install"
                            class="rounded-lg bg-honey px-6 py-3 text-sm font-semibold text-white shadow-lg shadow-honey/25 hover:bg-honey-dark transition-colors"
                        >
                            "Get Started"
                        </a>
                        <a
                            href="#pipeline"
                            class="rounded-lg border border-honey/30 px-6 py-3 text-sm font-semibold text-bark hover:bg-honey/5 transition-colors"
                        >
                            "See How It Works"
                        </a>
                    </div>
                </crate::components::scroll_reveal::ScrollReveal>

                // Terminal mockup
                <crate::components::scroll_reveal::ScrollReveal class="reveal-scale">
                    <div class="mt-16 mx-auto max-w-2xl rounded-xl border border-honey/20 bg-surface-raised shadow-2xl shadow-honey/5 overflow-hidden">
                        <div class="flex items-center gap-2 px-4 py-3 bg-pollen/50 border-b border-honey/10">
                            <div class="w-3 h-3 rounded-full bg-red-400/60"/>
                            <div class="w-3 h-3 rounded-full bg-yellow-400/60"/>
                            <div class="w-3 h-3 rounded-full bg-green-400/60"/>
                            <span class="ml-2 text-xs text-comb">"~/project"</span>
                        </div>
                        <div class="p-4 font-mono text-sm text-bark">
                            <div class="text-comb">"$ commitbee"</div>
                            <div class="mt-2 text-comb">"Analyzing 3 staged files..."</div>
                            <div class="text-comb">"Extracting symbols (tree-sitter)..."</div>
                            <div class="mt-2">"feat(auth): add JWT token refresh with configurable expiry"</div>
                            <div class="mt-1 text-comb">"Commit? (Y/n) " <span class="terminal-cursor">" "</span></div>
                        </div>
                    </div>
                </crate::components::scroll_reveal::ScrollReveal>
            </div>
        </section>
    }
}

// --- Section 2: Problem Statement ---

#[component]
fn ProblemSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface-raised">
            <div class="mx-auto max-w-6xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Every other tool just pipes your diff to an LLM and "
                        <span class="text-honey">"hopes for the best."</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-16 grid grid-cols-1 gap-8 md:grid-cols-2">
                    // Left: raw diff
                    <crate::components::scroll_reveal::ScrollReveal class="reveal-left">
                        <div class="rounded-xl border border-red-500/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-red-500 mb-4">"What other tools see"</h3>
                            <pre class="font-mono text-xs text-comb overflow-x-auto">
                                <code>
                                    "- fn validate(&self) -> bool {\n"
                                    "-     self.token.len() > 0\n"
                                    "- }\n"
                                    "+ fn validate(&self) -> Result<(), AuthError> {\n"
                                    "+     if self.token.is_empty() {\n"
                                    "+         return Err(AuthError::EmptyToken);\n"
                                    "+     }\n"
                                    "+     self.check_expiry()?;\n"
                                    "+     Ok(())\n"
                                    "+ }"
                                </code>
                            </pre>
                            <div class="mt-4 rounded-lg bg-red-500/5 p-3">
                                <p class="text-sm text-comb italic">"\"update validate function\""</p>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    // Right: structured analysis
                    <crate::components::scroll_reveal::ScrollReveal class="reveal-right">
                        <div class="rounded-xl border border-honey/30 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-honey mb-4">"What CommitBee sees"</h3>
                            <div class="space-y-3">
                                <div class="rounded-lg bg-pollen p-3">
                                    <p class="text-xs font-semibold text-honey">"Symbol: validate()"</p>
                                    <p class="text-xs text-comb">"Modified signature: bool -> Result<(), AuthError>"</p>
                                </div>
                                <div class="rounded-lg bg-pollen p-3">
                                    <p class="text-xs font-semibold text-honey">"Evidence: BREAKING_CHANGE"</p>
                                    <p class="text-xs text-comb">"Public API return type changed"</p>
                                </div>
                                <div class="rounded-lg bg-pollen p-3">
                                    <p class="text-xs font-semibold text-honey">"Evidence: BUG_FIX"</p>
                                    <p class="text-xs text-comb">"Error handling added (was ignoring failures)"</p>
                                </div>
                            </div>
                            <div class="mt-4 rounded-lg bg-honey/10 p-3 border border-honey/20">
                                <p class="text-sm text-bark font-medium">"\"fix(auth)!: return Result from validate with expiry check\""</p>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>
                </div>
            </div>
        </section>
    }
}

// --- Section 3: Pipeline Demo ---

#[component]
fn PipelineSection() -> impl IntoView {
    view! {
        <section id="pipeline" class="py-24">
            <div class="mx-auto max-w-5xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "A " <span class="text-honey">"7-stage pipeline"</span> " from diff to commit"
                    </h2>
                    <p class="mt-4 text-center text-comb max-w-2xl mx-auto">
                        "Watch how CommitBee processes your code, step by step."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-16">
                    <crate::components::pipeline_demo::PipelineDemo/>
                </div>
            </div>
        </section>
    }
}

// --- Section 4: Differentiators ---

#[component]
fn DifferentiatorsSection() -> impl IntoView {
    view! {
        <section class="py-24 bg-surface-raised">
            <div class="mx-auto max-w-6xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "What sets CommitBee " <span class="text-honey">"apart"</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-16 grid grid-cols-1 gap-8 md:grid-cols-3 reveal-stagger">
                    <FeatureCard
                        title="Tree-sitter Semantic Analysis"
                        description="It reads your code, not just your diffs. Extracts 10 symbol types across 10 languages and maps diff hunks to their spans."
                        icon="🌳"
                    />
                    <FeatureCard
                        title="Commit Splitting"
                        description="It detects mixed concerns and splits them into separate, well-typed commits using diff-shape fingerprinting."
                        icon="🔀"
                    />
                    <FeatureCard
                        title="25-Pattern Secret Scanning"
                        description="It catches leaked credentials before they reach any LLM. 25 patterns across 13 categories, fully customizable."
                        icon="🔒"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    title: &'static str,
    description: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <crate::components::scroll_reveal::ScrollReveal>
            <div class="rounded-xl border border-honey/20 bg-surface p-6 hover:border-honey/40 hover:shadow-lg hover:shadow-honey/5 transition-all">
                <div class="text-3xl mb-4">{icon}</div>
                <h3 class="text-lg font-semibold text-bark">{title}</h3>
                <p class="mt-2 text-sm text-comb">{description}</p>
            </div>
        </crate::components::scroll_reveal::ScrollReveal>
    }
}

// --- Section 5: Competitive Comparison (layered cards, not a flat table) ---

#[component]
fn ComparisonSection() -> impl IntoView {
    view! {
        <section class="py-24">
            <div class="mx-auto max-w-5xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "CommitBee vs. the field"
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                // Two stacked cards: CommitBee (foreground, elevated) vs Others (background, recessed)
                <div class="mt-12 relative">
                    // Background card: "Others" — slightly offset, muted
                    <crate::components::scroll_reveal::ScrollReveal class="reveal-scale">
                        <div class="rounded-xl border border-comb/10 bg-surface-raised p-6 ml-4 mr-0 sm:ml-8 sm:mr-0">
                            <h3 class="text-sm font-semibold text-comb mb-4">"Other commit generators"</h3>
                            <div class="grid grid-cols-2 gap-3 sm:grid-cols-3">
                                <ComparisonFeature label="Local LLM" has=true/>
                                <ComparisonFeature label="Cloud providers" has=true/>
                                <ComparisonFeature label="Git hooks" has=true/>
                                <ComparisonFeature label="Streaming output" has=true/>
                                <ComparisonFeature label="Tree-sitter AST" has=false/>
                                <ComparisonFeature label="Commit splitting" has=false/>
                                <ComparisonFeature label="Secret scanning" has=false/>
                                <ComparisonFeature label="Evidence typing" has=false/>
                                <ComparisonFeature label="Multi-pass validation" has=false/>
                                <ComparisonFeature label="Token budget" has=false/>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    // Foreground card: CommitBee — overlapping, elevated with shadow
                    <crate::components::scroll_reveal::ScrollReveal class="reveal-scale">
                        <div class="-mt-8 relative z-10 rounded-xl border-2 border-honey/30 bg-surface p-6 shadow-xl shadow-honey/10 mr-4 ml-0 sm:mr-8 sm:ml-0">
                            <div class="flex items-center gap-2 mb-4">
                                <span class="text-xl">"🐝"</span>
                                <h3 class="text-sm font-semibold text-honey">"CommitBee"</h3>
                            </div>
                            <div class="grid grid-cols-2 gap-3 sm:grid-cols-3">
                                <ComparisonFeature label="Tree-sitter AST" has=true/>
                                <ComparisonFeature label="Commit splitting" has=true/>
                                <ComparisonFeature label="Secret scanning" has=true/>
                                <ComparisonFeature label="Evidence typing" has=true/>
                                <ComparisonFeature label="Multi-pass validation" has=true/>
                                <ComparisonFeature label="Token budget" has=true/>
                                <ComparisonFeature label="Local LLM" has=true/>
                                <ComparisonFeature label="Cloud providers" has=true/>
                                <ComparisonFeature label="Git hooks" has=true/>
                                <ComparisonFeature label="Streaming output" has=true/>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ComparisonFeature(
    label: &'static str,
    has: bool,
) -> impl IntoView {
    view! {
        <div class=format!(
            "flex items-center gap-2 rounded-lg px-3 py-2 text-sm {}",
            if has { "bg-honey/5 text-bark" } else { "bg-surface-raised text-comb/50" }
        )>
            {if has {
                view! { <span class="text-honey font-medium">"✓"</span> }.into_any()
            } else {
                view! { <span class="text-comb/30">"—"</span> }.into_any()
            }}
            <span>{label}</span>
        </div>
    }
}

// --- Section 6: Install + Quick Start ---

#[component]
fn InstallSection() -> impl IntoView {
    view! {
        <section id="install" class="py-24 bg-surface-raised">
            <div class="mx-auto max-w-3xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Get started in " <span class="text-honey">"30 seconds"</span>
                    </h2>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-12 space-y-6">
                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="rounded-xl border border-honey/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-comb mb-3">"1. Install"</h3>
                            <div class="space-y-2">
                                <div class="rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                    <span class="text-comb">"$ "</span>"cargo install commitbee"
                                </div>
                                <div class="text-center text-xs text-comb">"or"</div>
                                <div class="rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                    <span class="text-comb">"$ "</span>"brew install sephyi/tap/commitbee"
                                </div>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="rounded-xl border border-honey/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-comb mb-3">"2. Pull a model"</h3>
                            <div class="rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                <span class="text-comb">"$ "</span>"ollama pull qwen3.5:4b"
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <div class="rounded-xl border border-honey/20 bg-surface p-6">
                            <h3 class="text-sm font-semibold text-comb mb-3">"3. Commit"</h3>
                            <div class="rounded-lg bg-pollen p-4 font-mono text-sm text-bark">
                                <div><span class="text-comb">"$ "</span>"git add src/feature.rs"</div>
                                <div><span class="text-comb">"$ "</span>"commitbee"</div>
                            </div>
                        </div>
                    </crate::components::scroll_reveal::ScrollReveal>

                    <crate::components::scroll_reveal::ScrollReveal>
                        <p class="text-center text-comb">
                            "That's it. Works with zero configuration if Ollama is running."
                        </p>
                    </crate::components::scroll_reveal::ScrollReveal>
                </div>
            </div>
        </section>
    }
}

// --- Section 7: Docs Transition ---

#[component]
fn DocsTransitionSection() -> impl IntoView {
    view! {
        <section class="py-24">
            <div class="mx-auto max-w-5xl px-4">
                <crate::components::scroll_reveal::ScrollReveal>
                    <h2 class="text-3xl font-bold text-center text-bark sm:text-4xl">
                        "Dive deeper"
                    </h2>
                    <p class="mt-4 text-center text-comb max-w-xl mx-auto">
                        "Explore the full documentation to master CommitBee."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <div class="mt-12 grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <DocLink slug="getting-started" title="Getting Started" description="Install and generate your first commit message"/>
                    <DocLink slug="configuration" title="Configuration" description="5-level config system, TOML files, environment variables"/>
                    <DocLink slug="llm-providers" title="LLM Providers" description="Ollama, OpenAI, and Anthropic setup"/>
                    <DocLink slug="architecture" title="Architecture" description="Deep dive into the pipeline internals"/>
                </div>
            </div>
        </section>
    }
}

#[component]
fn DocLink(
    slug: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <crate::components::scroll_reveal::ScrollReveal>
            <a
                href=format!("/docs/{slug}")
                class="block rounded-xl border border-honey/20 bg-surface-raised p-6 hover:border-honey/40 hover:shadow-lg hover:shadow-honey/5 transition-all group"
            >
                <h3 class="text-lg font-semibold text-bark group-hover:text-honey transition-colors">{title}</h3>
                <p class="mt-1 text-sm text-comb">{description}</p>
                <span class="mt-3 inline-block text-sm text-honey">"Read more →"</span>
            </a>
        </crate::components::scroll_reveal::ScrollReveal>
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/pages/landing.rs
git commit -m "feat(landing): add cinematic landing page with hero, problem, pipeline, differentiators, install, and docs transition"
```

### Task 19: Docs Page

**Files:**
- Create: `src/pages/docs.rs`

- [ ] **Step 1: Create docs.rs with three-column doc layout**

```rust
use leptos::prelude::*;
use leptos_meta::*;
use crate::content::loader;
use crate::components::{doc_sidebar::DocSidebar, doc_toc::DocToc};

#[component]
pub fn DocsPage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let slug = move || {
        params.with(|p| p.get("slug").unwrap_or_default().to_string())
    };

    let page_view = move || {
        let current_slug = slug();
        match loader::get_page(&current_slug) {
            Some(page) => {
                let (prev, next) = loader::get_adjacent(&current_slug);

                view! {
                    <Title text=format!("{} - CommitBee Docs", page.title)/>
                    <Meta name="description" content=page.description/>
                    <Meta property="og:title" content=format!("{} - CommitBee Docs", page.title)/>
                    <Meta property="og:description" content=page.description/>

                    <div class="flex min-h-screen">
                        // Left sidebar
                        <div class="hidden lg:block">
                            <DocSidebar current_slug=current_slug.clone()/>
                        </div>

                        // Main content
                        <main class="flex-1 min-w-0 px-6 py-8 lg:px-12">
                            // Breadcrumbs
                            <nav class="mb-6 text-sm text-comb" aria-label="Breadcrumb">
                                <a href="/docs/getting-started" class="hover:text-honey transition-colors">"Docs"</a>
                                " / "
                                <span class="text-comb">{page.section}</span>
                                " / "
                                <span class="text-bark font-medium">{page.title}</span>
                            </nav>

                            // Rendered markdown content
                            <article class="prose max-w-prose">
                                <div inner_html=page.html_content/>
                            </article>

                            // Prev/next navigation
                            <nav class="mt-12 flex justify-between border-t border-honey/10 pt-6">
                                {prev.map(|p| view! {
                                    <a href=format!("/docs/{}", p.slug) class="group">
                                        <span class="text-xs text-comb">"Previous"</span>
                                        <div class="text-sm font-medium text-bark group-hover:text-honey transition-colors">
                                            "← " {p.title}
                                        </div>
                                    </a>
                                })}
                                <div/>
                                {next.map(|p| view! {
                                    <a href=format!("/docs/{}", p.slug) class="group text-right">
                                        <span class="text-xs text-comb">"Next"</span>
                                        <div class="text-sm font-medium text-bark group-hover:text-honey transition-colors">
                                            {p.title} " →"
                                        </div>
                                    </a>
                                })}
                            </nav>
                        </main>

                        // Right TOC
                        <DocToc headings=page.headings/>
                    </div>
                }.into_any()
            }
            None => {
                view! {
                    <Title text="Page Not Found - CommitBee Docs"/>
                    <div class="flex items-center justify-center min-h-[50vh]">
                        <div class="text-center">
                            <h1 class="text-4xl font-bold text-bark">"404"</h1>
                            <p class="mt-2 text-comb">"Documentation page not found."</p>
                            <a href="/docs/getting-started" class="mt-4 inline-block text-honey hover:underline">
                                "Go to Getting Started"
                            </a>
                        </div>
                    </div>
                }.into_any()
            }
        }
    };

    view! {
        {page_view}
    }
}
```

### Task 20: Not Found Page

**Files:**
- Create: `src/pages/not_found.rs`

- [ ] **Step 1: Create not_found.rs**

```rust
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="404 - CommitBee"/>
        <div class="flex items-center justify-center min-h-screen">
            <div class="text-center">
                <div class="text-6xl mb-4">"🐝"</div>
                <h1 class="text-5xl font-bold text-bark">"404"</h1>
                <p class="mt-4 text-lg text-comb">"This page buzzed off somewhere."</p>
                <div class="mt-8 flex items-center justify-center gap-4">
                    <a href="/" class="rounded-lg bg-honey px-6 py-3 text-sm font-semibold text-white hover:bg-honey-dark transition-colors">
                        "Go Home"
                    </a>
                    <a href="/docs/getting-started" class="rounded-lg border border-honey/30 px-6 py-3 text-sm font-semibold text-bark hover:bg-honey/5 transition-colors">
                        "Read Docs"
                    </a>
                </div>
            </div>
        </div>
    }
}
```

### Task 21: Update Router in app.rs

**Files:**
- Modify: `src/app.rs`

- [ ] **Step 1: Update app.rs with full routing and layout**

Replace the entire `src/app.rs`:

```rust
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{docs::DocsPage, landing::Landing, not_found::NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Nav/>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("/") view=Landing/>
                <Route path=path!("/docs/:slug") view=DocsPage/>
            </Routes>
            <Footer/>
        </Router>
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/images/favicon.svg" type="image/svg+xml"/>
                <link rel="alternate icon" href="/images/favicon.ico"/>
                <link rel="apple-touch-icon" href="/images/apple-touch-icon.png"/>
                <link rel="preload" href="/fonts/Inter-Regular.woff2" as_="font" type_="font/woff2" crossorigin="anonymous"/>
                // Inline theme script: prevents flash of wrong theme (FOUC).
                // Runs before first paint, before any WASM hydrates.
                <script>{r#"
                    (function(){
                        var t = localStorage.getItem('theme');
                        if (t === 'dark' || (!t && matchMedia('(prefers-color-scheme:dark)').matches)) {
                            document.documentElement.classList.add('dark');
                        }
                    })();
                "#}</script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body class="bg-surface text-bark antialiased">
                <App/>
            </body>
        </html>
    }
}
```

- [ ] **Step 2: Verify full build**

Run: `cargo leptos build`

Expected: Compiles successfully. The landing page and docs routes are registered.

- [ ] **Step 3: Commit**

```bash
git add src/pages/ src/app.rs
git commit -m "feat(pages): add landing page, docs renderer, 404 page, and full router"
```

## Chunk 6: Interactive Islands

### Task 22: Theme Toggle Island

**Files:**
- Create: `src/components/theme_toggle.rs`

- [ ] **Step 1: Create theme_toggle.rs**

```rust
use leptos::prelude::*;

#[island]
pub fn ThemeToggle() -> impl IntoView {
    let (is_dark, set_is_dark) = signal(false);

    // Initialize from localStorage or system preference
    Effect::new(move || {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        let preference = storage.get_item("theme").unwrap_or(None);
        let dark = match preference.as_deref() {
            Some("dark") => true,
            Some("light") => false,
            _ => {
                // Check system preference
                window
                    .match_media("(prefers-color-scheme: dark)")
                    .ok()
                    .flatten()
                    .map(|mql| mql.matches())
                    .unwrap_or(false)
            }
        };

        set_is_dark.set(dark);
        apply_theme(dark);
    });

    let toggle = move |_| {
        let new_dark = !is_dark.get();
        set_is_dark.set(new_dark);
        apply_theme(new_dark);

        if let Some(storage) = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
        {
            let _ = storage.set_item("theme", if new_dark { "dark" } else { "light" });
        }
    };

    view! {
        <button
            on:click=toggle
            class="p-2 rounded-lg text-comb hover:text-bark hover:bg-surface-raised transition-colors"
            aria-label="Toggle dark mode"
        >
            {move || if is_dark.get() {
                // Sun icon for dark mode (click to go light)
                view! {
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/>
                    </svg>
                }.into_any()
            } else {
                // Moon icon for light mode (click to go dark)
                view! {
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/>
                    </svg>
                }.into_any()
            }}
        </button>
    }
}

fn apply_theme(dark: bool) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(html) = document.document_element() {
            let class_list = html.class_list();
            if dark {
                let _ = class_list.add_1("dark");
            } else {
                let _ = class_list.remove_1("dark");
            }
        }
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/theme_toggle.rs
git commit -m "feat(island): add dark/light theme toggle with localStorage persistence"
```

### Task 23: Code Block Copy Island

**Files:**
- Create: `src/components/code_block.rs`

- [ ] **Step 1: Create code_block.rs with copy-to-clipboard functionality**

This island attaches click handlers to all `.copy-btn` elements rendered by the build-time code blocks.

```rust
use leptos::prelude::*;

/// Island that activates copy-to-clipboard buttons on code blocks.
/// Rendered code blocks (from build.rs) contain `<button class="copy-btn" data-code="...">`.
/// This island finds them and attaches click handlers.
#[island]
pub fn CodeBlockActivator() -> impl IntoView {
    Effect::new(move || {
        let document = web_sys::window().unwrap().document().unwrap();
        let buttons = document.query_selector_all(".copy-btn").unwrap();

        for i in 0..buttons.length() {
            if let Some(btn) = buttons.item(i) {
                let btn_el: web_sys::HtmlElement = btn.unchecked_into();
                let btn_clone = btn_el.clone();

                let closure = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                    if let Some(code) = btn_clone.dataset().get("code") {
                        let window = web_sys::window().unwrap();
                        let navigator = window.navigator();
                        let clipboard = navigator.clipboard();
                        let _ = clipboard.write_text(&code);

                        // Brief visual feedback
                        btn_clone.set_inner_html("Copied!");
                        let btn_reset = btn_clone.clone();
                        let timeout_closure = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                            btn_reset.set_inner_html("Copy");
                        });
                        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout_closure.as_ref().unchecked_ref(),
                            2000,
                        );
                        timeout_closure.forget();
                    }
                });

                let _ = btn_el.add_event_listener_with_callback(
                    "click",
                    closure.as_ref().unchecked_ref(),
                );
                closure.forget();
            }
        }
    });

    // This island renders nothing visible — it just activates existing buttons
    view! { <div class="hidden" data-code-block-activator="true"/> }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/components/code_block.rs
git commit -m "feat(island): add code block copy-to-clipboard activator"
```

### Task 24: Doc Search Island

**Files:**
- Create: `src/components/doc_search.rs`

- [ ] **Step 1: Create doc_search.rs with fuzzy search modal**

```rust
use leptos::prelude::*;

#[island]
pub fn DocSearch() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (query, set_query) = signal(String::new());
    let (results, set_results) = signal::<Vec<SearchResult>>(vec![]);
    let (index, set_index) = signal::<Option<Vec<SearchEntry>>>(None);

    // Listen for Cmd+K / Ctrl+K
    Effect::new(move || {
        let closure = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
            move |e: web_sys::KeyboardEvent| {
                if (e.meta_key() || e.ctrl_key()) && e.key() == "k" {
                    e.prevent_default();
                    set_is_open.update(|v| *v = !*v);
                }
                if e.key() == "Escape" {
                    set_is_open.set(false);
                }
            },
        );
        let window = web_sys::window().unwrap();
        let _ = window.add_event_listener_with_callback(
            "keydown",
            closure.as_ref().unchecked_ref(),
        );
        closure.forget();
    });

    // Lazy-load search index when modal opens
    Effect::new(move || {
        if is_open.get() && index.get().is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                let window = web_sys::window().unwrap();
                let resp = wasm_bindgen_futures::JsFuture::from(
                    window.fetch_with_str("/search_index.json"),
                )
                .await;

                if let Ok(resp) = resp {
                    let resp: web_sys::Response = resp.unchecked_into();
                    if let Ok(json) = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap()).await {
                        if let Some(text) = json.as_string() {
                            if let Ok(entries) = serde_json::from_str::<Vec<SearchEntry>>(&text) {
                                set_index.set(Some(entries));
                            }
                        }
                    }
                }
            });
        }
    });

    // Perform search when query changes
    Effect::new(move || {
        let q = query.get();
        if q.is_empty() {
            set_results.set(vec![]);
            return;
        }

        if let Some(entries) = index.get().as_ref() {
            let mut scored: Vec<SearchResult> = entries
                .iter()
                .filter_map(|entry| {
                    let title_match = sublime_fuzzy::best_match(&q, &entry.title);
                    let excerpt_match = sublime_fuzzy::best_match(&q, &entry.excerpt);
                    let heading_score: Option<isize> = entry
                        .headings
                        .iter()
                        .filter_map(|h| sublime_fuzzy::best_match(&q, h).map(|m| m.score()))
                        .max();

                    let best_score = [
                        title_match.as_ref().map(|m| m.score() * 3), // Title matches weighted 3x
                        heading_score.map(|s| s * 2),                // Heading matches weighted 2x
                        excerpt_match.as_ref().map(|m| m.score()),
                    ]
                    .into_iter()
                    .flatten()
                    .max();

                    best_score.map(|score| SearchResult {
                        slug: entry.slug.clone(),
                        title: entry.title.clone(),
                        section: entry.section.clone(),
                        score,
                    })
                })
                .collect();

            scored.sort_by(|a, b| b.score.cmp(&a.score));
            scored.truncate(8);
            set_results.set(scored);
        }
    });

    view! {
        // Search trigger button (visible in nav)
        <button
            on:click=move |_| set_is_open.set(true)
            class="hidden md:flex items-center gap-2 rounded-lg border border-honey/20 bg-surface px-3 py-1.5 text-sm text-comb hover:border-honey/40 transition-colors"
        >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            "Search docs"
            <kbd class="ml-2 rounded bg-surface-raised px-1.5 py-0.5 text-xs text-comb">"⌘K"</kbd>
        </button>

        // Modal overlay
        <Show when=move || is_open.get()>
            <div
                class="fixed inset-0 z-[100] flex items-start justify-center pt-[20vh] bg-bark/50 backdrop-blur-sm"
                on:click=move |_| set_is_open.set(false)
            >
                <div
                    class="w-full max-w-lg rounded-xl border border-honey/20 bg-surface shadow-2xl"
                    on:click=move |e| e.stop_propagation()
                >
                    // Search input
                    <div class="flex items-center border-b border-honey/10 px-4">
                        <svg class="w-5 h-5 text-comb" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                        </svg>
                        <input
                            type="text"
                            placeholder="Search documentation..."
                            class="flex-1 bg-transparent px-3 py-4 text-bark placeholder:text-comb/50 outline-none"
                            autofocus=true
                            on:input=move |e| set_query.set(event_target_value(&e))
                        />
                        <kbd class="rounded bg-surface-raised px-2 py-1 text-xs text-comb">"esc"</kbd>
                    </div>

                    // Results
                    <div class="max-h-80 overflow-y-auto p-2">
                        {move || {
                            let r = results.get();
                            if r.is_empty() && !query.get().is_empty() {
                                view! { <p class="p-4 text-sm text-comb text-center">"No results found"</p> }.into_any()
                            } else {
                                r.into_iter()
                                    .map(|result| {
                                        let href = format!("/docs/{}", result.slug);
                                        view! {
                                            <a
                                                href=href
                                                class="block rounded-lg px-4 py-3 hover:bg-honey/5 transition-colors"
                                                on:click=move |_| set_is_open.set(false)
                                            >
                                                <div class="text-sm font-medium text-bark">{result.title.clone()}</div>
                                                <div class="text-xs text-comb">{result.section.clone()}</div>
                                            </a>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
struct SearchEntry {
    slug: String,
    title: String,
    section: String,
    headings: Vec<String>,
    excerpt: String,
}

#[derive(Clone, Debug)]
struct SearchResult {
    slug: String,
    title: String,
    section: String,
    score: isize,
}
```

Note: This island requires `wasm-bindgen-futures` and `js-sys` as dependencies. Add them to `Cargo.toml`:

```toml
# Under [dependencies]
wasm-bindgen-futures = { version = "0.4", optional = true }
js-sys = { version = "0.3", optional = true }

# Add to hydrate feature
# hydrate = [..., "dep:wasm-bindgen-futures", "dep:js-sys"]
```

- [ ] **Step 2: Update Cargo.toml with new dependencies**

Add `wasm-bindgen-futures` and `js-sys` to dependencies (optional, gated behind `hydrate`).

- [ ] **Step 3: Commit**

```bash
git add src/components/doc_search.rs Cargo.toml
git commit -m "feat(island): add fuzzy doc search with Cmd+K shortcut and lazy-loaded index"
```

### Task 25: Pipeline Demo Island

**Files:**
- Create: `src/components/pipeline_demo.rs`

- [ ] **Step 1: Create pipeline_demo.rs with step-through animated walkthrough**

```rust
use leptos::prelude::*;

const STEPS: &[PipelineStep] = &[
    PipelineStep {
        name: "Git Service",
        icon: "📂",
        description: "Reading staged changes...",
        detail: "3 files staged: auth.rs, config.rs, tests/auth_test.rs",
    },
    PipelineStep {
        name: "Tree-sitter",
        icon: "🌳",
        description: "Parsing symbols...",
        detail: "Found: fn validate() [modified-signature], fn refresh_token() [added], struct AuthConfig [modified]",
    },
    PipelineStep {
        name: "Splitter",
        icon: "🔀",
        description: "Analyzing change groups...",
        detail: "1 logical group detected — all files share auth symbol dependencies",
    },
    PipelineStep {
        name: "Context Builder",
        icon: "📋",
        description: "Building evidence...",
        detail: "EVIDENCE: bug_fix=true, breaking_change=true (return type changed)\nBUDGET: 2,847 / 6,000 tokens",
    },
    PipelineStep {
        name: "LLM",
        icon: "🧠",
        description: "Generating commit message...",
        detail: "fix(auth)!: return Result from validate with expiry check",
    },
    PipelineStep {
        name: "Validator",
        icon: "✅",
        description: "Checking 7 rules...",
        detail: "✓ fix-evidence  ✓ breaking-change  ✓ anti-hallucination\n✓ mechanical  ✓ dependency  ✓ specificity  ✓ length (52/72)",
    },
    PipelineStep {
        name: "Sanitizer",
        icon: "🧹",
        description: "Cleaning output...",
        detail: "fix(auth)!: return Result from validate with expiry check\n\nBREAKING CHANGE: validate() now returns Result<(), AuthError>\ninstead of bool. Callers must handle the error case.\n\nAdd token expiry checking to the validation pipeline. The\nprevious implementation silently ignored expired tokens.",
    },
];

struct PipelineStep {
    name: &'static str,
    icon: &'static str,
    description: &'static str,
    detail: &'static str,
}

#[island]
pub fn PipelineDemo() -> impl IntoView {
    let (current_step, set_current_step) = signal(0usize);
    let (is_playing, set_is_playing) = signal(false);

    // Auto-advance when playing
    Effect::new(move || {
        if is_playing.get() {
            let step = current_step.get();
            if step < STEPS.len() - 1 {
                let handle = set_timeout_with_handle(
                    move || {
                        set_current_step.update(|s| *s += 1);
                    },
                    std::time::Duration::from_millis(2000),
                );
                // Clean up on next effect run
                on_cleanup(move || {
                    if let Ok(h) = handle {
                        h.clear();
                    }
                });
            } else {
                set_is_playing.set(false);
            }
        }
    });

    view! {
        <div class="rounded-xl border border-honey/20 bg-surface-raised overflow-hidden">
            // Controls
            <div class="flex items-center gap-3 px-6 py-4 border-b border-honey/10">
                <button
                    on:click=move |_| {
                        if is_playing.get() {
                            set_is_playing.set(false);
                        } else {
                            if current_step.get() >= STEPS.len() - 1 {
                                set_current_step.set(0);
                            }
                            set_is_playing.set(true);
                        }
                    }
                    class="rounded-lg bg-honey px-4 py-2 text-sm font-medium text-white hover:bg-honey-dark transition-colors"
                >
                    {move || if is_playing.get() { "Pause" } else { "Play" }}
                </button>
                <button
                    on:click=move |_| {
                        set_is_playing.set(false);
                        set_current_step.update(|s| if *s > 0 { *s -= 1 });
                    }
                    class="rounded-lg border border-honey/20 px-3 py-2 text-sm text-comb hover:text-bark transition-colors"
                    disabled=move || current_step.get() == 0
                >
                    "← Prev"
                </button>
                <button
                    on:click=move |_| {
                        set_is_playing.set(false);
                        set_current_step.update(|s| if *s < STEPS.len() - 1 { *s += 1 });
                    }
                    class="rounded-lg border border-honey/20 px-3 py-2 text-sm text-comb hover:text-bark transition-colors"
                    disabled=move || current_step.get() >= STEPS.len() - 1
                >
                    "Next →"
                </button>
                <span class="ml-auto text-sm text-comb">
                    {move || format!("Step {} of {}", current_step.get() + 1, STEPS.len())}
                </span>
            </div>

            // Step progress bar
            <div class="flex gap-1 px-6 py-3 bg-pollen/30">
                {STEPS.iter().enumerate().map(|(i, step)| {
                    view! {
                        <div class="flex-1">
                            <div class=move || {
                                let active = i <= current_step.get();
                                let current = i == current_step.get();
                                format!(
                                    "h-1 rounded-full transition-all duration-500 {}",
                                    if current { "bg-honey" }
                                    else if active { "bg-honey/40" }
                                    else { "bg-honey/10" }
                                )
                            }/>
                            <div class="mt-1 text-center">
                                <span class=move || {
                                    let active = i <= current_step.get();
                                    format!(
                                        "text-[10px] font-medium {}",
                                        if active { "text-honey" } else { "text-comb/50" }
                                    )
                                }>
                                    {step.name}
                                </span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            // Current step detail
            <div class="p-6">
                {move || {
                    let step = &STEPS[current_step.get()];
                    view! {
                        <div class="pipeline-step active">
                            <div class="flex items-center gap-3 mb-4">
                                <span class="text-3xl">{step.icon}</span>
                                <div>
                                    <h3 class="text-lg font-semibold text-bark">{step.name}</h3>
                                    <p class="text-sm text-comb">{step.description}</p>
                                </div>
                            </div>
                            <pre class="rounded-lg bg-pollen p-4 font-mono text-sm text-bark whitespace-pre-wrap">
                                {step.detail}
                            </pre>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
```

- [ ] **Step 2: Verify full build with all islands**

Run: `cargo leptos build`

Expected: Both server and WASM client compile successfully.

- [ ] **Step 3: Commit**

```bash
git add src/components/pipeline_demo.rs
git commit -m "feat(island): add interactive pipeline demo with play/pause and step-through"
```

## Chunk 7: Deployment

### Task 26: Pre-Render Script

**Files:**
- Create: `scripts/prerender.sh`

- [ ] **Step 1: Create scripts/prerender.sh**

```bash
#!/usr/bin/env bash
set -euo pipefail

# Pre-render all routes from the built Leptos app to static HTML.
# Reads routes from the build-generated routes.txt manifest.

SITE_DIR="target/site"
DIST_DIR="dist"
PORT=3000
SERVER_BIN="target/release/commitbee-web"

echo "==> Starting pre-render..."

# Clean dist
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Start the server in background
LEPTOS_SITE_ADDR="127.0.0.1:${PORT}" "$SERVER_BIN" &
SERVER_PID=$!

# Wait for server to be ready
for i in $(seq 1 30); do
    if curl -s "http://127.0.0.1:${PORT}/" > /dev/null 2>&1; then
        echo "==> Server ready on port ${PORT}"
        break
    fi
    if [ "$i" -eq 30 ]; then
        echo "ERROR: Server failed to start"
        kill "$SERVER_PID" 2>/dev/null || true
        exit 1
    fi
    sleep 0.5
done

# Find routes.txt in OUT_DIR (build artifacts)
ROUTES_FILE=$(find target -name "routes.txt" -path "*/build/commitbee-web-*/out/*" | head -1)

if [ -z "$ROUTES_FILE" ]; then
    echo "ERROR: routes.txt not found in build output"
    kill "$SERVER_PID" 2>/dev/null || true
    exit 1
fi

echo "==> Using routes from: ${ROUTES_FILE}"

# Pre-render each route
while IFS= read -r route; do
    [ -z "$route" ] && continue

    if [ "$route" = "/" ]; then
        OUTPUT_PATH="${DIST_DIR}/index.html"
    else
        OUTPUT_PATH="${DIST_DIR}${route}/index.html"
    fi

    mkdir -p "$(dirname "$OUTPUT_PATH")"
    echo "    Rendering: ${route} -> ${OUTPUT_PATH}"
    curl -s "http://127.0.0.1:${PORT}${route}" > "$OUTPUT_PATH"
done < "$ROUTES_FILE"

# Copy static assets
echo "==> Copying static assets..."
cp -r "${SITE_DIR}/pkg" "${DIST_DIR}/pkg"

if [ -d "public" ]; then
    cp -r public/* "${DIST_DIR}/" 2>/dev/null || true
fi

# Copy Tailwind CSS output
if [ -f "${SITE_DIR}/pkg/commitbee-web.css" ]; then
    cp "${SITE_DIR}/pkg/commitbee-web.css" "${DIST_DIR}/pkg/"
fi

# Generate 404.html
if [ -f "${DIST_DIR}/index.html" ]; then
    cp "${DIST_DIR}/index.html" "${DIST_DIR}/404.html"
fi

# Stop server
kill "$SERVER_PID" 2>/dev/null || true
wait "$SERVER_PID" 2>/dev/null || true

echo "==> Pre-render complete. Output in ${DIST_DIR}/"
echo "    Routes rendered: $(wc -l < "$ROUTES_FILE" | tr -d ' ')"
echo "    Total files: $(find "$DIST_DIR" -type f | wc -l | tr -d ' ')"
```

- [ ] **Step 2: Make it executable**

Run: `chmod +x scripts/prerender.sh`

- [ ] **Step 3: Add prerender task to mise.toml**

Add to `mise.toml`:

```toml
[tasks.prerender]
run = "scripts/prerender.sh"
description = "Pre-render all routes to static HTML"
depends = ["build"]
```

- [ ] **Step 4: Commit**

```bash
git add scripts/prerender.sh mise.toml
git commit -m "feat(deploy): add pre-render script for static HTML generation"
```

### Task 27: GitHub Actions Workflow

**Files:**
- Create: `.github/workflows/deploy.yml`

- [ ] **Step 1: Create deploy.yml**

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [development]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@nightly
        with:
          targets: wasm32-unknown-unknown

      - name: Install cargo-leptos
        run: cargo install cargo-leptos

      - name: Cache cargo registry and build
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Build release
        run: cargo leptos build --release

      - name: Pre-render to static HTML
        run: |
          chmod +x scripts/prerender.sh
          scripts/prerender.sh

      - name: Setup Pages
        uses: actions/configure-pages@v5

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: dist

      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/deploy.yml
git commit -m "ci: add GitHub Actions workflow for build, pre-render, and deploy to Pages"
```

## Chunk 8: Integration and Polish

### Task 28: Add Search Island to Docs Nav

**Files:**
- Modify: `src/components/nav.rs`

- [ ] **Step 1: Add DocSearch island to nav for docs pages**

Update the nav desktop links section to include the search button. Add after the nav links `<div>`:

```rust
// Add search (appears on all pages, searches docs)
<crate::components::doc_search::DocSearch/>
```

### Task 29: Add CodeBlockActivator to Docs Page

**Files:**
- Modify: `src/pages/docs.rs`

- [ ] **Step 1: Add CodeBlockActivator island to docs page template**

Add inside the `<main>` content area, after the article:

```rust
<crate::components::code_block::CodeBlockActivator/>
```

### Task 30: Prose Styling for Doc Content

**Files:**
- Modify: `style/tailwind.css`

- [ ] **Step 1: Add prose styles for rendered markdown content**

Add to `@layer components` in `style/tailwind.css`:

```css
@layer components {
  .prose {
    @apply text-bark leading-relaxed;
  }

  .prose h1 {
    @apply text-3xl font-bold mt-0 mb-6;
  }

  .prose h2 {
    @apply text-2xl font-semibold mt-12 mb-4 pb-2 border-b border-honey/10;
  }

  .prose h3 {
    @apply text-xl font-semibold mt-8 mb-3;
  }

  .prose h4 {
    @apply text-lg font-medium mt-6 mb-2;
  }

  .prose p {
    @apply my-4;
  }

  .prose a {
    @apply text-honey hover:underline;
  }

  .prose ul {
    @apply my-4 pl-6 list-disc;
  }

  .prose ol {
    @apply my-4 pl-6 list-decimal;
  }

  .prose li {
    @apply my-1;
  }

  .prose code:not(pre code) {
    @apply rounded bg-pollen px-1.5 py-0.5 text-sm font-mono;
  }

  .prose pre {
    @apply my-6 rounded-lg bg-pollen p-4 overflow-x-auto;
  }

  .prose blockquote {
    @apply my-4 border-l-4 border-honey/30 pl-4 italic text-comb;
  }

  .prose table {
    @apply my-6 w-full border-collapse;
  }

  .prose th {
    @apply border-b border-honey/20 px-4 py-2 text-left font-semibold;
  }

  .prose td {
    @apply border-b border-honey/5 px-4 py-2;
  }

  .prose hr {
    @apply my-8 border-honey/10;
  }

  .prose strong {
    @apply font-semibold;
  }

  .prose img {
    @apply my-6 rounded-lg;
  }
}
```

### Task 31: Add Favicon and OG Image Placeholders

**Files:**
- Create: `public/images/favicon.svg`
- Create: `public/images/.gitkeep` (already exists)

- [ ] **Step 1: Create minimal SVG favicon**

```svg
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <text y=".9em" font-size="90">🐝</text>
</svg>
```

Save to `public/images/favicon.svg`.

### Task 32: Full Build Verification

- [ ] **Step 1: Build the complete project**

Run: `cargo leptos build --release`

Expected: Successful release build of both server binary and WASM bundle.

- [ ] **Step 2: Run content validation tests**

Run: `cargo test --test content_check`

Expected: All tests pass.

- [ ] **Step 3: Test the dev server**

Run: `cargo leptos watch` (then visit `http://127.0.0.1:3000` in browser)

Verify:
- Landing page renders with all sections
- `/docs/getting-started` renders with sidebar and content
- Theme toggle works (switches dark/light)
- Pipeline demo plays through steps
- Code blocks have copy buttons
- `Cmd+K` opens search modal

- [ ] **Step 4: Run the pre-render script**

Run: `mise run prerender`

Expected: `dist/` directory populated with static HTML for all routes.

- [ ] **Step 5: Final commit**

```bash
git add -A
git commit -m "feat: complete commitbee-web with landing page, docs wiki, islands, and deployment pipeline"
```

## Dependency Summary

For quick reference when resolving build issues:

| Crate | Version | Feature Gate | Purpose |
| --- | --- | --- | --- |
| `leptos` | 0.7 | islands + ssr/hydrate | Core framework |
| `leptos_meta` | 0.7 | ssr/hydrate | Meta tags |
| `leptos_router` | 0.7 | ssr/hydrate | Routing |
| `leptos_axum` | 0.7 | ssr | Server integration |
| `axum` | 0.8 | ssr | HTTP server |
| `tokio` | 1 | ssr | Async runtime |
| `tower` | 0.5 | ssr | Service traits |
| `tower-http` | 0.6 | ssr | Static files |
| `wasm-bindgen` | 0.2 | hydrate | WASM bindings |
| `web-sys` | 0.3 | hydrate | Browser APIs |
| `js-sys` | 0.3 | hydrate | JS interop |
| `wasm-bindgen-futures` | 0.4 | hydrate | Async WASM |
| `console_error_panic_hook` | 0.1 | hydrate | WASM debugging |
| `sublime_fuzzy` | 0.7 | always | Search matching |
| `serde` | 1 | always | Serialization |
| `serde_json` | 1 | always | JSON |
| `pulldown-cmark` | 0.12 | build-dep | Markdown |
| `syntect` | 5 | build-dep | Syntax highlighting |
| `serde_yaml` | 0.9 | build-dep + dev-dep | Frontmatter |
| `walkdir` | 2 | build-dep + dev-dep | Directory walk |
| `slug` | 0.1 | build-dep | URL slugs |
