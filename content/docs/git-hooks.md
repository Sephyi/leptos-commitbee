---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Git Hooks"
order: 1
section: "Integration"
description: "Integrate commitbee with git prepare-commit-msg hook"
---

# Git Hooks

CommitBee can run automatically when you `git commit`.

## Install the Hook

```bash
commitbee hook install
```

This creates a `prepare-commit-msg` hook that generates a commit message using CommitBee whenever you run `git commit` without a `-m` flag.

The hook:

- Skips merge, squash, amend, and message-provided commits
- Silently does nothing if `commitbee` isn't on your PATH
- Writes the generated message to the commit message file
- Runs in `--yes --dry-run` mode (non-interactive)

## Manage the Hook

```bash
commitbee hook status      # Check if installed
commitbee hook uninstall   # Remove (restores any backed-up previous hook)
```

If you already had a `prepare-commit-msg` hook, CommitBee backs it up as `prepare-commit-msg.commitbee-backup` and restores it on uninstall.

## TTY Safety

CommitBee detects whether it's running in an interactive terminal. In non-interactive contexts (git hooks, CI pipelines, piped output), it:

- Skips interactive prompts (candidate selection, split confirmation)
- Never blocks waiting for user input
- Prints messages to stdout for piping
