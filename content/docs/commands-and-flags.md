---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Commands & Flags"
order: 1
section: "Reference"
description: "Complete reference for commitbee CLI commands and flags"
---

# Commands & Flags

## Main Usage

```bash
commitbee [FLAGS] [COMMAND]
```

When run without a command, CommitBee generates a commit message for your staged changes.

## Flags

| Flag | Short | Description |
| --- | --- | --- |
| `--dry-run` | | Print message only, don't commit |
| `--yes` | `-y` | Auto-confirm and commit without prompting |
| `--generate N` | `-n N` | Generate N candidates (1-5), pick interactively |
| `--no-split` | | Disable commit split suggestions |
| `--no-scope` | | Disable scope in commit messages |
| `--clipboard` | | Copy message to clipboard instead of committing |
| `--exclude <GLOB>` | | Exclude files matching glob pattern (repeatable) |
| `--allow-secrets` | | Allow committing with detected secrets (Ollama only) |
| `--show-prompt` | | Display the full prompt sent to the LLM |
| `--verbose` | `-v` | Show symbol extraction details |
| `--provider` | `-p` | Override LLM provider |
| `--model` | `-m` | Override model name |

## Commands

| Command | Description |
| --- | --- |
| `init` | Create a config file at the platform path |
| `config` | Show current configuration values |
| `config check` | Validate configuration |
| `config set-key <provider>` | Store API key in system keychain |
| `config get-key <provider>` | Check if key exists in keychain |
| `doctor` | Check configuration, connectivity, and model availability |
| `completions <shell>` | Generate shell completions (bash, zsh, fish, powershell) |
| `hook install` | Install `prepare-commit-msg` git hook |
| `hook uninstall` | Remove the git hook |
| `hook status` | Check if the hook is installed |

## Usage Patterns

```bash
# The basics
commitbee                        # Interactive: generate, review, commit
commitbee --dry-run              # Preview without committing
commitbee --yes                  # Non-interactive: generate and commit

# Debugging
commitbee --show-prompt          # See exactly what the LLM receives
commitbee --verbose              # See tree-sitter symbol extraction
COMMITBEE_LOG=debug commitbee    # Full debug logging

# Multiple candidates
commitbee -n 3                   # Generate 3 options, pick the best

# Clipboard
commitbee --clipboard            # Copy message to clipboard (no commit)

# Exclude files
commitbee --exclude "*.lock"     # Skip lock files from analysis
commitbee --exclude "*.lock" --exclude "vendor/**"  # Multiple patterns

# Scripting / CI
commitbee --yes --dry-run        # Generate message, print to stdout, exit
commitbee --no-split --yes       # Skip split suggestion, auto-commit
```
