// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use serde::Deserialize;
use std::collections::HashSet;
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
    let end = after_first.find("---").ok_or("Missing closing ---")?;
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
