// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

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

#[island]
pub fn MobileMenu() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Global keydown: Escape to close, Tab focus trap while open.
    Effect::new(move |_| {
        let Some(window) = web_sys::window() else {
            return;
        };
        let closure =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
                if !is_open.get_untracked() {
                    return;
                }
                let key = e.key();
                if key == "Escape" {
                    e.prevent_default();
                    set_is_open.set(false);
                    return;
                }
                if key == "Tab" {
                    if let Some(document) = web_sys::window().and_then(|w| w.document())
                        && let Some(panel) = document.get_element_by_id("mobile-menu-panel")
                    {
                        let items = focusable_elements(&panel);
                        if items.is_empty() {
                            return;
                        }
                        let first = &items[0];
                        let last = &items[items.len() - 1];
                        if let Some(active) = document.active_element() {
                            let active_node = active.unchecked_ref::<web_sys::Node>();
                            if e.shift_key() {
                                if active_node
                                    .is_same_node(Some(first.unchecked_ref::<web_sys::Node>()))
                                {
                                    e.prevent_default();
                                    let _ = last.focus();
                                }
                            } else if active_node
                                .is_same_node(Some(last.unchecked_ref::<web_sys::Node>()))
                            {
                                e.prevent_default();
                                let _ = first.focus();
                            }
                        }
                    }
                }
            });
        let _ =
            window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();
    });

    // Side effects on open/close: scroll lock + focus management.
    Effect::new(move |prev: Option<bool>| {
        let open = is_open.get();
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(body) = document.body() {
                let _ = body
                    .style()
                    .set_property("overflow", if open { "hidden" } else { "" });
            }
            // Only act on transitions (not the initial mount where prev is None).
            if let Some(was_open) = prev
                && was_open != open
            {
                if open {
                    // Move focus into the panel after it mounts.
                    if let Some(panel) = document.get_element_by_id("mobile-menu-panel") {
                        let items = focusable_elements(&panel);
                        if let Some(first) = items.first() {
                            let _ = first.focus();
                        }
                    }
                } else if let Some(el) = document.get_element_by_id("mobile-menu-trigger")
                    && let Ok(btn) = el.dyn_into::<web_sys::HtmlElement>()
                {
                    let _ = btn.focus();
                }
            }
        }
        open
    });

    view! {
        // Hamburger button (mobile only)
        <button
            id="mobile-menu-trigger"
            type="button"
            on:click=move |e| {
                e.stop_propagation();
                set_is_open.update(|v| *v = !*v);
            }
            class="md:hidden p-2 text-comb hover:text-bark transition-colors"
            aria-label="Open menu"
            aria-haspopup="dialog"
            aria-controls="mobile-menu-panel"
            aria-expanded=move || if is_open.get() { "true" } else { "false" }
        >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
            </svg>
        </button>

        // Full-screen overlay + slide-out panel
        <Show when=move || is_open.get()>
            <div
                class="fixed inset-0 z-[100] bg-bark/60 backdrop-blur-xs md:hidden"
                on:click=move |e| {
                    // Only close when the backdrop itself was clicked, not children.
                    if let Some(target) = e.target()
                        && let Some(current) = e.current_target()
                        && target.unchecked_ref::<web_sys::Node>()
                            .is_same_node(Some(current.unchecked_ref::<web_sys::Node>()))
                    {
                        set_is_open.set(false);
                    }
                }
            >
                // Menu panel (slides in from right)
                <div
                    id="mobile-menu-panel"
                    role="dialog"
                    aria-modal="true"
                    aria-label="Navigation menu"
                    tabindex="-1"
                    class="absolute right-0 top-0 h-full w-72 bg-surface border-l border-honey/10 shadow-2xl p-6 flex flex-col gap-6 animate-slide-in-right"
                >
                    // Close button
                    <div class="flex justify-end">
                        <button
                            type="button"
                            on:click=move |_| set_is_open.set(false)
                            class="p-2 text-comb hover:text-bark transition-colors"
                            aria-label="Close menu"
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                            </svg>
                        </button>
                    </div>

                    // Navigation links
                    <nav class="flex flex-col gap-4">
                        <a
                            href="/"
                            class="text-lg font-medium text-bark hover:text-honey transition-colors"
                            on:click=move |_| set_is_open.set(false)
                        >
                            "Home"
                        </a>
                        <a
                            href="/docs/getting-started"
                            class="text-lg font-medium text-bark hover:text-honey transition-colors"
                            on:click=move |_| set_is_open.set(false)
                        >
                            "Docs"
                        </a>
                        <a
                            href="https://github.com/sephyi/commitbee"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-lg font-medium text-comb hover:text-honey transition-colors"
                            on:click=move |_| set_is_open.set(false)
                        >
                            "GitHub"
                        </a>
                        <a
                            href="https://crates.io/crates/commitbee"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-lg font-medium text-comb hover:text-honey transition-colors"
                            on:click=move |_| set_is_open.set(false)
                        >
                            "crates.io"
                        </a>
                    </nav>
                </div>
            </div>
        </Show>
    }
}
