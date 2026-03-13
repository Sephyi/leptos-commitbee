---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Commands & Flags"
order: 1
section: "Usage"
description: "Complete reference for commitbee CLI commands and flags"
---

# Commands & Flags

## Options

| Flag | Description |
| --- | --- |
| `--dry-run` | Print message only, don't commit |
| `--yes` | Auto-confirm and commit |
| `-n, --generate` | Generate N candidates (1-5, default 1) |
| `--no-split` | Disable commit split suggestions |
| `--no-scope` | Disable scope in commit messages |
| `--clipboard` | Copy message to clipboard instead of committing |
| `--exclude <GLOB>` | Exclude files matching glob pattern (repeatable) |
| `--allow-secrets` | Allow committing with detected secrets |
| `--verbose` | Show symbol extraction details |
| `--show-prompt` | Debug: display the full LLM prompt |

## Commands

| Command | Description |
| --- | --- |
| `init` | Create a config file |
| `config` | Show current configuration |
| `doctor` | Check configuration and connectivity |
| `completions <shell>` | Generate shell completions |
| `hook install` | Install prepare-commit-msg hook |
| `hook uninstall` | Remove prepare-commit-msg hook |
| `hook status` | Check if hook is installed |
