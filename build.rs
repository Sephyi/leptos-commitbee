// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: String,
    order: u32,
    section: String,
    description: String,
}

struct Heading {
    level: u8,
    text: String,
    id: String,
}

struct DocPage {
    slug: String,
    title: String,
    order: u32,
    section: String,
    description: String,
    html_content: String,
    headings: Vec<Heading>,
    word_excerpt: String,
}

fn main() {
    println!("cargo::rerun-if-changed=content/docs/");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let content_dir = Path::new("content/docs");

    if !content_dir.exists() {
        generate_empty_module(&out_dir);
        return;
    }

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let mut pages: Vec<DocPage> = Vec::new();
    let mut routes = vec![
        "/".to_string(),
        "/docs".to_string(),
        "/not-found".to_string(),
    ];

    for entry in WalkDir::new(content_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let raw = fs::read_to_string(entry.path())
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", entry.path().display()));

        let (fm, markdown) = parse_frontmatter(&raw);
        let slug = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let headings = extract_headings(&markdown);
        let html_content = render_markdown_with_syntax_highlighting(&markdown, &ss, theme);
        let html_content = ammonia::Builder::default()
            .add_generic_attributes(["id", "class"])
            .add_tag_attributes("span", ["style"])
            .add_tag_attributes("pre", ["style"])
            .add_tag_attributes("div", ["style", "data-lang"])
            .add_tag_attributes("button", ["data-code"])
            .add_tags(["button"])
            .clean(&html_content)
            .to_string();
        let word_excerpt = extract_excerpt(&markdown, 200);

        routes.push(format!("/docs/{slug}"));

        pages.push(DocPage {
            slug,
            title: fm.title,
            order: fm.order,
            section: fm.section,
            description: fm.description,
            html_content,
            headings,
            word_excerpt,
        });
    }

    // Sort by logical section order, then by order within section
    let section_order = ["Basics", "Usage", "Internals", "Integration", "Reference"];
    let section_rank = |s: &str| -> usize {
        section_order
            .iter()
            .position(|&x| x == s)
            .unwrap_or(usize::MAX)
    };
    pages.sort_by(|a, b| {
        section_rank(&a.section)
            .cmp(&section_rank(&b.section))
            .then(a.order.cmp(&b.order))
    });

    generate_rust_module(&pages, &out_dir);
    generate_search_index(&pages, &out_dir);

    // Routes manifest for pre-rendering
    fs::write(Path::new(&out_dir).join("routes.txt"), routes.join("\n")).unwrap();
}

fn parse_frontmatter(content: &str) -> (Frontmatter, String) {
    let content = content.trim_start();
    if !content.starts_with("---") {
        panic!("Missing YAML frontmatter delimiter");
    }
    let after_first = &content[3..];
    // Match `---` only on its own line, not embedded in YAML values.
    let end = after_first
        .find("\n---\n")
        .or_else(|| after_first.find("\n---\r\n"))
        .or_else(|| {
            // Handle `---` at end of string (no trailing newline)
            after_first
                .find("\n---")
                .filter(|&i| i + 4 >= after_first.len())
        })
        .expect("Missing closing frontmatter delimiter");
    let yaml = &after_first[..end];
    let after_delim = &after_first[end..];
    let markdown = after_delim
        .strip_prefix("\n---\r\n")
        .or_else(|| after_delim.strip_prefix("\n---\n"))
        .or_else(|| after_delim.strip_prefix("\n---"))
        .unwrap_or("");

    let fm: Frontmatter = serde_yaml::from_str(yaml).expect("Invalid frontmatter YAML");

    (fm, markdown.trim().to_string())
}

fn extract_headings(markdown: &str) -> Vec<Heading> {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut headings = Vec::new();
    let mut current_level: Option<u8> = None;
    let mut current_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current_level = Some(level as u8);
                current_text.clear();
            }
            Event::Text(text) if current_level.is_some() => {
                current_text.push_str(&text);
            }
            Event::Code(code) if current_level.is_some() => {
                current_text.push_str(&code);
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(level) = current_level.take() {
                    let id = slug::slugify(&current_text);
                    headings.push(Heading {
                        level,
                        text: current_text.clone(),
                        id,
                    });
                }
            }
            _ => {}
        }
    }

    headings
}

