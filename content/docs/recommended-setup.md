---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Recommended Setup"
order: 2
section: "Basics"
description: "The official baseline configuration for generating the most accurate and descriptive commits"
---

# Recommended Setup

While the local `qwen3.5:4b` defaults are great for getting started quickly, CommitBee truly shines when you configure it for  
high-quality logic handling complex, multi-file architectural changes.

This setup represents the officially recommended baseline.

## ⚙️ Config

```toml
# LLM Provider Configuration
provider = "anthropic"
model = "claude-sonnet-4-6"

# Increase character context budget for large changes 
max_context_chars = 48000

# Standardized settings
temperature = 0.7
num_predict = 4096
```

> **Note**: Claude Haiku 4.5 is also an excellent, faster alternative when tweaking your workflow!

## Secure Your API Key

**Security Caution**: **Never** store your `api_key` in this configuration file! Always use `set-key` to store cloud provider  
credentials securely inside your native OS keychain:

```bash
commitbee config set-key anthropic
```

## 💬 Community Discussion

This configuration is discussed in a "living post" that will adapt as models evolve or features land.  
You can compare notes, evaluate local models, or see what is actively being tested next by reading the conversation on GitHub:

Read and contribute here: [Recommended Setup Discussion #4](https://github.com/Sephyi/commitbee/discussions/4)
