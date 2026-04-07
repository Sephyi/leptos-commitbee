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

CommitBee supports three primary providers out of the box. All use streaming for responsive output.

> **⚠️ Provider Support:** Native integrations are currently limited to the providers listed below.  
> For others not yet natively supported, a proxy is the recommended approach until official support lands.

## Secure Key Storage (Recommended)

When working with cloud providers (OpenAI or Anthropic), **never** store API keys as plain text in `config.toml`.

The `secure-storage` feature (enabled by default) stores keys directly in your OS keychain via  
platform-native backends (macOS Keychain, Windows Credential Manager, Linux Secret Service):

```bash
commitbee config set-key openai      # Prompts for your OpenAI key
commitbee config set-key anthropic   # Prompts for your Anthropic key
commitbee config get-key openai      # Check if a key is stored
```

**Key lookup order:** CLI `--provider` flag → config file → environment variable → keychain.

Keys are held as `secrecy::SecretString` — memory is zeroed on drop and they appear as `[REDACTED]` in debug logs.

To opt out and build without keychain support, compile from source:

```bash
cargo install commitbee --no-default-features --features all-languages
```

## Ollama (default, local)

The default local setup. Your code never leaves your machine.

```toml
provider = "ollama"
model = "qwen3.5:4b"
ollama_host = "http://localhost:11434"
```

**Recommended models:**

| Model | Size | Notes |
|---|---|---|
| `qwen3.5:4b` | 3.4 GB | Default — fast, clean JSON output |

**Thinking mode:** Some models (e.g. `qwen3.5:4b`) support built-in reasoning, producing thinking blocks before their final response.  
To enable this, set `think = true` in your config and raise `num_predict` to `8192` or higher. The default `qwen3.5:4b` configuration does not require this.

## OpenAI & Compatible Proxies

Ensure your key is stored securely before use:

```bash
commitbee config set-key openai
```

```toml
provider = "openai"
model = "gpt-4o-mini"
```

Or via environment variables:

```bash
export COMMITBEE_PROVIDER=openai
export COMMITBEE_MODEL=gpt-4o-mini
export OPENAI_API_KEY=sk-...
```

**OpenAI-Compatible Providers:**

Services like Groq, LM Studio, and vLLM work out of the box — just set `openai_base_url` to the provider's endpoint.  
For broader compatibility, a local proxy such as [Bifrost](https://github.com/maximhq/bifrost) can bridge providers without native OpenAI-compatible APIs.  

```toml
openai_base_url = "https://localhost:8080/v1"
```

## Anthropic

Ensure your key is stored securely before use:
```bash
commitbee config set-key anthropic
```
```toml
provider = "anthropic"
model = "claude-sonnet-4-20250514"
```

Or via environment variables:
```bash
export COMMITBEE_PROVIDER=anthropic
export ANTHROPIC_API_KEY=sk-ant-...
```
