<!--
SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>

SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
-->

# CommitBee Web

Website for [CommitBee](https://github.com/sephyi/commitbee) — cinematic landing page + docs wiki. Built with Leptos 0.8 islands, pre-rendered to static HTML for GitHub Pages.

## Prerequisites

- Rust nightly (`rust-toolchain.toml` specifies channel)
- `wasm32-unknown-unknown` target (auto-installed via `rust-toolchain.toml`)
- `cargo-leptos` (`cargo install cargo-leptos`)
- `mise` (optional, for task orchestration — manual commands listed below)

## Quick Start

```bash
mise run dev          # Dev server with hot reload (localhost:3000)
mise run build        # Production build (server + WASM)
```

## Architecture

**Dual-target build**: cargo-leptos compiles `src/main.rs` with `--features ssr` (native) and `src/lib.rs` with `--features hydrate` (wasm32). Both share the same source — feature gates control what compiles where.

- `src/main.rs` — Axum server (SSR only, not deployed)
- `src/lib.rs` — App root + WASM hydrate entry
- `src/app.rs` — Router, HTML shell, `<App/>` component
- `src/pages/` — Landing (7 sections), docs, 404
- `src/components/` — Islands and SSR components (see classification table below)
- `src/content/loader.rs` — `include!`s build.rs-generated content
- `src/bin/prerender.rs` — Cross-platform static HTML pre-renderer (zero extra deps)
- `build.rs` — Markdown pipeline: `content/docs/*.md` → `content_generated.rs` + `search_index.json` + `routes.txt`
- `content/docs/` — 12 markdown files with YAML frontmatter
- `style/` — Tailwind v4 CSS-first config + animations (no JS config)

### Component Classification

| Component | Type | Ships WASM | Purpose |
| --- | --- | --- | --- |
| `ThemeToggle` | `#[island]` | Yes | Dark/light mode toggle with localStorage |
| `PipelineDemo` | `#[island]` | Yes | Animated 7-step pipeline walkthrough |
| `CodeBlockActivator` | `#[island]` | Yes | Attaches copy-to-clipboard to `.copy-btn` elements |
| `DocSearch` | `#[island]` | Yes | Cmd+K fuzzy search modal, lazy-loads search index |
| `Nav` | `#[component]` | No | Sticky glassmorphism navigation header |
| `Footer` | `#[component]` | No | Site footer with hex-bg pattern |
| `ScrollReveal` | `#[component]` | No | Wraps children in `.reveal` div for scroll animation |
| `DocSidebar` | `#[component]` | No | Sticky docs section tree with thin scrollbar |
| `DocToc` | `#[component]` | No | Right-side table of contents |

## Commands

```bash
mise run dev          # cargo leptos watch
mise run build        # cargo leptos build --release
mise run check        # Clippy for both SSR and WASM targets
mise run content      # Validate markdown frontmatter and links
mise run fmt          # cargo fmt
mise run prerender    # Pre-render all routes to static HTML (depends: build)
```

### Manual checks (when mise unavailable)

```bash
cargo check --features ssr                                        # Server
cargo check --features hydrate --target wasm32-unknown-unknown    # WASM
cargo test --test content_check                                   # Content validation (3 tests)
cargo leptos build --release                                      # Full build
```

## Content Workflow

Add a doc page:

1. Create `content/docs/my-page.md` with frontmatter: `title`, `order`, `section`, `description`
2. Run `mise run content` to validate
3. The page auto-appears at `/docs/my-page` on next build

Valid sections: `Basics`, `Usage`, `Internals`, `Integration`, `Reference`

## Key Design Decisions

1. **Islands architecture** — Only interactive components (`#[island]`) ship WASM; everything else is pure SSR HTML
2. **Router is NOT hydrated** — All `<a>` links trigger full browser navigation (server round-trips). `leptos_router` provides SSR-side routing only. Do not use `<A>` component or expect client-side nav.
3. **View Transitions + prefetch** — `@view-transition { navigation: auto; }` in tailwind.css gives cross-document transitions a SPA-like feel (Chrome 126+, Safari 18+). An inline prefetch script in `app.rs` pre-fetches same-origin pages on hover for near-instant navigation.
4. **Build-time content** — Markdown processed by `build.rs` into `const` statics via `include!` — zero runtime cost
5. **Cross-platform prerender** — `src/bin/prerender.rs` uses only `std` (TcpStream HTTP/1.0, no shell deps). Override port with `PRERENDER_PORT` env var.
6. **Self-hosted fonts** — Inter + JetBrains Mono in `public/fonts/`, no external font CDN
7. **Three load-bearing inline scripts in `app.rs`** — Must execute before WASM hydration, never move to islands: (1) theme class application (prevents FOUC), (2) scroll-reveal IntersectionObserver (reveals above-fold content instantly), (3) link prefetch on hover

## Leptos 0.8 Island Gotchas

**`#[island]` does NOT cfg-gate the function body.** The body compiles under both `ssr` and `hydrate` features. This is why `web-sys`, `wasm-bindgen`, `js-sys` are non-optional in Cargo.toml — they must resolve under SSR too (they compile as stubs on non-wasm targets).

**`#[prop(default = expr)]` fails for method calls.** The macro parses defaults as paths, not expressions. `"reveal".to_string()` causes a parse error. Use `#[prop(optional, into)]` and handle the default in the body instead.

**`leptos_meta` and `leptos_router` have no `hydrate` feature in 0.8.** Only `leptos` itself has `hydrate` and `islands`. SSR features: `leptos_meta/ssr`, `leptos_router/ssr`.

**Multiple `[[bin]]` targets break cargo-leptos.** When adding extra binaries (like `prerender`), you must set `bin-target = "commitbee-web"` in `[package.metadata.leptos]` so cargo-leptos knows which binary is the server.

**`ssr` and `hydrate` features are mutually exclusive.** Never combine them. `cargo leptos` manages this automatically; manual `cargo` commands must specify one or the other.

**`recursion_limit = "256"`** is set in `src/main.rs` because Leptos view macros expand deeply. If adding heavily nested views and hitting the limit, increase it.

## Gotchas

### Build System

- `cargo leptos watch` / `cargo leptos build` must be used for full builds (compiles both SSR + WASM + Tailwind in parallel)
- `cargo check --features ssr` and `cargo check --features hydrate --target wasm32-unknown-unknown` are independent — both must pass
- On a fresh clone, `search_index.json` is written to `target/site/` only if that directory exists. First build must be `cargo leptos build` (creates `target/site/`), not plain `cargo build`
- `routes.txt` manifest is generated by `build.rs` — the prerender binary finds it by walking `target/build/`
- cargo-leptos auto-generates `tailwind.config.js` (gitignored) — Tailwind v4 ignores it since we use CSS-first config in `style/tailwind.css`

### CSS / Styling

- Tailwind v4 uses `@source "../src/**/*.rs"` to scan Rust files for utility classes — classes in `build.rs` are NOT scanned, use CSS rules in `style/tailwind.css` instead
- `.prose` is a custom component layer in `tailwind.css`, not the Tailwind typography plugin
- Code blocks use a hardcoded `#2b303b` fallback background from syntect's base16-ocean.dark theme (set in `.code-block-wrapper` CSS rule)
- `build.rs` strips syntect's outer `<pre>` wrapper to avoid nested `<pre>` tags — the background color is extracted and applied to the wrapper div

## Code Style

- Rust 2024 edition, nightly toolchain
- `cargo fmt` + `clippy` (enforced by hooks)
- Bee-themed Tailwind tokens: `honey`, `nectar`, `comb`, `bark`, `pollen`, `surface`
- License: PolyForm-Noncommercial-1.0.0 (REUSE compliant via `REUSE.toml`)

## Claude Code Hooks

- **PreToolUse**: `block-generated-files.sh` — prevents editing `$OUT_DIR` artifacts and generated files
- **PostToolUse**: `rust-fmt.sh` — auto-formats any edited `.rs` file after changes

## References

- **PRD & Roadmap**: `PRD.md`
- **Design system spec**: `docs/superpowers/specs/2026-03-13-commitbee-web-design.md`
- **CommitBee CLI**: `../commitbee/` (the tool this website documents)
