// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Island that sets up scroll-spy highlighting for the right-side table of contents.
/// Uses `IntersectionObserver` to watch heading elements and highlights the matching
/// TOC link when a heading enters the viewport.
#[island]
pub fn TocHighlighter() -> impl IntoView {
    Effect::new(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Find all TOC links (anchors with fragment hrefs inside the aside)
        let toc_links = document.query_selector_all("aside a[href^='#']").unwrap();
        if toc_links.length() == 0 {
            return;
        }

        // Collect heading IDs referenced by the TOC (in document order)
        let mut heading_ids: Vec<String> = Vec::new();
        for i in 0..toc_links.length() {
            if let Some(link) = toc_links.item(i) {
                let el: &web_sys::Element = link.unchecked_ref();
                if let Some(href) = el.get_attribute("href")
                    && let Some(id) = href.strip_prefix('#')
                {
                    heading_ids.push(id.to_string());
                }
            }
        }

        if heading_ids.is_empty() {
            return;
        }

        // Collect the actual heading elements
        let headings: Vec<web_sys::Element> = heading_ids
            .iter()
            .filter_map(|id| document.get_element_by_id(id))
            .collect();

        if headings.is_empty() {
            return;
        }

        // Active class constants
        let active_classes: &[&str] = &["text-honey", "font-medium", "border-l-2", "border-honey"];
        let inactive_class = "text-comb";

        // Track which heading IDs are currently visible in the observer root
        let visible: Rc<RefCell<HashSet<String>>> = Rc::new(RefCell::new(HashSet::new()));

        // Build the observer callback
        let visible_ref = Rc::clone(&visible);
        let heading_ids_ref = heading_ids.clone();
        let toc_links_ref = toc_links.clone();
        let callback = Closure::<dyn Fn(js_sys::Array)>::new(move |entries: js_sys::Array| {
            // Update the visible set based on intersection changes
            let mut set = visible_ref.borrow_mut();
            for i in 0..entries.length() {
                let entry: web_sys::IntersectionObserverEntry = entries.get(i).unchecked_into();
                let id = entry.target().id();
                if entry.is_intersecting() {
                    set.insert(id);
                } else {
                    set.remove(&id);
                }
            }

            // Pick the first heading (in document order) that is currently visible
            let active_id = heading_ids_ref.iter().find(|id| set.contains(id.as_str()));

            // Update TOC link styles
            for j in 0..toc_links_ref.length() {
                if let Some(link) = toc_links_ref.item(j) {
                    let el: web_sys::HtmlElement = link.unchecked_into();
                    let class_list = el.class_list();
                    if let Some(href) = el.get_attribute("href") {
                        let is_active = active_id
                            .is_some_and(|aid| href.strip_prefix('#').is_some_and(|id| id == aid));
                        if is_active {
                            let _ = class_list.remove_1(inactive_class);
                            for cls in active_classes {
                                let _ = class_list.add_1(cls);
                            }
                        } else {
                            for cls in active_classes {
                                let _ = class_list.remove_1(cls);
                            }
                            let _ = class_list.add_1(inactive_class);
                        }
                    }
                }
            }
        });

        // Configure observer with a negative bottom margin so headings trigger
        // when they reach the top portion of the viewport
        let options = web_sys::IntersectionObserverInit::new();
        options.set_root_margin("0px 0px -70% 0px");

        let observer = web_sys::IntersectionObserver::new_with_options(
            callback.as_ref().unchecked_ref(),
            &options,
        )
        .unwrap();

        for heading in &headings {
            observer.observe(heading);
        }

        // Leak the callback and observer so they stay alive for the page lifetime
        callback.forget();
        std::mem::forget(observer);
    });

    view! { <div class="hidden" data-toc-highlighter="true"/> }
}
