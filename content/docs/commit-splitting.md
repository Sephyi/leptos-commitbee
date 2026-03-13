---
# SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
#
# SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

title: "Commit Splitting"
order: 4
section: "Usage"
description: "Automatic detection and splitting of multi-concern staged changes"
---

# Commit Splitting

When your staged changes mix independent work, CommitBee detects it and offers to split them into separate commits.

The splitter uses diff-shape fingerprinting combined with Jaccard similarity on content vocabulary.
