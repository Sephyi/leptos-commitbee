// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[island]
pub fn DocSearch() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (query, set_query) = signal(String::new());
    let (results, set_results) = signal::<Vec<SearchResult>>(vec![]);
    let (index, set_index) = signal::<Option<Vec<SearchEntry>>>(None);
    let (selected_index, set_selected_index) = signal::<Option<usize>>(None);
    // Tracks the previous `is_open` value across effect runs so we can detect
    // a true→false transition and restore focus to the trigger. Using a
    // `StoredValue` (non-reactive) avoids writing to a signal from inside an
    // effect, which in Leptos 0.8 made the focus-restore effect interact with
    // `is_open` in a way that prevented the modal from opening reliably.
    let prev_open = StoredValue::new(false);

    // Listen for Cmd+K / Ctrl+K and arrow key navigation
    Effect::new(move || {
        let closure =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
                let key = e.key();
                if (e.meta_key() || e.ctrl_key()) && key == "k" {
                    e.prevent_default();
                    set_is_open.update(|v| *v = !*v);
                }
                if key == "Escape" {
                    set_is_open.set(false);
                }
                // Arrow key navigation within modal
                if is_open.get_untracked() {
                    let len = results.get_untracked().len();
                    if key == "ArrowDown" {
                        e.prevent_default();
                        if len > 0 {
                            set_selected_index.update(|idx| {
                                *idx = Some(match *idx {
                                    Some(i) => (i + 1) % len,
                                    None => 0,
                                });
                            });
                        }
                    }
                    if key == "ArrowUp" {
                        e.prevent_default();
                        if len > 0 {
                            set_selected_index.update(|idx| {
                                *idx = Some(match *idx {
                                    Some(0) | None => len.saturating_sub(1),
                                    Some(i) => i - 1,
                                });
                            });
                        }
                    }
                    if key == "Enter"
                        && let Some(i) = selected_index.get_untracked()
                    {
                        let r = results.get_untracked();
                        if let Some(result) = r.get(i) {
                            let href = format!("/docs/{}", result.slug);
                            set_is_open.set(false);
                            let window = web_sys::window().unwrap();
                            let _ = window.location().set_href(&href);
                        }
                    }
                    // Tab focus trap
                    if key == "Tab" {
                        if let Some(document) = web_sys::window().and_then(|w| w.document())
                            && let Some(modal) = document.get_element_by_id("doc-search-modal")
                        {
                            let items = focusable_elements(&modal);
                            if !items.is_empty() {
                                if let Some(active) = document.active_element() {
                                    let active_node = active.unchecked_ref::<web_sys::Node>();
                                    if e.shift_key() {
                                        if active_node.is_same_node(Some(
                                            items[0].unchecked_ref::<web_sys::Node>(),
                                        )) {
                                            e.prevent_default();
                                            let _ = items[items.len() - 1].focus();
                                        }
                                    } else if active_node.is_same_node(Some(
                                        items[items.len() - 1].unchecked_ref::<web_sys::Node>(),
                                    )) {
                                        e.prevent_default();
                                        let _ = items[0].focus();
                                    }
                                }
                            }
                        }
                    }
                }
            });
        let window = web_sys::window().unwrap();
        let _ =
            window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();

        // Listen for a custom open-search event so other components (e.g. the
        // mobile nav search button) can open the modal without sharing signals.
        let open_closure = Closure::<dyn Fn(web_sys::Event)>::new(move |_| {
            set_is_open.set(true);
        });
        let _ = window.add_event_listener_with_callback(
            "commitbee:open-search",
            open_closure.as_ref().unchecked_ref(),
        );
        open_closure.forget();
    });

    // Scroll lock: prevent body scroll when modal is open
    Effect::new(move || {
        let open = is_open.get();
        if let Some(document) = web_sys::window().and_then(|w| w.document())
            && let Some(body) = document.body()
        {
            let _ = body
                .style()
                .set_property("overflow", if open { "hidden" } else { "" });
        }
    });

    // Restore focus to trigger button when modal closes
    Effect::new(move || {
        let open = is_open.get();
        let last = prev_open.get_value();
        prev_open.set_value(open);
        if !open && last {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                let desktop = document
                    .get_element_by_id("doc-search-trigger")
                    .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                    .filter(|el| el.offset_parent().is_some());
                let target = desktop.or_else(|| {
                    document
                        .get_element_by_id("mobile-search-trigger")
                        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                });
                if let Some(btn) = target {
                    let _ = btn.focus();
                }
            }
        }
    });

    // Auto-focus the search input when modal opens
    Effect::new(move || {
        if is_open.get() {
            // Use a short delay to ensure the DOM has rendered the input
            let cb = Closure::<dyn Fn()>::new(move || {
                if let Some(document) = web_sys::window().and_then(|w| w.document())
                    && let Some(el) = document.get_element_by_id("doc-search-input")
                {
                    let _ = el.dyn_into::<web_sys::HtmlElement>().map(|el| el.focus());
                }
            });
            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                50,
            );
            cb.forget();
        }
    });

    // Lazy-load search index when modal opens
    Effect::new(move || {
        if is_open.get() && index.get().is_none() {
            wasm_bindgen_futures::spawn_local(async move {
                let window = web_sys::window().unwrap();
                let resp = wasm_bindgen_futures::JsFuture::from(
                    window.fetch_with_str("/search_index.json"),
                )
                .await;

                if let Ok(resp) = resp {
                    let resp: web_sys::Response = resp.unchecked_into();
                    if resp.ok()
                        && let Ok(text_promise) = resp.text()
                        && let Ok(json) = wasm_bindgen_futures::JsFuture::from(text_promise).await
                        && let Some(text) = json.as_string()
                        && let Ok(entries) = serde_json::from_str::<Vec<SearchEntry>>(&text)
                    {
                        set_index.set(Some(entries));
                    }
                }
            });
        }
    });

    // Perform search when query changes
    Effect::new(move || {
        let q = query.get();
        set_selected_index.set(None);
        if q.is_empty() {
            set_results.set(vec![]);
            return;
        }

        if let Some(entries) = index.get().as_ref() {
            let mut scored: Vec<SearchResult> = entries
                .iter()
                .filter_map(|entry| {
                    let title_match = sublime_fuzzy::best_match(&q, &entry.title);
                    let excerpt_match = sublime_fuzzy::best_match(&q, &entry.excerpt);
                    let heading_score: Option<isize> = entry
                        .headings
                        .iter()
                        .filter_map(|h| sublime_fuzzy::best_match(&q, h).map(|m| m.score()))
                        .max();

                    let best_score = [
                        title_match.as_ref().map(|m| m.score() * 3),
                        heading_score.map(|s| s * 2),
                        excerpt_match.as_ref().map(|m| m.score()),
                    ]
                    .into_iter()
                    .flatten()
                    .max();

                    best_score.map(|score| SearchResult {
                        slug: entry.slug.clone(),
                        title: entry.title.clone(),
                        section: entry.section.clone(),
                        score,
                    })
                })
                .collect();

            scored.sort_by(|a, b| b.score.cmp(&a.score));
            scored.truncate(8);
            set_results.set(scored);
        }
    });

    view! {
        // Search trigger button
        <button
            id="doc-search-trigger"
            on:click=move |_| set_is_open.set(true)
            class="hidden md:flex w-full items-center gap-2 rounded-lg border border-honey/20 bg-surface px-3 py-1.5 text-sm text-comb hover:border-honey/40 transition-colors"
        >
            <svg aria-hidden="true" class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            "Search docs"
            <kbd class="ml-auto rounded bg-surface-raised px-1.5 py-0.5 text-xs text-comb">"⌘K"</kbd>
        </button>

        // Modal overlay
        <Show when=move || is_open.get()>
            <div
                class="fixed inset-0 z-[100] flex items-start justify-center pt-[20vh] bg-bark/50 backdrop-blur-xs"
                on:click=move |_| set_is_open.set(false)
            >
                <div
                    id="doc-search-modal"
                    role="dialog"
                    aria-modal="true"
                    aria-label="Search documentation"
                    class="w-full max-w-lg border shadow-2xl rounded-xl border-honey/20 bg-surface"
                    on:click=move |e| e.stop_propagation()
                >
                    // Search input
                    <div class="flex items-center px-4 border-b border-honey/10">
                        <svg aria-hidden="true" class="w-5 h-5 text-comb" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                        </svg>
                        <input
                            id="doc-search-input"
                            type="text"
                            role="combobox"
                            aria-label="Search documentation"
                            aria-expanded="true"
                            aria-controls="doc-search-results"
                            aria-autocomplete="list"
                            aria-activedescendant=move || {
                                selected_index
                                    .get()
                                    .map(|i| format!("doc-search-result-{i}"))
                                    .unwrap_or_default()
                            }
                            placeholder="Search documentation..."
                            class="flex-1 px-3 py-4 bg-transparent text-bark placeholder:text-comb/70 outline-hidden"
                            autofocus=true
                            on:input=move |e| set_query.set(event_target_value(&e))
                        />
                        <kbd class="px-2 py-1 text-xs rounded bg-surface-raised text-comb">"esc"</kbd>
                    </div>

                    // Screen-reader result count announcement
                    <div class="sr-only" role="status">
                        {move || {
                            let n = results.get().len();
                            if query.get().is_empty() {
                                String::new()
                            } else if n == 0 {
                                "No results".to_string()
                            } else {
                                format!("{n} results")
                            }
                        }}
                    </div>

                    // Results
                    <div id="doc-search-results" role="listbox" aria-label="Search results" class="p-2 overflow-y-auto max-h-80">
                        {move || {
                            let r = results.get();
                            if r.is_empty() && !query.get().is_empty() {
                                view! { <p class="p-4 text-sm text-center text-comb">"No results found"</p> }.into_any()
                            } else {
                                r.into_iter()
                                    .enumerate()
                                    .map(|(i, result)| {
                                        let href = format!("/docs/{}", result.slug);
                                        let is_selected = selected_index.get() == Some(i);
                                        let class = if is_selected {
                                            "block rounded-lg px-4 py-3 bg-honey/10 border border-honey/20 transition-colors"
                                        } else {
                                            "block rounded-lg px-4 py-3 hover:bg-honey/5 transition-colors"
                                        };
                                        view! {
                                            <a
                                                href=href
                                                id=format!("doc-search-result-{i}")
                                                role="option"
                                                aria-selected=if is_selected { "true" } else { "false" }
                                                class=class
                                                on:click=move |_| set_is_open.set(false)
                                            >
                                                <div class="text-sm font-medium text-bark">{result.title.clone()}</div>
                                                <div class="text-xs text-comb">{result.section.clone()}</div>
                                            </a>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            }
                        }}
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// Mobile-only search trigger. Lives in the nav bar on small screens and
/// dispatches a `commitbee:open-search` window event that the `DocSearch`
/// island listens for. Kept as a separate tiny island so the parent `Nav`
/// can stay a pure SSR component.
#[island]
pub fn MobileSearchButton() -> impl IntoView {
    view! {
        <button
            id="mobile-search-trigger"
            type="button"
            class="p-2 transition-colors md:hidden text-comb hover:text-bark"
            aria-label="Search documentation"
            on:click=move |_| {
                if let Some(window) = web_sys::window()
                    && let Ok(event) = web_sys::Event::new("commitbee:open-search")
                {
                    let _ = window.dispatch_event(&event);
                }
            }
        >
            <svg aria-hidden="true" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
        </button>
    }
}

fn focusable_elements(container: &web_sys::Element) -> Vec<web_sys::HtmlElement> {
    let selector =
        "a[href], button:not([disabled]), input:not([disabled]), [tabindex]:not([tabindex='-1'])";
    container
        .query_selector_all(selector)
        .map(|list| {
            (0..list.length())
                .filter_map(|i| list.item(i)?.dyn_into::<web_sys::HtmlElement>().ok())
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Clone, Debug, serde::Deserialize)]
struct SearchEntry {
    slug: String,
    title: String,
    section: String,
    headings: Vec<String>,
    excerpt: String,
}

#[derive(Clone, Debug)]
struct SearchResult {
    slug: String,
    title: String,
    section: String,
    score: isize,
}
