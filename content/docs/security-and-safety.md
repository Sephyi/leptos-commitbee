---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Security & Safety"
order: 2
section: "Internals"
description: "Secret scanning with 24 built-in patterns across 13 categories"
---

# Security & Safety

## Secret Scanning

Before anything is sent to an LLM, CommitBee scans all staged content with **24 built-in patterns** across 13 categories:

| Category | Patterns |
| --- | --- |
| Cloud Providers | AWS access key, AWS secret key, GCP service account, GCP API key, Azure storage key |
| AI/ML | OpenAI key, Anthropic key, HuggingFace token |
| Source Control | GitHub token, GitHub fine-grained token, GitLab token |
| Communication | Slack token, Slack webhook, Discord webhook |
| Payment & SaaS | Stripe key, Twilio key, SendGrid key, Mailgun key |
| Database | Connection strings (MongoDB, PostgreSQL, MySQL, Redis, AMQP) |
| Cryptographic | Private keys (PEM), JWT tokens |
| Generic | API key assignments, quoted secrets, unquoted secrets |

You can extend or customize the pattern set via config:

```toml
# Add custom regex patterns
custom_secret_patterns = ["CUSTOM_KEY_[a-zA-Z0-9]{32}"]

# Disable built-in patterns by name (case-insensitive)
disabled_secret_patterns = ["Generic Secret (unquoted)"]
```

If secrets are found:

- **Ollama (local)**: Warning displayed, proceeds (data stays on your machine)
- **Cloud providers**: Hard error, commit blocked. Use `--allow-secrets` to override (Ollama only)

Scanning only checks added lines — removed lines are ignored (they're already in git history).

## Merge Conflict Detection

CommitBee checks for unresolved merge conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) in staged changes. If found, the commit is blocked with an actionable error.

The conflict checker is smart about false positives:

- Ignores conflict markers in test files and documentation
- Ignores diff headers (lines starting with `---` or `+++`)
- Uses component-based path matching to avoid false positives from CommitBee's own source code

## Data Privacy

With the default Ollama provider, **no data ever leaves your machine**. The entire pipeline runs locally. Cloud providers (OpenAI, Anthropic) send the prompt over HTTPS — which includes your diff and symbol information. Choose your provider accordingly.
