// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav id="site-nav" class="sticky top-0 z-50 w-full border-b border-transparent bg-transparent transition-all duration-300 scrolled:border-honey/10 scrolled:bg-surface/80 scrolled:backdrop-blur-lg">
            <div class="relative flex h-16 w-full items-center justify-between px-4">
                // Left: Logo
                <a href="/" class="relative z-10 flex items-center gap-2 font-bold text-lg text-bark hover:text-honey transition-colors">
                    <span class="text-2xl">"🐝"</span>
                    <span>"CommitBee"</span>
                </a>

                // Center: Search (desktop) — absolutely centered, independent of side widths
                <div class="pointer-events-none absolute inset-x-0 top-1/2 hidden -translate-y-1/2 justify-center md:flex">
                    <div class="pointer-events-auto w-[26rem]">
                        <super::doc_search::DocSearch/>
                    </div>
                </div>

                // Right: Desktop nav links + theme toggle, or mobile cluster.
                // -mr-2 cancels the trailing icon button's internal p-2 so its
                // visible right edge aligns exactly with the px-4 nav boundary
                // (which in turn matches the TOC content right edge).
                <div class="relative z-10 flex items-center -mr-2">
                    <div class="hidden md:flex items-center gap-5">
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
                    <super::theme_toggle::ThemeToggle/>

                    // Mobile-only: search icon + hamburger
                    <super::doc_search::MobileSearchButton/>
                    <super::mobile_menu::MobileMenu/>
                </div>
            </div>
        </nav>
    }
}
