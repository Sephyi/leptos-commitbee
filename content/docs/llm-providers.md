---
title: "LLM Providers"
order: 3
section: "Usage"
description: "Configure Ollama, OpenAI, and Anthropic providers for commit message generation"
---

# LLM Providers

CommitBee supports three LLM providers:

## Ollama (Default)

Local-first. Your code never leaves your machine.

```toml
[provider]
name = "ollama"
model = "qwen3.5:4b"
```

## OpenAI

```toml
[provider]
name = "openai"
model = "gpt-4o-mini"
api_key_env = "OPENAI_API_KEY"
```

## Anthropic

```toml
[provider]
name = "anthropic"
model = "claude-sonnet-4-20250514"
api_key_env = "ANTHROPIC_API_KEY"
```
