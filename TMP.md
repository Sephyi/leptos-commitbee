<!--
SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
-->

# TMP.md — Codebase Audit & Improvement Plan

> Created: 2026-03-28. Last updated: 2026-04-07. Row #27 wrong-fix-reverted 2026-04-07.

## Implementation Progress

- **Phase 0 (CI infra)** — DONE — `6b5593c` (#28, #1, #2, #3, #4)
- **Phase 1 (WASM safety + XSS)** — DONE — `39d6998` (#6, #6b, #7, #29)
- **Phase 2 (build hardening)** — DONE — `4b4ca12` (#30, #10, #11), `87cb02c` (#13, #36)
- **Phase 3 (a11y)** — DONE — `970402a` (#31, #32), `ba11ecf` (#33)
- **Phase 4 (assets)** — DONE — `3133ed5` (#34, #15, #16 partial)
- **Phase 5 (CSP)** — DONE — `6d3de24` (#17)
- **Phase 6 (tests)** — DONE — `3ff7a0a` (#18, #20)
- **Phase 7 (landing split)** — DONE (#21)
- **Follow-up #16** — DONE — `dda7c67` (favicon + apple-touch-icon link tags)
- Merges into `development`: `d0b491c`, `eb0c994`, `6d94c95`

> Auditors: Claude Opus 4.6 (initial audit + plan), Codex gpt-5.4 xhigh (independent review x2)
> Status: Pre-implementation — all findings reconciled across three independent passes.

---

## Issue Tracker

### Original Issues (#1-#27)

| # | Category | Severity | Issue | Status | Source | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | CI | High | Deploy workflow missing `cargo test --test content_check` | Done | Claude | Covered by ci.yml. Commit `6b5593c` |
| 2 | CI | High | No `cargo clippy` in CI — linting regressions can ship | Done | Claude | ci.yml runs clippy SSR + WASM. Commit `6b5593c` |
| 3 | CI | Medium | No `cargo fmt --check` in CI — formatting not enforced | Done | Claude | Commit `6b5593c` |
| 4 | CI | Medium | No `cargo audit` in CI — security advisories not caught (PRD SR-003) | Done | Claude | Added to deploy.yml. Commit `6b5593c` |
| 5 | CI | Low | No Lighthouse CI (PRD TR-003) | Deferred | Claude | Disproportionate at current scale |
| 6 | WASM | Medium | `theme_toggle.rs` — double-unwrap on `local_storage()` panics if localStorage blocked | Done | Claude | Graceful None handling + try/catch on inline bootstrap. Commit `39d6998` |
| 7 | WASM | Medium | `doc_search.rs:119` — `resp.text().unwrap()` panics on Response body failure | Done | Claude | resp.ok() guard + chained validation. Commit `39d6998` |
| 8 | WASM | Low | `web_sys::window().unwrap()` appears ~11 times across island code | Open | Claude | Multiple island files |
| 9 | WASM | Low | 5 islands use `Closure::forget()` / `std::mem::forget()` — intentional, never cleaned up | Open | Claude | `DocSearch`, `CodeBlockActivator`, `MobileMenu`, `TocHighlighter`, `ThemeToggle` |
| 10 | Build | Medium | `build.rs` `add_heading_ids()` uses string-based HTML parsing — fragile if pulldown-cmark output changes | Done | Claude | Rewritten: IDs injected in cmark event stream; `add_heading_ids` removed. Commit `4b4ca12` |
| 11 | Build | Medium | `build.rs` `generate_rust_module()` minimal escaping (only `"` to `\"`) — `title`, `description`, and headings all affected | Done | Claude, Codex | Raw strings `r#"..."#` used for title, description, heading text. Commit `4b4ca12` |
| 12 | Build | Low | `build.rs` ~30 `unwrap()`/`expect()` calls — acceptable for build script | Skipped | Claude | Build script panics are diagnostic; skipped per plan |
| 13 | Build | Low | `prerender.rs` doesn't check HTTP status codes — 500 response written as-is | Done | Claude | http_get() parses status code, Err on non-200. Commit `87cb02c` |
| 14 | Assets | Medium | No OG images (PRD PR-005 mentions WebP) | Deferred | Claude | Requires design assets |
| 15 | Assets | Low | No CNAME file for custom domain (PRD DR-001) | Done | Claude | `public/CNAME` → `commitbee.dev`. Commit `3133ed5` |
| 16 | Assets | Low | No `apple-touch-icon` or `favicon.ico` | Partial | Claude | Placeholder 1×1 PNGs added to `public/images/`. `<link>` tags in `src/app.rs` deferred (off-limits worktree). Commit `3133ed5` |
| 17 | Security | Medium | CSP allows `'unsafe-inline'` and `'unsafe-eval'` — weakens policy significantly | Done | Claude | `unsafe-eval` dropped; `unsafe-inline` retained for scripts (Leptos hydration) and styles (Tailwind v4) with documented justification. Commit `6d3de24` |
| 18 | Testing | Medium | Only 3 content-validation tests — no Rust unit tests, component tests, or WASM tests | Done | Claude | 3→7 tests: order uniqueness, description length, delimiter-on-own-line, internal links. Commit `3ff7a0a` |
| 19 | Testing | Low | No PRD TR-001 through TR-005 test requirements implemented | Partial | Claude | TR-001 covered by content tests; TR-002..005 need browser infra |
| 20 | Testing | Low | No internal link validation between doc pages | Done | Claude | `internal_links_resolve()` test added. Commit `3ff7a0a` |
| 21 | Code | Low | `landing.rs` 415 lines in single file — could split into section modules | Done | Claude | Split into `src/pages/landing/{hero,problem,pipeline,differentiators,comparison,install,docs_transition}.rs` |
| 22 | Code | Low | Scroll-reveal `IntersectionObserver` in inline script runs indefinitely (never disconnected) | Open | Claude | `src/app.rs` inline script |
| 23 | Code | Low | `doc_search.rs` — `serde_json` imported but could be more explicit about dependency chain | Open | Claude | `src/components/doc_search.rs` |
| 24 | Docs | Done | All 12 doc pages updated from stubs to comprehensive content | Done | Claude | `content/docs/*.md` |
| 25 | Docs | Done | Test count corrected from 308 to 424 across all pages | Done | Claude | `content/docs/architecture.md` |
| 26 | Docs | Done | LLM provider TOML syntax fixed (was `[provider]` sections, now flat keys) | Done | Claude | `content/docs/llm-providers.md` |
| 27 | Docs | Wrong-fix-undone | Secret pattern count "corrected" (24 → 25) was wrong — true count is 24 | Reverted | Claude | Audit was incorrect; true count is 24 (verified against `src/services/safety.rs::builtin_patterns()` and DOCS.md). Also fixed `architecture.md` crate-tree comment (same wrong value). Reverted in `416d055` |

### New Issues from Codex Cross-Review (#28-#36)

These issues were discovered by Codex gpt-5.4 during independent codebase audit and plan review, and confirmed by Claude during reconciliation.

| # | Category | Severity | Issue | Status | Source | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| 28 | CI | **Critical** | No PR validation workflow — `deploy.yml` only runs on push to `development`, no `ci.yml` on `pull_request` | Open | Codex | `.github/workflows/` — highest-leverage CI gap |
| 29 | Security | High | Raw docs HTML injected via `inner_html` + CSP allows `unsafe-inline` = XSS vector if docs contain malicious content | Open | Codex | `build.rs:175,233` → `src/pages/docs.rs:48` → `src/app.rs:58` |
| 30 | Build | Medium | Frontmatter parsing bug — `after_first.find("---")` matches `---` inside YAML content, not just delimiter lines | Done | Codex | Fixed in both `build.rs` and `tests/content_check.rs`. Commit `4b4ca12` |
| 31 | A11y | Medium | Search modal lacks `role="dialog"`, `aria-modal`, focus trap — background content remains tabbable while open | Open | Codex | `src/components/doc_search.rs:189,195,203` |
| 32 | A11y | Medium | Mobile menu overlay lacks dialog/focus management — no scroll lock, keyboard focus can escape into page content | Open | Codex | `src/components/mobile_menu.rs:39,45,63` |
| 33 | A11y | Medium | Copy buttons invisible to keyboard users — `opacity-0` hover-only, tabbable but not visible | Open | Codex | `build.rs:213`, `src/pages/landing.rs:331` |
| 34 | Build | Medium | Search index only copied to `target/site` if directory already exists — first build can miss it; client fails silently | Done | Codex | `create_dir_all` before write. Commit `3133ed5` |
| 35 | Build | Low | Tailwind `@source "../src/**/*.rs"` doesn't scan `build.rs` output — classes emitted by build script may be missing from CSS | Open | Codex | `style/tailwind.css:11`, `build.rs:213` |
| 36 | Code | Low | Route discovery for prerendering uses recursive search for first `routes.txt` under `target/` — can pick stale artifact | Open | Codex | `src/bin/prerender.rs:60,205` |

---

## Reconciliation: Claude vs Codex Findings

### Items Both Auditors Agree On

- All 4 CI gaps (#1-#4) are real and should be fixed
- `Closure::forget()` (#9) is intentional — document only
- `build.rs` `unwrap()` usage (#12) is appropriate — skip
- OG images (#14) require design assets — defer
- CNAME (#15) and apple-touch-icon (#16) are correct and straightforward
- `landing.rs` refactor (#21) is low priority
- Scroll-reveal observer (#22) is effectively idle — document only
- `serde_json` clarity (#23) — not a real issue

### Codex Corrections Accepted

| Original | Correction | Rationale |
| --- | --- | --- |
| #1 severity High | Should be part of a larger CI workflow fix (#28) | The bigger gap is no PR validation at all, not just missing test step |
| #6 fix scope | Must also guard inline theme bootstrap script in `app.rs:64-71` | The inline `localStorage.getItem('theme')` has the same unwrap assumption as the island |
| #7 fix scope | Should also check `resp.ok()` / status before calling `text()` | A 404/500 on search_index.json would silently fail |
| #11 fix scope | Must fix `title` and `description` fields too, not just headings | All three are emitted as normal string literals with only `"` escaping |
| #13 fix | Parse numeric status code instead of `contains("200")` | More precise validation for the prerender HTTP check |
| #8 (tracked) | **Remove from active tracking** | Replacing `unwrap()` with `expect()` in a helper doesn't materially improve safety — `window` is always available in browser WASM context |
| #17 approach | Should experiment with tightening CSP before documenting as-is | Particularly `unsafe-eval` — may not be required by Leptos 0.8 |
| #23 (tracked) | **Remove from active tracking** | Codex and Claude agree this is not a real issue |

### Codex Corrections Rejected / Modified

| Codex Suggestion | Decision | Rationale |
| --- | --- | --- |
| Downgrade #1, #2 from High | **Partially accepted** — reframe as part of #28 | CI gates matter but the real fix is a separate PR workflow |
| #5 should not be deferred (PRD TR-003) | **Rejected** — still defer | PRD requirement exists but Lighthouse CI is disproportionate for a static site at current scale |
| #10 severity should be lower | **Accepted** — keep Medium but acknowledge the rewrite is more invasive than originally described |
| #17 should not be "document only" | **Accepted** — change to "experiment first, then document" |
| #18 fix doesn't address the issue statement | **Partially accepted** — content tests are still valuable; component/WASM tests require browser infrastructure |
| #19 should be higher than Low | **Rejected** — the PRD requirements are aspirational; current test coverage is appropriate for project stage |

---

## What's Already Done

- All 12 documentation pages updated from sparse stubs to comprehensive content matching source DOCS.md
- Architecture page: test count 308 → 424, added full crate structure, design decisions, error philosophy
- Commands page: added short flags, `config set-key`/`get-key`, usage patterns
- Configuration page: full TOML reference, config priority, environment variables
- LLM Providers page: fixed TOML syntax, added recommended models, thinking mode, secure key storage
- All other pages expanded with complete content from source

---

## Implementation Plan

### Summary: Prioritized Execution Order

| Phase | Issues | Effort | Impact | Priority |
| --- | --- | --- | --- | --- |
| 0 | **#28**, #1, #2, #3, #4 | 3 hours | Prevents broken deploys on PRs | **Critical** |
| 1 | #6, #6b, #7, #29 | 4-6 hours | Prevents runtime panics + XSS hardening | High |
| 2 | #30, #13, #10, #11 | 6-10 hours | Build correctness + parser bugs | Medium |
| 3 | #31, #32, #33 | 3-4 hours | Accessibility compliance | Medium |
| 4 | #15, #16, #34, #36 | 2 hours | Asset completeness + build robustness | Low |
| 5 | #17 | 2 hours | Security — CSP experiment + document | Low |
| 6 | #20, #18 | 2-4 hours | Testing improvements | Low |
| 7 | #21 | 2-3 hours | Code organization | Lowest |

### Explicitly Deferred or Skipped

| Issue | Action | Reason |
| --- | --- | --- |
| #5 | Defer | Lighthouse CI is disproportionate for static site at current scale; manual testing sufficient |
| #8 | **Remove** | Replacing `unwrap()` with `expect()` helper doesn't materially improve safety |
| #9 | Document only | `Closure::forget()` is correct for long-lived listeners in Leptos islands |
| #12 | Skip | Build script `unwrap()` is appropriate; panics are diagnostic |
| #14 | Defer | Requires design assets (1200x630 OG images), not code |
| #19 | Partial (TR-001 only) | TR-002 through TR-005 need browser testing infrastructure |
| #22 | Document only | IntersectionObserver is effectively idle after reveal |
| #23 | **Remove** | Both auditors agree this is not a real issue |
| #35 | Document | Note in CLAUDE.md that build.rs classes need explicit CSS rules in tailwind.css (current workaround already in place) |

---

### Phase 0: CI Infrastructure (Issues #28, #1-4) — 3 hours

The highest-leverage improvement. Create a proper CI workflow that runs on all PRs, then add validation gates.

**Issue #28: Create `ci.yml` PR validation workflow** [S]

File: `.github/workflows/ci.yml` (new file)

```yaml
name: CI

on:
  push:
    branches: [development]
  pull_request:
    branches: [development]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@nightly
        with:
          targets: wasm32-unknown-unknown

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

      - name: Check formatting
        run: cargo fmt --check

      - name: Validate content
        run: cargo test --test content_check

      - name: Lint (SSR)
        run: cargo clippy --features ssr -- -D warnings

      - name: Lint (WASM)
        run: cargo clippy --features hydrate --target wasm32-unknown-unknown -- -D warnings
```

This runs on both push and PR. The deploy workflow (`deploy.yml`) remains separate and only handles build + deploy on push.

**Issue #1: Content validation** — covered by `ci.yml` step above.

**Issue #2: Clippy** — covered by `ci.yml` steps above (both SSR and WASM targets).

**Issue #3: Formatting check** — covered by `ci.yml` step above.

**Issue #4: Add `cargo audit` to deploy workflow** [S]

Add to `.github/workflows/deploy.yml` before the build step:

```yaml
- name: Install cargo-audit
  run: cargo install cargo-audit
- name: Audit dependencies
  run: cargo audit
```

Note: Only in deploy workflow since audit failures should block deployment but not necessarily block PR merging (some advisories are informational). Consider `cargo binstall cargo-audit` to speed up CI.

---

### Phase 1: WASM Runtime Safety + XSS Hardening (Issues #6, #6b, #7, #29) — 4-6 hours

**Issue #6: Fix `theme_toggle.rs` double-unwrap** [S]

File: `src/components/theme_toggle.rs`

Lines 14-15: `window.local_storage().unwrap().unwrap()`. Line 38: same pattern.

Fix: Replace with graceful fallback:

```rust
// Before (line 14-15)
let window = web_sys::window().unwrap();
let storage = window.local_storage().unwrap().unwrap();

// After
let storage = web_sys::window()
    .and_then(|w| w.local_storage().ok())
    .flatten();
```

When `storage` is `None`, fall through to `match_media` system preference check (existing fallback). In the `toggle` closure (line 38), use `if let Some(storage)` — theme toggles visually but is not persisted.

**Issue #6b (NEW): Guard inline theme bootstrap script in `app.rs`** [S]

File: `src/app.rs`, lines 65-71

The inline script does `localStorage.getItem('theme')` without guarding against localStorage being unavailable (same scenario as #6 — Safari ITP, private browsing).

Fix: Wrap in try-catch:

```javascript
(function(){
    try {
        var t = localStorage.getItem('theme');
        if (t === 'dark' || (!t && matchMedia('(prefers-color-scheme:dark)').matches)) {
            document.documentElement.classList.add('dark');
        }
    } catch(e) {
        if (matchMedia('(prefers-color-scheme:dark)').matches) {
            document.documentElement.classList.add('dark');
        }
    }
})();
```

**Issue #7: Fix `doc_search.rs` `resp.text().unwrap()` panic + add status check** [S]

File: `src/components/doc_search.rs`, line 117-119

Fix: Check response status before reading body:

```rust
// Before
if let Ok(resp) = resp {
    if let Ok(json) = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap()).await

// After
if let Ok(resp) = resp {
    if resp.ok()
        && let Ok(text_promise) = resp.text()
        && let Ok(json) = wasm_bindgen_futures::JsFuture::from(text_promise).await
        && let Some(text) = json.as_string()
        && let Ok(entries) = serde_json::from_str::<Vec<SearchEntry>>(&text)
    {
        set_index.set(Some(entries));
    }
}
```

This adds: (1) status check via `resp.ok()`, (2) guarded `text()` call, (3) chained validation.

**Issue #29: XSS hardening — docs HTML + CSP** [M]

File: `src/app.rs:58`, `build.rs:175,233`, `src/pages/docs.rs:48`

The raw HTML pipeline: `build.rs` renders markdown to HTML → `generate_rust_module()` embeds as `r##"..."##` → `docs.rs` injects via `inner_html=page.html_content` → CSP allows `unsafe-inline`.

Since docs content is authored at build time (not user input), the practical risk is low. However, combined with the permissive CSP, any malicious content in a markdown file would execute.

Mitigation strategy (in order of impact):
1. Add `html-sanitize` or `ammonia` in `build.rs` to strip `<script>`, `onclick`, `onerror` etc. from rendered HTML before embedding
2. Alternatively, tighten CSP to remove `unsafe-inline` for scripts (requires hash-based approach for the 3 known inline scripts + Leptos hydration)
3. Minimum: add a build-time test that verifies no `<script>` tags exist in rendered doc HTML

---

### Phase 2: Build Pipeline Hardening (Issues #30, #13, #10, #11) — 6-10 hours

**Issue #30: Fix frontmatter parsing — delimiter matching** [S]

File: `build.rs:124-126`, `tests/content_check.rs:25-27`

Current code: `after_first.find("---")` matches `---` anywhere in the remaining content, including inside YAML values. If a YAML value contains `---` (e.g., a description with an em-dash separator), the frontmatter would be truncated.

Fix: Match `---` on its own line, not embedded in text:

```rust
// Before
let end = after_first.find("---").expect("Missing closing frontmatter delimiter");

// After
let end = after_first
    .find("\n---\n")
    .or_else(|| after_first.find("\n---\r\n"))
    .or_else(|| {
        // Also handle --- at end of string (no trailing newline)
        after_first
            .find("\n---")
            .filter(|&i| i + 4 >= after_first.len())
    })
    .expect("Missing closing frontmatter delimiter");
```

Same fix needed in `tests/content_check.rs:25-27`.

**Issue #13: Check HTTP status in prerender** [S]

File: `src/bin/prerender.rs`, function `http_get()`

Parse the numeric status code from the HTTP response line:

```rust
let status_line = response.lines().next().unwrap_or("");
let status_code: u16 = status_line
    .split_whitespace()
    .nth(1)
    .and_then(|c| c.parse().ok())
    .unwrap_or(0);
if status_code != 200 {
    return Err(format!("HTTP {status_code} for {path}: {status_line}").into());
}
```

**Issue #10: Replace string-based `add_heading_ids()` in build.rs** [M]

File: `build.rs`, function `add_heading_ids()`

Current implementation uses string `find()` and `strip_prefix()` to locate `<hN>` tags — fragile. Rewrite to inject heading IDs during the pulldown-cmark event stream processing inside `render_markdown_with_syntax_highlighting()`.

Strategy: When encountering `Event::Start(Tag::Heading { level, .. })`, track heading text content. On `Event::End(TagEnd::Heading(_))`, emit the full `<hN id="slug">` tag. This eliminates `add_heading_ids()` entirely.

Note: This is more invasive than originally described — requires buffering heading content before emitting the opening tag. The `extract_headings()` function already does this, so the logic can be shared.

**Issue #11: Improve escaping in `generate_rust_module()` for all fields** [S]

File: `build.rs`, function `generate_rust_module()`

The fix applies to `title`, `description`, and heading text — not just headings. All three are emitted as normal string literals with only `"` to `\"` escaping.

Fix: Use raw strings for all text fields:

```rust
// Before (for title, description, headings)
writeln!(code, "        title: \"{}\",", page.title.replace('"', "\\\"")).unwrap();

// After
writeln!(code, "        title: r#\"{}\"#,", page.title).unwrap();
```

Raw strings `r#"..."#` don't need quote escaping. The only edge case is if content contains `"#` which is essentially impossible in headings/titles/descriptions. Apply to all three fields.

---

### Phase 3: Accessibility (Issues #31, #32, #33) — 3-4 hours

**Issue #31: Search modal dialog semantics** [M]

File: `src/components/doc_search.rs`

Add to the search modal container:
- `role="dialog"` attribute
- `aria-modal="true"` attribute
- `aria-label="Search documentation"` attribute
- Focus trap: on Tab/Shift+Tab when modal is open, wrap focus within the modal
- On close: restore focus to the search trigger button

**Issue #32: Mobile menu dialog semantics** [M]

File: `src/components/mobile_menu.rs`

Add to the menu overlay:
- `role="dialog"` attribute
- `aria-modal="true"` attribute
- `aria-label="Navigation menu"` attribute
- Scroll lock: set `document.body.style.overflow = "hidden"` when open, restore on close
- Focus trap: prevent Tab from escaping into background content

**Issue #33: Copy buttons keyboard visibility** [S]

File: `style/tailwind.css` (or inline in `build.rs:213` and `src/pages/landing.rs:331`)

Current: `opacity-0 group-hover:opacity-100` — invisible to keyboard users who Tab to the button.

Fix: Add `focus:opacity-100` alongside `group-hover:opacity-100`:

```
opacity-0 group-hover:opacity-100 focus:opacity-100
```

This makes the button visible when focused via keyboard navigation.

---

### Phase 4: Build Robustness + Assets (Issues #34, #36, #15, #16) — 2 hours

**Issue #34: Ensure search index is always written** [S]

File: `build.rs:464-466`

Current code only writes `search_index.json` to `target/site/` if that directory exists. First build creates `target/site/` later, so the search index can be missing.

Fix: Create `target/site/` directory before writing:

```rust
let site_dir = Path::new("target/site");
fs::create_dir_all(site_dir).unwrap();
fs::write(site_dir.join("search_index.json"), &json).unwrap();
```

**Issue #36: Prerender route discovery robustness** [S]

File: `src/bin/prerender.rs:60,205`

Current code recursively searches for the first `routes.txt` under `target/`. This can pick up a stale artifact from a previous build.

Fix: Use the `OUT_DIR` environment variable (set by cargo for the commitbee-web crate) or the known build output path:

```rust
// Instead of recursive search, use known location
let routes_file = find_routes_file();
// Prefer target/build/commitbee-web-*/out/routes.txt (most recent)
```

Or simpler: sort found paths by modification time and use the newest.

**Issue #15: Add CNAME file** [S]

Create `public/CNAME` with the content:
```
commitbee.dev
```

Prerender copies `public/` to `dist/`, so this deploys automatically.

**Issue #16: Add apple-touch-icon and favicon.ico** [S]

Create:
- `public/images/apple-touch-icon.png` — 180x180 PNG
- `public/images/favicon.ico` — multi-size ICO (16x16, 32x32, 48x48)

Update `src/app.rs` `<head>` section:
```html
<link rel="apple-touch-icon" href="/images/apple-touch-icon.png"/>
<link rel="icon" href="/images/favicon.ico" sizes="any"/>
```

Keep existing SVG favicon link (Safari/Chrome support it).

---

### Phase 5: CSP Experiment + Documentation (Issue #17) — 2 hours

**Issue #17: CSP hardening — experiment first, then document** [M]

File: `src/app.rs:58`

Current CSP:
```
default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; font-src 'self'; img-src 'self' data:; connect-src 'self'
```

**Step 1: Test without `unsafe-eval`**

Build and test the site with `unsafe-eval` removed from `script-src`. Codex flagged that this may not actually be required by Leptos 0.8 — WASM instantiation doesn't necessarily need eval. If the site works (hydration, islands, theme toggle, search, pipeline demo), remove it permanently.

**Step 2: If `unsafe-eval` is required**

Document why in a comment in `app.rs` and CLAUDE.md.

**Step 3: Evaluate `unsafe-inline` reduction**

- The 3 inline scripts in `app.rs` could use hash-based CSP: compute SHA-256 of each script's content, add to `script-src`
- Leptos hydration scripts are dynamically generated — test whether their content is deterministic between builds
- If deterministic: use hash-based CSP for all inline scripts
- If not: document `unsafe-inline` as a Leptos framework limitation

**Step 4: `unsafe-inline` for styles**

Tailwind CSS utility classes are injected via `<style>` tags and `style` attributes. Removing `unsafe-inline` from `style-src` would break styling. Document as a Tailwind limitation.

---

### Phase 6: Testing Improvements (Issues #20, #18) — 2-4 hours

**Issue #20: Add internal link validation test** [S]

File: `tests/content_check.rs`

Add a test that:
1. Reads all markdown files in `content/docs/`
2. Extracts all `[text](/docs/slug)` and `[text](slug)` style internal links via regex
3. Verifies each linked slug corresponds to an actual `.md` file

**Issue #18: Expand content validation tests** [S]

File: `tests/content_check.rs`

Add tests:
1. **`order_values_are_unique_within_section()`** — no two docs in same section share an `order` value
2. **`description_length_reasonable()`** — descriptions are 10-200 characters (catches empty or paste-error descriptions)
3. **`frontmatter_delimiter_on_own_line()`** — verify closing `---` is on its own line (complements fix for #30)

---

### Phase 7: Code Organization (Issue #21) — 2-3 hours

**Issue #21: Split `landing.rs` into section modules** [M]

File: `src/pages/landing.rs` (415 lines)

Split into:
- `src/pages/landing/mod.rs` — `Landing` component, re-exports, `CodeBlockActivator` island render
- `src/pages/landing/hero.rs` — `HeroSection`
- `src/pages/landing/problem.rs` — `ProblemSection`
- `src/pages/landing/pipeline.rs` — `PipelineSection`
- `src/pages/landing/differentiators.rs` — `DifferentiatorsSection`, `FeatureCard`
- `src/pages/landing/comparison.rs` — `ComparisonSection`, `ComparisonFeature`
- `src/pages/landing/install.rs` — `InstallSection`
- `src/pages/landing/docs_transition.rs` — `DocsTransitionSection`, `DocLink`

Each section file is self-contained. Best done when no other changes are pending in `landing.rs`.

---

### Critical Files for Implementation

| File | Issues | Phase |
| --- | --- | --- |
| `.github/workflows/ci.yml` (new) | #28, #1, #2, #3 | 0 |
| `.github/workflows/deploy.yml` | #4 | 0 |
| `src/components/theme_toggle.rs` | #6 | 1 |
| `src/app.rs` | #6b, #17, #29 | 1, 5 |
| `src/components/doc_search.rs` | #7, #31 | 1, 3 |
| `build.rs` | #10, #11, #29, #30, #34 | 1, 2 |
| `src/bin/prerender.rs` | #13, #36 | 2, 4 |
| `tests/content_check.rs` | #18, #20, #30 | 2, 6 |
| `src/components/mobile_menu.rs` | #32 | 3 |
| `style/tailwind.css` | #33 | 3 |
| `public/` | #15, #16 | 4 |
| `src/pages/landing.rs` | #21 | 7 |

---

## Audit Sources

| Auditor | Model | Mode | Scope |
| --- | --- | --- | --- |
| Claude Opus 4.6 | claude-opus-4-6 | Full codebase read + plan creation | All `.rs`, `build.rs`, CSS, CI, content, assets |
| Codex gpt-5.4 (pass 1) | gpt-5.4 xhigh | Independent review of TMP.md plan | Verified each issue against codebase, assessed fixes |
| Codex gpt-5.4 (pass 2) | gpt-5.4 xhigh | Full independent codebase audit | All `.rs`, `build.rs`, CSS, CI, tests — 16 findings |
