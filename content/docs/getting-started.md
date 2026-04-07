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

**Stable Release**
```bash
cargo install commitbee
```

**Development Version**

If you want access to the latest features weeks early (warning: can be unstable, no guarantees):
```bash
cargo install --git https://github.com/Sephyi/commitbee --branch development commitbee
```

**Or build it yourself from source:**

```bash
git clone https://github.com/sephyi/commitbee.git
cd commitbee
cargo build --release
# Binary at ./target/release/commitbee
```

## Requirements

- **Rust 1.94+** (edition 2024)
- **An LLM Provider**:
  - **Local (Default):** [Ollama](https://ollama.ai) running locally with a model pulled (e.g., `ollama pull qwen3.5:4b`).
  - **Cloud:** An API key for Anthropic or OpenAI (see [LLM Providers](llm-providers)).

> More providers will be added natively in the future. For now, you can hook into almost any other LLM (like Gemini, Mistral, or custom  
> endpoints) by using the `openai` provider and setting the `openai_base_url` in your config to point to an OpenAI-compatible proxy.

## First Run

The most important concept in CommitBee is that **you must always stage your files first**. CommitBee only looks at your staging area,  
giving you complete control over which files are included or ignored in the generated commit message.

```bash
# Explicitly stage the files you want
git add src/my_change.rs

# Generate a commit message from staged context
commitbee
```

That's it. Zero configuration needed if Ollama is running with `qwen3.5:4b`.

CommitBee will analyze your staged changes, extract semantic information via tree-sitter, send a structured prompt to the LLM, validate  
the output, and present you with a commit message to approve.

## Quick Config

Want to customize things? Create a config file:

```bash
commitbee init
```

This creates a config at your platform's standard location (run `commitbee doctor` to see where). Edit it to change the model, provider, or formatting preferences.

---

**🔥 Next Step:** Ready to level up? Check out the [Recommended Setup](recommended-setup) guide for the official configuration we use for the highest quality commit messages.
