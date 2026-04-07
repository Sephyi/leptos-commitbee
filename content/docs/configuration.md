---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Configuration"
order: 1
section: "Usage"
description: "Configure commitbee with TOML files, environment variables, and CLI flags"
---

# Configuration

## Config File

CommitBee uses platform-standard config directories:

| Platform | Path |
| --- | --- |
| macOS | `~/Library/Application\ Support/commitbee/config.toml` |
| Linux | `~/.config/commitbee/config.toml` |
| Windows | `%APPDATA%\commitbee\config\config.toml` |

Run `commitbee doctor` to see the exact path on your system.

## Full Config Reference

```toml
# LLM provider: ollama, openai, anthropic
provider = "ollama"

# Model name (for Ollama, use `ollama list` to see available)
model = "qwen3.5:4b"

# Ollama server URL
ollama_host = "http://localhost:11434"

# API key for cloud providers (OpenAI, Anthropic)
# [!CAUTION] Setting this in the config file or via environment variables is highly insecure!
# Use `commitbee config set-key <provider>` to store it securely in your OS keychain instead.
api_key = "sk-..."

# Maximum lines of diff to include in prompt (10-10000)
max_diff_lines = 500

# Maximum lines per file in diff (10-1000)
max_file_lines = 100

# Maximum context characters for LLM prompt (~4 chars per token)
# Default 24000 is safe for 8K context models
# Increase for larger models (e.g., 48000 for 16K context)
max_context_chars = 24000

# Request timeout in seconds (1-3600)
timeout_secs = 300

# LLM temperature (0.0-2.0). Lower = more deterministic
temperature = 0.3

# Maximum tokens to generate (default 256)
# Increase to 8192+ if using thinking models with think = true
num_predict = 256

# Enable thinking/reasoning for Ollama models (default: false)
# When enabled, models like qwen3 will reason before responding.
# Requires higher num_predict (8192+) to accommodate thinking tokens.
think = false

# Rename detection similarity threshold (0-100, default 70)
# Set to 0 to disable rename detection
rename_threshold = 70

# Custom secret patterns (regex). Added to the built-in patterns.
# custom_secret_patterns = ["CUSTOM_KEY_[a-zA-Z0-9]{32}"]

# Disable built-in secret patterns by name (case-insensitive).
# disabled_secret_patterns = ["Generic Secret (unquoted)"]

# Exclude files matching glob patterns from analysis and diff context.
# Excluded files are listed in output but not sent to the LLM.
# exclude_patterns = ["*.lock", "**/*.generated.*"]

# Commit message format options
[format]
# Include body/description in commit message
include_body = true

# Include scope in commit type, e.g., feat(scope): subject
include_scope = true

# Enforce lowercase first character of subject
lowercase_subject = true
```

## Config Priority

Configuration is layered (highest priority wins):

1. **CLI flags** — `--provider`, `--model`, `--no-scope`
2. **Environment variables** — `COMMITBEE_PROVIDER`, `COMMITBEE_MODEL`, etc.
3. **User config** — `config.toml` at the platform path
4. **Project config** — `.commitbee.toml` in the repository root
5. **Defaults** — built-in sensible defaults

This means you can set global preferences in your config file and override per-project with `.commitbee.toml` or per-invocation with env vars or flags.

## Environment Variables

| Variable | Description |
| --- | --- |
| `COMMITBEE_PROVIDER` | LLM provider (`ollama`, `openai`, `anthropic`) |
| `COMMITBEE_MODEL` | Model name |
| `COMMITBEE_OLLAMA_HOST` | Ollama server URL |
| `COMMITBEE_API_KEY` | API key for cloud providers (**Caution:** Highly insecure! Use `set-key` instead) |
| `COMMITBEE_LOG` | Log level filter (e.g., `debug`, `commitbee=debug`) |

Nested config keys use `__` as separator: `COMMITBEE_FORMAT__INCLUDE_BODY=false`.

## Project Config (.commitbee.toml)

In addition to your platform's global configs, you can place a `.commitbee.toml` at the top level of your git repository. This is highly recommended for teams  
as it allows you to sync and enforce `[limits]` and custom `[conventions]` directly into your codebase.

**Security Constraints**: To prevent malicious repositories from hijacking your local setup, the `.commitbee.toml` project-level configuration **will ignore**  
any `api_key` settings or cloud provider `base_url` redirection parameters. These sensitive security values must be configured via your global setup or `set-key`.
