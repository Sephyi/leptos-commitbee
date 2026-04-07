---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "How It Works"
order: 3
section: "Basics"
description: "Understand commitbee's 7-stage pipeline from diff to commit message"
---

# How It Works

Most commit message generators dump `git diff` output into an LLM and hope for the best. CommitBee takes a fundamentally different approach.

## The Pipeline

```txt
Stage Changes -> Git Service -> Tree-sitter -> Splitter -> Context Builder -> LLM -> Validator -> Sanitizer
```

Here's what each step actually does:

### 1. Git Service
Reads your staged changes using `gix` for repo discovery and the `git` CLI for diffs. 
- Paths are parsed with NUL-delimited output (`-z` flag) so filenames with spaces or special characters work correctly.

### 2. Tree-sitter Analyzer
Parses both the staged version and the HEAD version of every changed file — in parallel, using `rayon` across CPU cores. It extracts:
- **Full signatures** (e.g., `pub fn connect(host: &str, timeout: Duration) -> Result<Connection>`) by taking the definition node text before the body child. 
- **Parent scope** (enclosing impl, class, or trait — e.g., `CommitValidator::validate`). 
- **Structural AST diffs** that describe exactly what changed in modified symbols (parameters added/removed, return type changed, visibility changed, semantic markers like `unsafe`, `derive`, decorators, `export`, mutability, generic constraints, etc.). 
- **Cross-file connections** (caller+callee both changed). 
- **Doc-vs-code distinction:** Symbols are tracked in three states (added, removed, or modified-signature) indicating whether changes were documentation-only, code-only, or mixed.

### 3. Commit Splitter
Looks at your staged changes and decides whether they contain logically independent work. It groups files using:
- **Diff-shape fingerprinting** (what kind of changes — additions, deletions, modifications).
- **Jaccard similarity** on content vocabulary.
If it finds multiple concerns, it offers to split them into separate commits.

### 4. Context Builder
Assembles a budget-aware prompt within the token limit (~6K tokens).
- Classifies modified symbols as whitespace-only or semantic (via character-stream comparison).
- Computes **evidence flags** (mechanical change? public APIs removed? bug-fix evidence?).
- Detects **change intent** (error handling, test, logging, dependency update patterns) for the `INTENT:` prompt section.
- Detects cross-file connections, identifies import changes and test file correlations.
- Calculates the character budget for the subject line.
- **Adapts token budget:** When structural AST diffs are available, symbols get 20% of the budget (diffs carry more detail); when only signatures are available, symbols get 30%.

### 5. LLM Provider
Streams the prompt to your chosen model (Ollama, OpenAI, or Anthropic) and collects the response token by token.

### 6. Validator
Checks the LLM's output against the evidence flags. If the model says "fix" but there's no bug-fix evidence in the code, or if the subject  
is too long, or if it used generic wording — the validator catches it and retries with targeted correction instructions. Up to 3 attempts.

### 7. Sanitizer
Does the final cleanup: 
- Extracts JSON from potentially noisy LLM output (thinking blocks, code fences, conversational preambles).
- Validates the conventional commit format.
- Wraps the body at 72 characters, and constructs the final commit message string.

## What Makes the Prompt Special

CommitBee doesn't just send a diff. The prompt includes:

- **File summary** with per-file line counts (`+additions -deletions`)
- **Suggested commit type** inferred from code analysis (not guessed)
- **Evidence flags** telling the LLM deterministic facts about the change
- **Symbol changes with full signatures** — `[+] pub fn connect(host: &str) -> Result<()>`, not just "Function connect"
- **Signature diffs** — `[~] old_sig -> new_sig` for modified symbols
- **Structured AST diffs** — `CommitValidator::validate(): +param timeout, return Result<()> -> Result<Error>` (precise semantic changes from AST comparison)
- **Import changes** — `analyzer: added use crate::domain::DiffHunk` (tracked per file)
- **Test file correlations** — `src/services/context.rs <-> tests/context.rs (test file)`
- **Doc-vs-code annotations** — modified symbols tagged `[docs only]` or `[docs + code]`
- **Cross-file connections** — `validator calls parse() — both changed`
- **Primary change detection** — which file has the most significant changes
- **Change intent** — detected patterns like error handling, test additions, logging, or dependency updates
- **Constraints** — rules the LLM must follow based on evidence
- **Character budget** — exact number of chars available for the subject line

All of this is computed before the LLM ever sees the diff. The model gets to focus on writing a good commit message rather than doing code analysis.
