---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

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

### Homebrew

```bash
brew install sephyi/tap/commitbee
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
