use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[island]
pub fn DocSearch() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (query, set_query) = signal(String::new());
    let (results, set_results) = signal::<Vec<SearchResult>>(vec![]);
    let (index, set_index) = signal::<Option<Vec<SearchEntry>>>(None);

    // Listen for Cmd+K / Ctrl+K
    Effect::new(move || {
        let closure = Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
            move |e: web_sys::KeyboardEvent| {
                if (e.meta_key() || e.ctrl_key()) && e.key() == "k" {
                    e.prevent_default();
                    set_is_open.update(|v| *v = !*v);
                }
                if e.key() == "Escape" {
                    set_is_open.set(false);
                }
            },
        );
        let window = web_sys::window().unwrap();
        let _ = window.add_event_listener_with_callback(
            "keydown",
            closure.as_ref().unchecked_ref(),
        );
        closure.forget();
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
                    if let Ok(json) = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap()).await
                    {
                        if let Some(text) = json.as_string() {
                            if let Ok(entries) = serde_json::from_str::<Vec<SearchEntry>>(&text) {
                                set_index.set(Some(entries));
                            }
                        }
                    }
                }
            });
        }
    });

    // Perform search when query changes
    Effect::new(move || {
        let q = query.get();
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
            on:click=move |_| set_is_open.set(true)
            class="hidden md:flex items-center gap-2 rounded-lg border border-honey/20 bg-surface px-3 py-1.5 text-sm text-comb hover:border-honey/40 transition-colors"
        >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
            </svg>
            "Search docs"
            <kbd class="ml-2 rounded bg-surface-raised px-1.5 py-0.5 text-xs text-comb">"⌘K"</kbd>
        </button>

        // Modal overlay
        <Show when=move || is_open.get()>
            <div
                class="fixed inset-0 z-[100] flex items-start justify-center pt-[20vh] bg-bark/50 backdrop-blur-sm"
                on:click=move |_| set_is_open.set(false)
            >
                <div
                    class="w-full max-w-lg rounded-xl border border-honey/20 bg-surface shadow-2xl"
                    on:click=move |e| e.stop_propagation()
                >
                    // Search input
                    <div class="flex items-center border-b border-honey/10 px-4">
                        <svg class="w-5 h-5 text-comb" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                        </svg>
                        <input
                            type="text"
                            placeholder="Search documentation..."
                            class="flex-1 bg-transparent px-3 py-4 text-bark placeholder:text-comb/50 outline-none"
                            autofocus=true
                            on:input=move |e| set_query.set(event_target_value(&e))
                        />
                        <kbd class="rounded bg-surface-raised px-2 py-1 text-xs text-comb">"esc"</kbd>
                    </div>

                    // Results
                    <div class="max-h-80 overflow-y-auto p-2">
                        {move || {
                            let r = results.get();
                            if r.is_empty() && !query.get().is_empty() {
                                view! { <p class="p-4 text-sm text-comb text-center">"No results found"</p> }.into_any()
                            } else {
                                r.into_iter()
                                    .map(|result| {
                                        let href = format!("/docs/{}", result.slug);
                                        view! {
                                            <a
                                                href=href
                                                class="block rounded-lg px-4 py-3 hover:bg-honey/5 transition-colors"
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
