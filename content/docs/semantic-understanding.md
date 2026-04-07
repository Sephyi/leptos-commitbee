---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Deep Semantic Understanding"
order: 2
section: "Internals"
description: "How CommitBee understands your code via Tree-sitter AST and structural diffs."
---

# Deep Semantic Understanding

Most AI commit tools try to understand your changes by reading the raw `git diff`.  
This approach falls apart quickly: diffs lack context, lose track of enclosing scopes, and treat refactoring the same as bug fixes.

CommitBee uses a complete AST (Abstract Syntax Tree) engine powered by `tree-sitter`.  
By parsing both the old and new states of your source code, CommitBee achieves **Deep Semantic Understanding**—tracking exactly what changed and what it means.

## Structural AST Diffs

Instead of just knowing that line 42 changed, CommitBee computes exactly how the structure of your code was altered:

- **Full Signature Extraction:** It pulls complete function, struct, and trait signatures (e.g., `pub fn connect(host: &str) -> Result<Connection>`), truncating safe representations if they span multiple lines. 
- **Parent Scope Extraction:** It understands when a method belongs to a specific class or `impl` block, feeding the LLM contextual information like `Parent > signature`.
- **Signature Diffs:** It compares old and new tree-sitter nodes to detect precisely what changed, emitting structured diffs like `+param strict: bool, return bool → Result<()>`.

### Semantic Markers

CommitBee tracks semantic metadata about your changes from the AST itself:
- **Unsafe Added/Removed:** If you add an `unsafe` block in Rust, CommitBee flags it. This automatically triggers a validation rule requiring you to justify the safety invariants in the commit body.
- **Modifiers & Decorators:** Changes to mutability, generics, property visibility (`pub`), and decorators (`@Inject`) are identified natively without regex unreliability.

## Rich Change Classification

CommitBee categorizes changes far beyond "added" or "deleted".

### Doc-vs-Code Separation
By inspecting node kinds and character streams, CommitBee discriminates between whitespace, comments, and actual logic changes:
- *Whitespace-Only:* Automatically filtered out so the LLM doesn't waste tokens on indentation fixes.
- *Docs-Only:* Detected when changes only occur inside docstrings (e.g., `///`, `/**`, or `"""`). The classifier suggests `CommitType::Docs`.
- *Mixed vs Semantic:* Merged appropriately depending on the context.

## Cross-File & Intent Intelligence

CommitBee creates a graph of relationships across your staging area to synthesize high-level intent.

### Connection Detection
If `src/validator.rs` adds a call to `parse()` and `src/parser.rs` modifies `parse()`, CommitBee detects this cross-file linkage.  
The LLM receives a dedicated `CONNECTIONS:` context block demonstrating how the staged files interact, leading to cohesive, architectural commit messages rather than isolated file summaries.

### Test Correlation & Ratios
- **Test-to-Code Ratios:** If >80% of lines added are in files categorized as tests, CommitBee rigorously asserts the commit type as `test`, gracefully handling scenarios where test refactors touch marginal source code.
- **Coupled Files:** Staged source files are matched to their corresponding test files (e.g., `src/services/config.rs <-> tests/config.rs`).

### Intent Detection
By pattern-matching new AST additions, CommitBee infers the underlying reason for your commit:
- *Error Handling:* Identifies additions of `Result<>`, `.map_err()`, or `?` operators.
- *Performance/Logging:* Recognizes `tracing::info!()`, `console.log`, or performance instrumentation.
- *Dependency Updates:* Detects version bumps in manifest files.
