---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Security & Safety"
order: 4
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

### Full pattern list

Every built-in pattern, grouped by category. Names match the strings used in `disabled_secret_patterns`.

| # | Category | Name | Detects |
| --- | --- | --- | --- |
| 1 | Cloud Providers | AWS Access Key | AWS IAM access key ID (`AKIA...`) |
| 2 | Cloud Providers | AWS Secret Key | AWS secret access key assignment (40-char base64) |
| 3 | Cloud Providers | GCP Service Account | Google Cloud service account JSON key (`"type": "service_account"`) |
| 4 | Cloud Providers | GCP API Key | Google API key (`AIza...`) |
| 5 | Cloud Providers | Azure Storage Key | Azure storage account key (`AccountKey=...`) |
| 6 | AI/ML | OpenAI Key | OpenAI API key (legacy `sk-...`, project `sk-proj-...`, or service account `sk-svcacct-...`) |
| 7 | AI/ML | Anthropic Key | Anthropic API key (`sk-ant-...`) |
| 8 | AI/ML | HuggingFace Token | HuggingFace access token (`hf_...`) |
| 9 | Source Control | GitHub Token | GitHub personal access or OAuth token (`ghp_...`, `ghs_...`) |
| 10 | Source Control | GitHub Fine-Grained Token | GitHub fine-grained personal access token (`github_pat_...`) |
| 11 | Source Control | GitLab Token | GitLab personal access token (`glpat-...`) |
| 12 | Communication | Slack Token | Slack bot, user, or app token (`xoxb-`, `xoxp-`, `xoxa-`, `xoxr-`, `xoxs-`) |
| 13 | Communication | Slack Webhook | Slack incoming webhook URL (`hooks.slack.com/services/...`) |
| 14 | Communication | Discord Webhook | Discord webhook URL (`discord.com/api/webhooks/...`) |
| 15 | Payment & SaaS | Stripe Key | Stripe secret or restricted API key (`sk_live_`, `sk_test_`, `rk_live_`, `rk_test_`) |
| 16 | Payment & SaaS | Twilio Key | Twilio API key SID (`SK` + 32 hex chars) |
| 17 | Payment & SaaS | SendGrid Key | SendGrid API key (`SG.<id>.<secret>`) |
| 18 | Payment & SaaS | Mailgun Key | Mailgun API key (`key-` + 32 hex chars) |
| 19 | Database | Connection String | Database or message broker URI (`mongodb://`, `mongodb+srv://`, `postgres://`, `postgresql://`, `mysql://`, `redis://`, `amqp://`) |
| 20 | Cryptographic | Private Key | PEM-encoded private key (`-----BEGIN ... PRIVATE KEY-----`, RSA, EC, etc.) |
| 21 | Cryptographic | JWT Token | JSON Web Token (three-part Base64 `eyJ...`) |
| 22 | Generic | Generic API Key | Generic `api_key` / `apikey` assignment (20+ chars) |
| 23 | Generic | Generic Secret | Quoted `password` / `secret` / `token` assignment (8+ chars) |
| 24 | Generic | Generic Secret (unquoted) | Unquoted `password` / `secret` / `token` assignment (16+ chars) |

You can extend or customize the pattern set via config:

```toml
# Add custom regex patterns
custom_secret_patterns = ["CUSTOM_KEY_[a-zA-Z0-9]{32}"]

# Disable built-in patterns by name (case-insensitive)
disabled_secret_patterns = ["Generic Secret (unquoted)"]
```

If secrets are found:

- **Ollama (local)**: Warning displayed, proceeds (data stays on your machine)
- **Cloud providers**: Hard error, commit blocked. Use `--allow-secrets` to override

Scanning only checks added lines — removed lines are ignored (they're already in git history).

## Merge Conflict Detection

CommitBee checks for unresolved merge conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) in staged changes. If found, the commit is blocked with an actionable error.

The conflict checker is smart about false positives:

- Ignores conflict markers in test files and documentation
- Ignores diff headers (lines starting with `---` or `+++`)
- Uses component-based path matching to avoid false positives from CommitBee's own source code

## Data Privacy

With the default Ollama provider, **no data ever leaves your machine**. The entire pipeline runs locally.  
Cloud providers (OpenAI, Anthropic) send the prompt over HTTPS — which includes your diff and symbol information. Choose your provider accordingly.
