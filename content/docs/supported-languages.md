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

CommitBee uses tree-sitter to parse these languages:

**Rust, TypeScript, JavaScript, Python, Go, Java, C, C++, Ruby, C#**

All enabled by default, individually toggleable via Cargo feature flags. Files in other languages still get full diff context, just without symbol extraction.
