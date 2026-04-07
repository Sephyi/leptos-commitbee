---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "LLM Providers"
order: 2
section: "Usage"
description: "Configure Ollama, OpenAI, and Anthropic providers for commit message generation"
---

# LLM Providers

CommitBee supports three providers. All use streaming for responsive output.

## Ollama (default, local)

The recommended setup. Your code never leaves your machine.

```toml
provider = "ollama"
model = "qwen3.5:4b"
ollama_host = "http://localhost:11434"
```

**Recommended models:**

| Model | Size | Notes |
| --- | --- | --- |
| `qwen3.5:4b` | 3.4 GB | Default. Fast, clean JSON output |
| `llama3:8b` | 4.7 GB | Good quality, slower |
| `codellama:7b` | 3.8 GB | Code-focused alternative |

**Thinking mode**: Some models (like `qwen3:4b`) have built-in reasoning that produces thinking blocks before their response. CommitBee can handle these — set `think = true` in your config and bump `num_predict` to `8192` or higher to give the model room for both thinking and output tokens. The default model `qwen3.5:4b` doesn't need this.

## OpenAI

```toml
provider = "openai"
model = "gpt-4o-mini"
api_key = "sk-..."
```

Or use environment variables:

```bash
export COMMITBEE_PROVIDER=openai
export COMMITBEE_MODEL=gpt-4o-mini
export OPENAI_API_KEY=sk-...
```

Works with any OpenAI-compatible API. Set `openai_base_url` for custom endpoints:

```toml
openai_base_url = "https://api.together.xyz/v1"
```

## Anthropic

```toml
provider = "anthropic"
model = "claude-sonnet-4-20250514"
api_key = "sk-ant-..."
```

Or:

```bash
export COMMITBEE_PROVIDER=anthropic
export ANTHROPIC_API_KEY=sk-ant-...
```

## Secure Key Storage

API keys are stored as `secrecy::SecretString` — memory is zeroed on drop and keys show as `[REDACTED]` in debug output. Keys are only exposed at the HTTP header insertion point.

The `secure-storage` feature (enabled by default) stores keys in your OS keychain using platform-native backends (macOS Keychain, Windows Credential Manager, Linux Secret Service):

```bash
commitbee config set-key openai      # Prompts for key, stores in keychain
commitbee config set-key anthropic   # Same for Anthropic
commitbee config get-key openai      # Check if key exists
```

Key lookup order: CLI `--provider` flag -> config file -> environment variable -> keychain. The `set-key` and `get-key` commands do not require an API key to already be configured.

To build without keychain support:

```bash
cargo install commitbee --no-default-features --features all-languages
```
