---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Supported Languages"
order: 3
section: "Internals"
description: "10 languages supported by tree-sitter semantic analysis"
---

# Supported Languages

CommitBee uses tree-sitter to parse source files and extract semantic symbols. All 10 languages are enabled by default and individually toggleable via Cargo feature flags.

| Language | Feature Flag | What It Extracts |
| --- | --- | --- |
| Rust | `lang-rust` | Functions, structs, enums, impls, traits, methods |
| TypeScript | `lang-typescript` | Functions, classes, interfaces, methods, types |
| JavaScript | `lang-javascript` | Functions, classes, methods, arrow functions |
| Python | `lang-python` | Functions, classes, methods, decorators |
| Go | `lang-go` | Functions, types, methods, interfaces |
| Java | `lang-java` | Classes, methods, constructors, interfaces, enums |
| C | `lang-c` | Functions, structs, enums, typedefs |
| C++ | `lang-cpp` | Functions, classes, structs, enums, methods |
| Ruby | `lang-ruby` | Classes, modules, methods, singleton methods |
| C# | `lang-csharp` | Classes, methods, constructors, interfaces, enums |

## Custom Language Builds

To build with only the languages you need (reduces binary size):

```bash
# Only Rust and TypeScript support
cargo install commitbee --no-default-features --features lang-rust,lang-typescript

# All languages except C++ and C#
cargo install commitbee --no-default-features --features lang-rust,lang-typescript,lang-javascript,lang-python,lang-go,lang-java,lang-c,lang-ruby
```

**Files in unsupported or disabled languages still work** — they're included in the diff context, they just don't get semantic symbol extraction. The commit message will still be based on the actual diff content; it just won't know which specific functions or types changed.

## Symbol Tracking

For supported languages, symbols are tracked in three states:

- **Added** `[+]` — New function, struct, class, etc.
- **Removed** `[-]` — Deleted symbol
- **Modified (signature changed)** `[~]` — Symbol exists in both versions but its signature changed

Modified symbols include additional annotations: `[docs only]` when only documentation/comments changed, `[docs + code]` when both documentation and code changed. Methods show their parent scope (e.g., `CommitValidator::validate` rather than just `validate`).

This information appears in the prompt as a `SYMBOLS CHANGED` section. When structural AST diffs are available, a separate `STRUCTURED CHANGES` section provides precise details like `+param timeout`, `return Result<()> -> Result<Error>`, or `+field name`.
