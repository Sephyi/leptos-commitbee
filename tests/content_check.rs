// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Frontmatter {
    title: String,
    order: u32,
    section: String,
    description: String,
}

fn parse_frontmatter(content: &str) -> Result<Frontmatter, String> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return Err("Missing opening ---".to_string());
    }
    let after_first = &content[3..];
    // Match `---` only on its own line, not embedded in YAML values.
    let end = after_first
        .find("\n---\n")
        .or_else(|| after_first.find("\n---\r\n"))
        .or_else(|| {
            after_first
                .find("\n---")
                .filter(|&i| i + 4 >= after_first.len())
        })
        .ok_or("Missing closing ---")?;
    let yaml = &after_first[..end];
    serde_yaml::from_str(yaml).map_err(|e| e.to_string())
}

#[test]
fn all_docs_have_valid_frontmatter() {
    let content_dir = Path::new("content/docs");
    assert!(
        content_dir.exists(),
        "content/docs/ directory does not exist"
    );

    let mut count = 0;
    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        let result = parse_frontmatter(&raw);
        assert!(
            result.is_ok(),
            "Invalid frontmatter in {}: {}",
            entry.path().display(),
            result.unwrap_err()
        );

        let fm = result.unwrap();
        assert!(
            !fm.title.is_empty(),
            "Empty title in {}",
            entry.path().display()
        );
        assert!(
            !fm.section.is_empty(),
            "Empty section in {}",
            entry.path().display()
        );
        assert!(
            !fm.description.is_empty(),
            "Empty description in {}",
            entry.path().display()
        );

        count += 1;
    }

    assert!(count >= 12, "Expected at least 12 doc files, found {count}");
}

#[test]
fn no_duplicate_slugs() {
    let content_dir = Path::new("content/docs");
    let mut slugs = HashSet::new();

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let slug = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        assert!(slugs.insert(slug.clone()), "Duplicate slug: {slug}");
    }
}

#[test]
fn sections_are_recognized() {
    let valid_sections: HashSet<&str> =
        ["Basics", "Usage", "Internals", "Integration", "Reference"]
            .into_iter()
            .collect();

    let content_dir = Path::new("content/docs");

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        let fm = parse_frontmatter(&raw).unwrap();
        assert!(
            valid_sections.contains(fm.section.as_str()),
            "Unknown section '{}' in {}",
            fm.section,
            entry.path().display()
        );
    }
}

#[test]
fn order_values_are_unique_within_section() {
    let content_dir = Path::new("content/docs");
    // Map section → set of order values seen so far.
    let mut section_orders: HashMap<String, HashSet<u32>> = HashMap::new();

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        let fm = parse_frontmatter(&raw).unwrap();
        let seen = section_orders.entry(fm.section.clone()).or_default();
        assert!(
            seen.insert(fm.order),
            "Duplicate order {} within section '{}' (found in {})",
            fm.order,
            fm.section,
            entry.path().display()
        );
    }
}

#[test]
fn description_length_reasonable() {
    let content_dir = Path::new("content/docs");

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        let fm = parse_frontmatter(&raw).unwrap();
        let len = fm.description.len();
        assert!(
            len >= 10,
            "Description too short ({len} chars) in {}",
            entry.path().display()
        );
        assert!(
            len <= 200,
            "Description too long ({len} chars) in {}",
            entry.path().display()
        );
    }
}

#[test]
fn frontmatter_delimiter_on_own_line() {
    let content_dir = Path::new("content/docs");

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        let content = raw.trim_start();
        // Opening delimiter must be on its own line.
        assert!(
            content.starts_with("---\n") || content.starts_with("---\r\n"),
            "Opening --- not on its own line in {}",
            entry.path().display()
        );
        let after_first = &content[3..];
        // Closing delimiter must be on its own line (matching the fixed parser).
        let has_proper_close = after_first.contains("\n---\n")
            || after_first.contains("\n---\r\n")
            || after_first
                .rfind("\n---")
                .is_some_and(|i| i + 4 >= after_first.len());
        assert!(
            has_proper_close,
            "Closing --- not on its own line in {}",
            entry.path().display()
        );
    }
}

#[test]
fn internal_links_resolve() {
    let content_dir = Path::new("content/docs");

    // Build set of all known slugs.
    let slugs: HashSet<String> = WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .map(|e| e.path().file_stem().unwrap().to_str().unwrap().to_string())
        .collect();

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let raw = std::fs::read_to_string(entry.path()).unwrap();
        for linked_slug in extract_doc_link_slugs(&raw) {
            assert!(
                slugs.contains(&linked_slug),
                "Broken internal link /docs/{linked_slug} in {}",
                entry.path().display()
            );
        }
    }
}

/// Extract slugs from Markdown links of the form `(/docs/slug)` or `(/docs/slug#section)`.
fn extract_doc_link_slugs(content: &str) -> Vec<String> {
    let mut slugs = Vec::new();
    let mut remaining = content;
    while let Some(pos) = remaining.find("(/docs/") {
        let after = &remaining[pos + 7..];
        // Slug ends at `)`, `#`, `?`, or any whitespace.
        let end = after
            .find(|c: char| c == ')' || c == '#' || c == '?' || c.is_whitespace())
            .unwrap_or(after.len());
        let slug = after[..end].trim_matches('/');
        if !slug.is_empty() {
            slugs.push(slug.to_string());
        }
        remaining = &remaining[pos + 7..];
    }
    slugs
}
