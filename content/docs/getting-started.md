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

## Install

```bash
cargo install commitbee
```

Or build from source:

```bash
git clone https://github.com/sephyi/commitbee.git
cd commitbee
cargo build --release
# Binary at ./target/release/commitbee
```

## Requirements

- **Rust 1.94+** (edition 2024)
- **Ollama** running locally — [ollama.ai](https://ollama.ai)
- A model pulled: `ollama pull qwen3.5:4b`

## First Run

```bash
# Stage something
git add src/my_change.rs

# Generate a commit message
commitbee
```

That's it. Zero configuration needed if Ollama is running with `qwen3.5:4b`.

CommitBee will analyze your staged changes, extract semantic information via tree-sitter, send a structured prompt to the LLM, validate the output, and present you with a commit message to approve.

## Quick Config

Want to customize things? Create a config file:

```bash
commitbee init
```

This creates a config at your platform's standard location (run `commitbee doctor` to see where). Edit it to change the model, provider, or formatting preferences.
