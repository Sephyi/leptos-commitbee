use leptos::prelude::*;

#[island]
pub fn CodeCopyButton(code: String) -> impl IntoView {
    let _ = code;
    view! {
        <button class="copy-btn text-comb hover:text-honey text-xs">"Copy"</button>
    }
}
