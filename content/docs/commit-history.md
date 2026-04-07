---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Commit History Learning"
order: 6
section: "Usage"
description: "Experimental feature to learn and adopt your project's specific commit style."
---

# Commit History Learning (Experimental)

CommitBee can analyze your repository's recent commit history to learn and adopt your team's specific style conventions, without breaking the core Conventional Commits structure.

## How It Works

When history learning is enabled, CommitBee fetches the last $N$ commit subjects (default: 50) using `git log`. From these, it extracts:

- The distribution of commit **types** used in your project.
- Recurring **scope** patterns.
- Phrasing tendencies and **case style** (e.g., preference for imperative mood, capitalization).
- The ratio of conventional vs. non-conventional commits.

This information is injected into the LLM prompt as a `PROJECT STYLE` block.

> **Note:** History learning **does not** override the strict Conventional Commits format (type, optional scope, description). It only guides the LLM in how to formulate the *scope* names and the phrasing of the *subject description* so that it blands naturally with your existing git log.

## Enabling History Learning

Because this feature is experimental, it must be explicitly enabled either via the CLI or in your configuration.

### Via CLI

You can pass the experimental flag when running your commit:

```bash
commitbee --experimental-history
```

### Via Configuration

To enable history learning persistently, update your `.commitbee.toml` (project) or `config.toml` (global/user):

```toml
# Enable history learning
learn_from_history = true

# Number of commits to sample (default is 50)
history_sample_size = 50
```

*This feature was shipped in v0.4.0 (FR-058).*

