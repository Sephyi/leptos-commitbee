include!(concat!(env!("OUT_DIR"), "/content_generated.rs"));

/// Sections in display order with their pages
pub fn doc_tree() -> Vec<(&'static str, Vec<&'static DocPageData>)> {
    SECTION_ORDER
        .iter()
        .filter_map(|&section| {
            let pages = get_pages_by_section(section);
            if pages.is_empty() {
                None
            } else {
                Some((section, pages))
            }
        })
        .collect()
}
