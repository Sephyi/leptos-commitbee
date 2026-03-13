use leptos::prelude::*;

#[island]
pub fn ThemeToggle() -> impl IntoView {
    view! {
        <button class="p-2 text-comb hover:text-honey" aria-label="Toggle theme">"🌓"</button>
    }
}
