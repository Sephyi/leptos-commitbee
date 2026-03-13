use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 w-full border-b border-honey/10 bg-surface/80 backdrop-blur-lg transition-colors">
            <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
                // Logo
                <a href="/" class="flex items-center gap-2 font-bold text-lg text-bark hover:text-honey transition-colors">
                    <span class="text-2xl">"🐝"</span>
                    <span>"CommitBee"</span>
                </a>

                // Desktop nav links
                <div class="hidden md:flex items-center gap-6">
                    <a href="/" class="text-sm font-medium text-comb hover:text-bark transition-colors">"Home"</a>
                    <a href="/docs/getting-started" class="text-sm font-medium text-comb hover:text-bark transition-colors">"Docs"</a>
                    <a
                        href="https://github.com/sephyi/commitbee"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm font-medium text-comb hover:text-bark transition-colors"
                    >
                        "GitHub"
                    </a>
                    <a
                        href="https://crates.io/crates/commitbee"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="text-sm font-medium text-comb hover:text-bark transition-colors"
                    >
                        "crates.io"
                    </a>
                </div>

                // Right side: search + theme toggle + mobile menu
                <div class="flex items-center gap-3">
                    <super::doc_search::DocSearch/>
                    <super::theme_toggle::ThemeToggle/>

                    // Mobile hamburger
                    <button
                        class="md:hidden p-2 text-comb hover:text-bark"
                        aria-label="Toggle menu"
                        id="mobile-menu-toggle"
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                        </svg>
                    </button>
                </div>
            </div>
        </nav>
    }
}
