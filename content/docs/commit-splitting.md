---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Commit Splitting"
order: 3
section: "Usage"
description: "Automatic detection and splitting of multi-concern staged changes"
---

# Commit Splitting

One of CommitBee's standout features. When your staged changes contain logically independent work, CommitBee detects this and offers to create separate commits.

## How It Works

The splitter doesn't just look at directory structure. It uses two signals:

**Diff-shape fingerprinting** — Each file gets a "shape" based on its change pattern (ratio of additions to deletions, whether it's a new file, etc.).  
Files with similar shapes are more likely related.

**Jaccard similarity on content vocabulary** — The actual words in the diff are compared. If two files share similar vocabulary (same variable names,  
function names, imports), they're probably part of the same logical change.

Files are then grouped by combining these signals with category separation (tests stay with their source files, docs are separated from code,  
config files are grouped together).

## Example

```txt
Commit split suggested -- 2 logical change groups detected:

  Group 1: feat(llm)  [2 files]
    [M] src/services/llm/anthropic.rs (+20 -5)
    [M] src/services/llm/openai.rs (+8 -3)

  Group 2: fix(sanitizer)  [1 file]
    [M] src/services/sanitizer.rs (+3 -1)

Split into separate commits? (Y/n)
```

If you accept, CommitBee will:

1. Generate a commit message for each group using a group-specific prompt
2. Show you all proposed commits for review
3. Execute them sequentially (unstage all -> stage group files -> commit -> repeat)

## Limitations

- Requires an interactive terminal (no split in `--yes` mode or git hooks)
- Won't split if any staged files also have unstaged changes (safety check)
- Disable with `--no-split` if you know your change is intentionally combined
