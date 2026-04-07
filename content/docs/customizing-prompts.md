---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Customizing Prompts"
order: 4
section: "Usage"
description: "How to customize CommitBee's behavior and commit message format"
---

# Customizing Prompts

CommitBee generates messages according to the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification by default.  
However, you can tailor its behavior to match your team's specific conventions.

## Global Configuration

You can adjust formatting rules in your `config.toml`. 

```toml
[prompt]
enforce_imperative = true
max_subject_length = 72
include_issue_references = true
```

## System Prompts

Advanced users can override the default LLM system prompts. Create a `prompt.md` file in your repository's `.commitbee/` directory:

```md
You are an expert developer writing commit messages for our repository.
Our team requires the `JIRA-123:` prefix in the summary line for all commits.

CRITICAL:
- Do not use markdown backticks in the summary line.
- Always include a "Why" in the body.
```

When a custom prompt is detected, CommitBee will merge it with its semantic context (like tree-sitter symbols) before sending it to the LLM.

## Overriding Commit Types

If your repository uses custom types beyond `feat`, `fix`, `docs`, `chore`, etc., you can define them in your config:

```toml
[conventions.types]
wip = "Work in progress (do not merge)"
perf = "Performance improvements"
hotfix = "Emergency production fixes"
```

CommitBee's validation pipeline will automatically learn these new types and ensure the LLM uses them correctly.
