// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav id="site-nav" class="sticky top-0 z-50 w-full transition-all duration-300 bg-transparent border-b border-transparent scrolled:border-honey/10 scrolled:bg-surface/80 scrolled:backdrop-blur-lg">
            <div class="relative flex items-center justify-between w-full h-16 px-4">
                // Left: Logo
                <a href="/" class="relative z-10 flex items-center gap-2 text-lg font-bold transition-colors text-bark hover:text-honey">
                    <span class="text-2xl">"🐝"</span>
                    <span>"CommitBee"</span>
                </a>

                // Center: Search (desktop) — absolutely centered, independent of side widths
                <div class="absolute inset-x-0 justify-center hidden -translate-y-1/2 pointer-events-none top-1/2 md:flex">
                    <div class="pointer-events-auto w-[26rem]">
                        <super::doc_search::DocSearch/>
                    </div>
                </div>

                // Right: Desktop nav links + theme toggle, or mobile cluster.
                // -mr-2 cancels the trailing icon button's internal p-2 so its
                // visible right edge aligns exactly with the px-4 nav boundary
                // (which in turn matches the TOC content right edge).
                // TODO: Temporarily removed the theme toggle
                // <div class="relative z-10 flex items-center -mr-2">
                <div class="relative z-10 flex items-center">
                    <div class="items-center hidden gap-5 md:flex">
                        <a href="/" class="text-sm font-medium transition-colors text-comb hover:text-bark">"Home"</a>
                        <a href="/docs/getting-started" class="text-sm font-medium transition-colors text-comb hover:text-bark">"Docs"</a>
                        <a
                            href="https://github.com/sephyi/commitbee"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-sm font-medium transition-colors text-comb hover:text-bark"
                        >
                            "GitHub"
                        </a>
                        <a
                            href="https://crates.io/crates/commitbee"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-sm font-medium transition-colors text-comb hover:text-bark"
                        >
                            "crates.io"
                        </a>
                    </div>
                    // TODO: <super::theme_toggle::ThemeToggle/>

                    // Mobile-only: search icon + hamburger
                    <super::doc_search::MobileSearchButton/>
                    <super::mobile_menu::MobileMenu/>
                </div>
            </div>
        </nav>
    }
}
