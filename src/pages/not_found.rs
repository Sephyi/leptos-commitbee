use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="flex min-h-[60vh] items-center justify-center">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-honey">"404"</h1>
                <p class="mt-4 text-lg text-comb">"Page not found"</p>
                <a href="/" class="mt-6 inline-block text-sm text-honey hover:underline">"Back to home"</a>
            </div>
        </div>
    }
}
