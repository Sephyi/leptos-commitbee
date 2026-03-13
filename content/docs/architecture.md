---
title: "Architecture"
order: 1
section: "Reference"
description: "Deep dive into commitbee's internal architecture and design decisions"
---

# Architecture

CommitBee is ~18K lines of Rust compiled to a single static binary with LTO.

## Core Pipeline

```
Stage -> Git Service -> Tree-sitter Analyzer -> Split Detector
      -> Context Builder -> LLM Provider -> Validator -> Sanitizer
```

## Testing

308 tests: unit, snapshot (insta), property (proptest), integration (wiremock).
