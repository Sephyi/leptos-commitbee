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

## Common Issues

### Ollama not running

Run `commitbee doctor` to check connectivity and model availability.

### Empty commit message

Check that you have staged changes: `git diff --cached --stat`

### Model not found

Pull the model first: `ollama pull qwen3.5:4b`
