// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        // The compact strip is the footer. The giant "commitbee" wordmark is
        // an `absolute bottom-full` child so it's pulled out of layout flow
        // entirely — the footer starts at its natural position, and the
        // wordmark visually sits inside the bottom of whichever content
        // section is directly above the footer.
        <footer class="relative border-t border-honey/10 bg-surface/55 backdrop-blur-md">
            <h2 class="absolute bottom-full inset-x-0 pointer-events-none select-none text-center font-black text-honey/15 leading-[0.85] tracking-tight text-[clamp(4rem,18vw,14rem)] overflow-hidden">
                "commitbee"
            </h2>
            <div class="w-full px-4 py-4">
                <div class="flex flex-wrap items-center justify-between gap-x-4 gap-y-2 text-xs text-comb">
                    <span class="flex items-center gap-1.5">
                        <span class="text-base">"🐝"</span>
                        "© 2026 "
                        <a href="https://sephy.io" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sephyi"</a>
                    </span>
                    <nav class="flex flex-wrap items-center gap-x-4 gap-y-1">
                        <a href="/docs/getting-started" class="hover:text-honey transition-colors">"Docs"</a>
                        <a href="https://github.com/sephyi/commitbee" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"GitHub"</a>
                        <a href="https://github.com/sponsors/Sephyi" target="_blank" rel="noopener noreferrer" class="hover:text-honey transition-colors">"Sponsor"</a>
                        <a href="/imprint" class="hover:text-honey transition-colors">"Imprint"</a>
                        <a href="/privacy" class="hover:text-honey transition-colors">"Privacy"</a>
                    </nav>
                </div>
            </div>
        </footer>
    }
}
