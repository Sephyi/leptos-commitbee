---
title: "Configuration"
order: 2
section: "Usage"
description: "Configure commitbee with TOML files, environment variables, and CLI flags"
---

# Configuration

CommitBee uses a 5-level configuration system:

1. Built-in defaults
2. Project `.commitbee.toml`
3. User config (`~/.config/commitbee/config.toml`)
4. Environment variables
5. CLI flags (highest priority)

Run `commitbee init` to create a config file interactively.
