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
