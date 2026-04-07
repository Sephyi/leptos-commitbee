---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Advanced Workflows"
order: 5
section: "Usage"
description: "Mastering CommitBee for large diffs, monorepos, and daily productivity"
---

# Advanced Workflows

Once you've mastered the basics and configured your LLMs, you can supercharge your productivity. CommitBee is built specifically for users operating in massive projects and large changesets.

## Ignoring Files

Sometimes you have staging files that the LLM has no business reading—e.g., massive JSON dumps or lock files. While CommitBee already strips some of these natively, you can manually guide it:

### `.commitbeeignore`

Create this file in the root of your repo to skip directories or patterns entirely from the diff context building step.

```txt
tests/fixtures/*.json
target/
.github/
```

This prevents token limits being blown out on non-essential noise and ensures your semantic tree-sitter extraction uses its processing power on business logic.

## Dry Runs

You should always use `--dry-run` to experiment with your generated commits when implementing custom prompts or tweaking LLM settings. This ensures your repository remains pristine, generating and outputting the commit text securely to your terminal without running `git commit`.

```bash
# Output the markdown, do not touch git
commitbee --dry-run
```

## Large Codebases (Mono-repos)

When modifying multiple projects inside a mono-repo, CommitBee pairs perfectly with Commit Splitting. We recommend using a powerful model (e.g., Anthropic Claude 3.5 Sonnet or OpenAI `gpt-4o`) rather than a 4b local model to ensure it handles the cross-context reasoning effectively.

If your changes remain too big for the LLM token limits, CommitBee will summarize the `git diff` intelligently before feeding it. You can enforce a hard token cap in the `config.toml` to ensure API limits are never exceeded:

```toml
[limits]
max_diff_tokens = 8192
```
