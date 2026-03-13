---
title: "Git Hooks"
order: 1
section: "Integration"
description: "Integrate commitbee with git prepare-commit-msg hook"
---

# Git Hooks

CommitBee integrates with git's `prepare-commit-msg` hook.

```bash
# Install the hook
commitbee hook install

# Check status
commitbee hook status

# Remove the hook
commitbee hook uninstall
```

The hook has TTY detection for safe non-interactive fallback.
