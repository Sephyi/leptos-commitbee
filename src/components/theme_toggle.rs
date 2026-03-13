use leptos::prelude::*;
use leptos::web_sys;

#[island]
pub fn ThemeToggle() -> impl IntoView {
    let (is_dark, set_is_dark) = signal(false);

    // Initialize from localStorage or system preference
    Effect::new(move || {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        let preference = storage.get_item("theme").unwrap_or(None);
        let dark = match preference.as_deref() {
            Some("dark") => true,
            Some("light") => false,
            _ => window
                .match_media("(prefers-color-scheme: dark)")
                .ok()
                .flatten()
                .map(|mql: web_sys::MediaQueryList| mql.matches())
                .unwrap_or(false),
        };

        set_is_dark.set(dark);
        apply_theme(dark);
    });

    let toggle = move |_| {
        let new_dark = !is_dark.get();
        set_is_dark.set(new_dark);
        apply_theme(new_dark);

        if let Some(storage) = web_sys::window().unwrap().local_storage().unwrap() {
            let _ = storage.set_item("theme", if new_dark { "dark" } else { "light" });
        }
    };

    view! {
        <button
            on:click=toggle
            class="p-2 rounded-lg text-comb hover:text-bark hover:bg-surface-raised transition-colors"
            aria-label="Toggle dark mode"
        >
            {move || if is_dark.get() {
                view! {
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"/>
                    </svg>
                }.into_any()
            } else {
                view! {
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/>
                    </svg>
                }.into_any()
            }}
        </button>
    }
}

fn apply_theme(dark: bool) {
    if let Some(document) = web_sys::window().and_then(|w: web_sys::Window| w.document()) {
        if let Some(html) = document.document_element() {
            let class_list = html.class_list();
            if dark {
                let _ = class_list.add_1("dark");
            } else {
                let _ = class_list.remove_1("dark");
            }
        }
    }
}
