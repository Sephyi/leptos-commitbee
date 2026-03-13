---
title: "Validation Pipeline"
order: 1
section: "Internals"
description: "The 7-rule validation pipeline that ensures commit message quality"
---

# Validation Pipeline

Every generated message passes through a 7-rule validation pipeline:

1. **Fix requires evidence** - no bug comments, no `fix` type
2. **Breaking change detection** - removed public APIs must be flagged
3. **Anti-hallucination** - breaking change text can't copy internal field names
4. **Mechanical changes** must use `style`
5. **Dependency-only changes** must use `chore`
6. **Subject specificity** - rejects generic messages like "update code"
7. **Subject length** - enforces the 72-character first line limit
