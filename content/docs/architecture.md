---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Architecture"
order: 1
section: "Internals"
description: "Deep dive into commitbee's internal architecture and design decisions"
---

# Architecture

CommitBee is ~18K lines of Rust compiled to a single static binary with LTO.

## Crate Structure

```txt
src/
├── main.rs              # Entry point, tracing setup
├── lib.rs               # Library exports (for integration tests)
├── app.rs               # Application orchestrator (all the glue)
├── cli.rs               # CLI argument parsing (clap derive)
├── config.rs            # Configuration loading (figment layered)
├── error.rs             # Error types (thiserror + miette diagnostics)
├── domain/
│   ├── change.rs        # FileChange, StagedChanges, ChangeStatus
│   ├── symbol.rs        # CodeSymbol, SymbolKind, SpanChangeKind
│   ├── diff.rs          # SymbolDiff, ChangeDetail (structural AST diffs)
│   ├── context.rs       # PromptContext — assembles the LLM prompt
│   └── commit.rs        # CommitType enum (single source of truth)
└── services/
    ├── git.rs           # GitService — gix for discovery, git CLI for diffs
    ├── analyzer.rs      # AnalyzerService — tree-sitter parsing via rayon
    ├── context.rs       # ContextBuilder — evidence flags, token budget
    ├── differ.rs        # AstDiffer — structural comparison of old/new symbols
    ├── safety.rs        # Secret scanning (24 patterns), conflict detection
    ├── sanitizer.rs     # CommitSanitizer + CommitValidator
    ├── splitter.rs      # CommitSplitter — diff-shape + Jaccard clustering
    ├── progress.rs      # Progress indicators (indicatif spinners, TTY-aware)
    └── llm/
        ├── mod.rs       # LlmBackend enum dispatch, SYSTEM_PROMPT
        ├── ollama.rs    # OllamaProvider — streaming NDJSON
        ├── openai.rs    # OpenAiProvider — SSE streaming
        └── anthropic.rs # AnthropicProvider — SSE streaming
```

## Key Design Decisions

**Hybrid Git** — `gix` (pure Rust) is used for fast repo discovery, but the git CLI is used for diffs and staging operations.  
This avoids the complexity of reimplementing diff parsing in pure Rust while keeping startup fast.

**Full File Parsing** — Tree-sitter parses the complete staged and HEAD versions of files, not just the diff hunks.  
Diff hunks are then mapped to symbol spans. This means CommitBee knows the full context of what changed, not just the changed lines.

**Enum Dispatch** — The LLM provider uses an enum (`LlmBackend`) rather than a trait object.  
This avoids `async-trait` overhead and the complexity of `dyn` dispatch for async methods.

**Streaming with Cancellation** — All providers support Ctrl+C cancellation via `tokio_util::CancellationToken`.  
The streaming display runs in a separate tokio task with `tokio::select!` for responsive cancellation.

**Token Budget** — The context builder tracks character usage (~4 chars per token) and truncates the diff if it exceeds the budget, prioritizing the most important files.  
The budget adapts based on available information: when structural AST diffs are present, the symbol allocation shrinks (20%) since the diffs carry precise detail; when only  
signatures are available, symbols get 30%. The default 24K char budget (~6K tokens) is safe for 8K context models.

**Single Source of Truth for Types** — `CommitType::ALL` is a const array that defines all valid commit types.  
The system prompt's type list is verified at compile time (via a `#[test]`) to match this array exactly.

## Error Philosophy

Every error in CommitBee is:

- **Actionable** — Tells you what went wrong and how to fix it (via `miette` help messages)
- **Typed** — Uses `thiserror` for structured error variants, not string errors
- **Diagnostic** — Error codes like `commitbee::git::no_staged` for programmatic handling

No panics in user-facing code paths. The sanitizer and validator are tested with proptest to ensure they never panic on arbitrary input.

## Testing Strategy

CommitBee has 424 tests across multiple strategies:

| Strategy | What It Covers |
| --- | --- |
| Unit tests | Individual functions (sanitizer rules, type parsing, config defaults) |
| Snapshot tests (insta) | Output format stability |
| Property tests (proptest) | Never-panic guarantees for parsers |
| Integration tests (wiremock) | Full provider round-trips with mocked HTTP |
| Git fixture tests | Real git operations in temp directories |

Run them:

```bash
cargo test                    # All 424 tests
cargo test --test sanitizer   # Just sanitizer tests
cargo test --test integration # LLM provider mocks
COMMITBEE_LOG=debug cargo test -- --nocapture  # With logging
```
