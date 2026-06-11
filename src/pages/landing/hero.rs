// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="relative flex items-center justify-center min-h-screen overflow-hidden">

            <div class="relative max-w-4xl px-4 mx-auto text-center">
                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <h1 class="text-4xl font-bold tracking-tight text-bark sm:text-6xl lg:text-7xl">
                        "The commit message generator that "
                        <span class="text-honey">"actually understands"</span>
                        " your code."
                    </h1>
                </crate::components::scroll_reveal::ScrollReveal>

                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <p class="max-w-2xl mx-auto mt-6 text-lg text-comb sm:text-xl">
                        "CommitBee parses your code with tree-sitter, maps diff hunks to symbol spans, and gives the LLM structured semantic context."
                    </p>
                </crate::components::scroll_reveal::ScrollReveal>

                <crate::components::scroll_reveal::ScrollReveal class="reveal">
                    <div class="flex items-center justify-center gap-4 mt-10">
                        <a
                            href="/docs/getting-started"
                            class="px-6 py-3 text-sm font-semibold text-white transition-colors rounded-lg shadow-lg bg-honey shadow-honey/25 hover:bg-honey-dark"
                        >
                            "Get Started"
                        </a>
                        <a
                            href="#pipeline"
                            class="px-6 py-3 text-sm font-semibold transition-colors border rounded-lg border-honey/30 text-bark hover:bg-honey/5"
                        >
                            "See How It Works"
                        </a>
                    </div>
                </crate::components::scroll_reveal::ScrollReveal>

                // Terminal mockup
                <crate::components::scroll_reveal::ScrollReveal class="reveal-scale">
                    <div class="max-w-2xl mx-auto mt-16 overflow-hidden border shadow-2xl rounded-xl border-honey/20 bg-surface-raised shadow-honey/5">
                        <div class="flex items-center gap-2 px-4 py-3 border-b bg-pollen/50 border-honey/10">
                            <div class="w-3 h-3 rounded-full bg-red-400/60"/>
                            <div class="w-3 h-3 rounded-full bg-yellow-400/60"/>
                            <div class="w-3 h-3 rounded-full bg-green-400/60"/>
                            <span class="ml-2 text-xs text-comb">"~/project"</span>
                        </div>
                        <div class="p-4 font-mono text-sm text-left text-bark">
                            <div class="text-comb">"$ commitbee"</div>
                            <div class="mt-2 text-comb">"Analyzing 3 staged files..."</div>
                            <div class="text-comb">"Extracting symbols (tree-sitter)..."</div>
                            <div class="mt-2">"feat(auth): add JWT token refresh with configurable expiry"</div>
                            <div class="mt-1 text-comb">"Commit? (Y/n) " <span class="terminal-cursor">" "</span></div>
                        </div>
                    </div>
                </crate::components::scroll_reveal::ScrollReveal>
            </div>
        </section>
    }
}
