---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "CI/CD Integration"
order: 3
section: "Integration"
description: "Using CommitBee in GitHub Actions, GitLab CI, and beyond"
---

# CI/CD Integration

CommitBee is not just a local developer tool; you can plug its validation pipeline directly into your CI/CD to enforce conventions and summarize pull requests.

## GitHub Actions

Create `.github/workflows/commitbee.yml` inside your repository to use CommitBee's message validation checks automatically on every push:

```yaml
name: CommitBee Validation

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install CommitBee
        run: cargo install commitbee
      - name: Validate Commits
        run: commitbee check
```

## Running Non-Interactively

Whenever you run CommitBee in a headless CI environment, you must use the `--yes` (or `-y`) parameter, which bypasses the interactive approval prompt.

```bash
commitbee --yes --dry-run
```

By default, CommitBee runs the **Validation Pipeline** against the LLM's generated strings, ensuring no bad commits land on your main branches.  
For CI checks without generating anything new, use `commitbee check`.

## Securing your APIs

When integrating cloud LLM providers (e.g., OpenAI or Anthropic), define and pass the necessary environment variables explicitly to your jobs.  
**Important**: Only expose API keys in private repositories or as encrypted Repository Secrets:

```yaml
env:
  OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
```

For the recommended *local* LLM configurations, verify your runner image has Ollama bundled or pull it within your steps before running.