fn render_markdown_with_syntax_highlighting(
    markdown: &str,
    ss: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();
    let mut events: Vec<Event> = Vec::new();
    // Heading ID injection: buffer heading content, emit <hN id="slug">…</hN> on End.
    let mut heading_level: Option<u8> = None;
    let mut heading_text = String::new();
    let mut heading_inner: Vec<Event> = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(ref kind)) => {
                in_code_block = true;
                code_content.clear();
                code_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                    _ => String::new(),
                };
            }
            Event::Text(ref text) if in_code_block => {
                code_content.push_str(text);
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;

                let lang_display = if code_lang.is_empty() {
                    "text"
                } else {
                    &code_lang
                };

                let (highlighted, bg_style) =
                    if let Some(syntax) = ss.find_syntax_by_token(&code_lang) {
                        let raw = highlighted_html_for_string(&code_content, ss, syntax, theme)
                            .unwrap_or_else(|_| html_escape(&code_content));
                        strip_syntect_pre(&raw)
                    } else {
                        (html_escape(&code_content), String::new())
                    };

                let html = format!(
                    r#"<div class="code-block-wrapper relative group rounded-lg overflow-hidden my-6" data-lang="{lang_display}"{bg_style}><div class="code-block-header flex items-center justify-between px-4 py-2 text-xs border-b border-white/10"><span class="text-white/50">{lang_display}</span><button class="copy-btn opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity text-white/40 hover:text-white/80" data-code="{escaped}">Copy</button></div><pre><code>{highlighted}</code></pre></div>"#,
                    escaped = html_escape(&code_content)
                );

                events.push(Event::Html(html.into()));
                continue;
            }
            // Heading start: begin buffering; the id is injected on End.
            Event::Start(Tag::Heading { level, .. }) => {
                heading_level = Some(level as u8);
                heading_text.clear();
                heading_inner.clear();
                continue;
            }
            // Heading end: render buffered inner events, wrap in <hN id="slug">.
            Event::End(TagEnd::Heading(_)) => {
                if let Some(level) = heading_level.take() {
                    let id = slug::slugify(&heading_text);
                    let mut inner_html = String::new();
                    html::push_html(&mut inner_html, heading_inner.drain(..));
                    let heading_html = format!("<h{level} id=\"{id}\">{inner_html}</h{level}>\n");
                    events.push(Event::Html(heading_html.into()));
                }
                continue;
            }
            // Inside a heading: collect plain text for slugification, all events for rendering.
            Event::Text(ref text) if heading_level.is_some() => {
                heading_text.push_str(text);
                heading_inner.push(event);
                continue;
            }
            Event::Code(ref code) if heading_level.is_some() => {
                heading_text.push_str(code);
                heading_inner.push(event);
                continue;
            }
            _ if heading_level.is_some() => {
                heading_inner.push(event);
                continue;
            }
            _ => {}
        }

        if !in_code_block {
            events.push(event);
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    html_output
}

