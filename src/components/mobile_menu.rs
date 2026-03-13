// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[island]
pub fn MobileMenu() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    // Close on Escape key
    Effect::new(move || {
        let closure =
            Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Escape" {
                    set_is_open.set(false);
                }
            });
        let window = web_sys::window().unwrap();
        let _ =
            window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();
    });

    view! {
        // Hamburger button (mobile only)
        <button
            on:click=move |_| set_is_open.set(true)
            class="md:hidden p-2 text-comb hover:text-bark transition-colors"
            aria-label="Open menu"
        >
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
            </svg>
        </button>

        // Full-screen overlay + slide-out panel
        <Show when=move || is_open.get()>
            <div
                class="fixed inset-0 z-[100] bg-bark/60 backdrop-blur-xs"
                on:click=move |_| set_is_open.set(false)
            >
                // Menu panel (slides in from right)
                <div
                    class="absolute right-0 top-0 h-full w-72 bg-surface border-l border-honey/10 shadow-2xl p-6 flex flex-col gap-6 animate-slide-in-right"
                    on:click=move |e| e.stop_propagation()
                >
                    // Close button
                    <div class="flex justify-end">
                        <button
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
