---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Troubleshooting"
order: 2
section: "Integration"
description: "Common issues and their solutions"
---

# Troubleshooting

## `commitbee doctor`

Your first stop for diagnosing issues. It checks:

- Config file location and existence
- Provider connectivity (can CommitBee reach Ollama/OpenAI/Anthropic?)
- Model availability (is the configured model actually pulled?)
- Git repository detection

## Common Issues

### "Empty response from LLM"

The model ran out of tokens before producing output. Usually caused by thinking models consuming the token budget with thinking blocks.

Fix: Either switch to `qwen3.5:4b` (default, no thinking overhead) or increase `num_predict`:

```toml
num_predict = 8192
think = true
```

### "First line is X chars (max 72)"

The LLM generated a subject line that's too long. CommitBee will retry up to 3 times with correction instructions.  
If it still fails, the error tells you exactly how long the line was. This is rare with the default model.

### "No staged changes found"

You need to `git add` files before running CommitBee.

### "Cannot connect to Ollama"

Ollama isn't running. Start it with `ollama serve` or check that the configured `ollama_host` is correct.

### "Model not found"

The configured model isn't pulled. Run `ollama pull qwen3.5:4b` (or whichever model you've configured).

### "Potential secrets detected"

CommitBee found something that looks like an API key or credential in your staged changes. If it's a false positive and you're using Ollama (local), use `--allow-secrets`. For cloud providers, this is a hard block — remove the secret from your staged changes.

## Debug Mode

For deep debugging, use `--show-prompt` to see the exact prompt sent to the LLM:

```bash
commitbee --dry-run --show-prompt
```

This prints the full prompt including the diff, evidence flags, constraints, symbol list, and character budget. Very useful for understanding why the LLM made a particular choice.

For internal tracing:

```bash
COMMITBEE_LOG=debug commitbee --dry-run
```

This shows config loading, symbol counts, sanitizer steps, validation violations, and retry attempts.
