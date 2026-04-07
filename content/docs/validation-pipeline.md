---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Validation Pipeline"
order: 2
section: "Internals"
description: "The 7-rule validation pipeline that ensures commit message quality"
---

# Validation Pipeline

CommitBee doesn't blindly trust LLM output. Every generated message goes through a multi-stage validation pipeline.

## Stage 1: Evidence-Based Validation

Before the LLM generates anything, CommitBee computes five deterministic signals from your code:

| Signal | What It Detects |
| --- | --- |
| `is_mechanical` | Formatting-only changes (whitespace, import reordering) |
| `has_bug_evidence` | Bug-fix comments in the diff (`fix`, `bug`, `patch`) |
| `public_api_removed_count` | Removed public functions, structs, or traits |
| `has_new_public_api` | New public symbols added |
| `is_dependency_only` | All changes in dependency/config files |

After the LLM responds, the **CommitValidator** checks the output against these signals with 7 rules:

1. **Fix requires evidence** — `fix` type needs bug-fix comments in the diff, otherwise it should be `refactor`
2. **Breaking change detection** — If public APIs were removed, `breaking_change` must be set
3. **Anti-hallucination** — `breaking_change` must not copy internal field names from the prompt
4. **Mechanical = style** — Formatting-only changes can't be `feat` or `fix`
5. **Dependencies = chore** — Dependency-only changes must use `chore` type
6. **Subject specificity** — Rejects generic subjects like "update code" or "improve things"
7. **Subject length** — Rejects subjects that would produce a first line exceeding 72 characters

## Stage 2: Multi-Pass Retry

If any rules are violated, CommitBee appends a `CORRECTIONS` section to the prompt explaining what went wrong and re-prompts the LLM.  
It then **re-validates** the retry output. If violations persist, it retries again — up to 3 total attempts.

This is more sophisticated than a simple retry. Each attempt gets the full list of remaining violations, so the LLM can address them all at once.

## Stage 3: Sanitization

The final output goes through the sanitizer, which handles the messy reality of LLM output:

- **Thinking block removal** — Strips thinking blocks and `<thought>...</thought>` blocks (even unclosed ones)
- **Code fence extraction** — Finds JSON inside ` ```json ... ``` ` blocks
- **Preamble stripping** — Removes conversational text like "Here's the commit message:" before the actual content
- **JSON parsing** — Extracts structured commit data from the LLM's JSON response
- **Format validation** — Verifies the result is a valid conventional commit
- **Body wrapping** — Wraps body text at 72 characters, preserving paragraph breaks
- **First line enforcement** — Rejects messages where the first line exceeds 72 characters

If the sanitizer can't produce a valid commit message, you get a clear error explaining what went wrong — never a silently mangled message.