/// Strip syntect's outer `<pre style="...">...</pre>` wrapper.
/// Returns (inner_html, style_attr) where style_attr is ` style="..."` or empty.
fn strip_syntect_pre(html: &str) -> (String, String) {
    // syntect wraps output in: <pre style="background-color:#2b303b;">\n<span ...>...</span>\n</pre>\n
    if let Some(rest) = html.strip_prefix("<pre ") {
        // Extract the style attribute from the opening <pre> tag
        if let Some(close_bracket) = rest.find('>') {
            let attrs = &rest[..close_bracket]; // e.g. style="background-color:#2b303b;"
            let style_attr = if attrs.contains("style=") {
                format!(" {attrs}")
            } else {
                String::new()
            };
            let inner = &rest[close_bracket + 1..];
            // Strip trailing </pre> and whitespace
            let inner = inner
                .trim_end()
                .strip_suffix("</pre>")
                .unwrap_or(inner)
                .trim();
            return (inner.to_string(), style_attr);
        }
    }
    (html.to_string(), String::new())
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn extract_excerpt(markdown: &str, max_words: usize) -> String {
    let parser = Parser::new(markdown);
    let mut words = Vec::new();

    for event in parser {
        if let Event::Text(text) = event {
            words.extend(text.split_whitespace().map(String::from));
            if words.len() >= max_words {
                break;
            }
        }
    }

    words.truncate(max_words);
    words.join(" ")
}

fn generate_rust_module(pages: &[DocPage], out_dir: &str) {
    let mut code = String::new();

    writeln!(code, "pub const SECTION_ORDER: &[&str] = &[\"Basics\", \"Usage\", \"Internals\", \"Integration\", \"Reference\"];").unwrap();
    writeln!(code).unwrap();

    writeln!(code, "#[derive(Debug, Clone)]").unwrap();
    writeln!(code, "pub struct DocPageData {{").unwrap();
    writeln!(code, "    pub slug: &'static str,").unwrap();
    writeln!(code, "    pub title: &'static str,").unwrap();
    writeln!(code, "    pub section: &'static str,").unwrap();
    writeln!(code, "    pub description: &'static str,").unwrap();
    writeln!(code, "    pub order: u32,").unwrap();
    writeln!(code, "    pub html_content: &'static str,").unwrap();
    writeln!(
        code,
        "    pub headings: &'static [(u8, &'static str, &'static str)],"
    )
    .unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    for (i, page) in pages.iter().enumerate() {
        write!(code, "static HEADINGS_{i}: &[(u8, &str, &str)] = &[").unwrap();
        for h in &page.headings {
            // Use raw strings to avoid escaping quotes in heading text.
            write!(code, "({}, r#\"{}\"#, r#\"{}\"#),", h.level, h.text, h.id).unwrap();
        }
        writeln!(code, "];").unwrap();
    }
    writeln!(code).unwrap();

    writeln!(code, "pub static PAGES: &[DocPageData] = &[").unwrap();
    for (i, page) in pages.iter().enumerate() {
        writeln!(code, "    DocPageData {{").unwrap();
        writeln!(code, "        slug: \"{}\",", page.slug).unwrap();
        // Use raw strings for user-authored fields to avoid escaping quotes.
        writeln!(code, "        title: r#\"{}\"#,", page.title).unwrap();
        writeln!(code, "        section: \"{}\",", page.section).unwrap();
        writeln!(code, "        description: r#\"{}\"#,", page.description).unwrap();
        writeln!(code, "        order: {},", page.order).unwrap();
        writeln!(
            code,
            "        html_content: r##\"{}\"##,",
            page.html_content
        )
        .unwrap();
        writeln!(code, "        headings: HEADINGS_{i},").unwrap();
        writeln!(code, "    }},").unwrap();
    }
    writeln!(code, "];").unwrap();
    writeln!(code).unwrap();

    writeln!(
        code,
        "pub fn get_page(slug: &str) -> Option<&'static DocPageData> {{"
    )
    .unwrap();
    writeln!(code, "    PAGES.iter().find(|p| p.slug == slug)").unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    writeln!(
        code,
        "pub fn get_pages_by_section(section: &str) -> Vec<&'static DocPageData> {{"
    )
    .unwrap();
    writeln!(
        code,
        "    PAGES.iter().filter(|p| p.section == section).collect()"
    )
    .unwrap();
    writeln!(code, "}}").unwrap();
    writeln!(code).unwrap();

    writeln!(code, "pub fn get_adjacent(slug: &str) -> (Option<&'static DocPageData>, Option<&'static DocPageData>) {{").unwrap();
    writeln!(
        code,
        "    let idx = PAGES.iter().position(|p| p.slug == slug);"
    )
    .unwrap();
    writeln!(code, "    match idx {{").unwrap();
    writeln!(code, "        Some(i) => (").unwrap();
    writeln!(
        code,
        "            if i > 0 {{ Some(&PAGES[i - 1]) }} else {{ None }},"
    )
    .unwrap();
    writeln!(code, "            PAGES.get(i + 1),").unwrap();
    writeln!(code, "        ),").unwrap();
    writeln!(code, "        None => (None, None),").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}").unwrap();

    fs::write(Path::new(out_dir).join("content_generated.rs"), code).unwrap();
}

fn generate_search_index(pages: &[DocPage], out_dir: &str) {
    let entries: Vec<serde_json::Value> = pages
        .iter()
        .map(|p| {
            let heading_texts: Vec<&str> = p.headings.iter().map(|h| h.text.as_str()).collect();
            serde_json::json!({
                "slug": p.slug,
                "title": p.title,
                "section": p.section,
                "headings": heading_texts,
                "excerpt": p.word_excerpt,
            })
        })
        .collect();

    let json = serde_json::to_string(&entries).unwrap();

    fs::write(Path::new(out_dir).join("search_index.json"), &json).unwrap();

    // Write into the cargo-leptos `assets-dir` (public/) so it's copied to
    // target/site/ as part of the normal asset pipeline. Writing directly to
    // target/site/ races with cargo-leptos's asset copy and can leave the file
    // missing on `cargo check` runs that don't go through cargo-leptos.
    let public_dir = Path::new("public");
    fs::create_dir_all(public_dir).unwrap();
    fs::write(public_dir.join("search_index.json"), &json).unwrap();
}

fn generate_empty_module(out_dir: &str) {
    let code = r#"
pub const SECTION_ORDER: &[&str] = &[];

#[derive(Debug, Clone)]
pub struct DocPageData {
    pub slug: &'static str,
    pub title: &'static str,
    pub section: &'static str,
    pub description: &'static str,
    pub order: u32,
    pub html_content: &'static str,
    pub headings: &'static [(u8, &'static str, &'static str)],
}

pub static PAGES: &[DocPageData] = &[];

pub fn get_page(_slug: &str) -> Option<&'static DocPageData> { None }
pub fn get_pages_by_section(_section: &str) -> Vec<&'static DocPageData> { vec![] }
pub fn get_adjacent(_slug: &str) -> (Option<&'static DocPageData>, Option<&'static DocPageData>) { (None, None) }
"#;

    fs::write(Path::new(out_dir).join("content_generated.rs"), code).unwrap();
    fs::write(Path::new(out_dir).join("routes.txt"), "/\n").unwrap();
    fs::write(Path::new(out_dir).join("search_index.json"), "[]").unwrap();
}
